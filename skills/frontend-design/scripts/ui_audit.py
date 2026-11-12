#!/usr/bin/env python3
"""Static checks for UI source files: accessibility basics, IME-safe Enter handling, focus
styles, zoom blocking, spacing off the grid, colours written outside the tokens, styles repeated
in several places, and the main accessibility gaps in Flutter, SwiftUI and Compose code. It also
reports what the project already has: platform, framework, token files and component folders.

The Engine calls it with a JSON file of arguments and runs it in the open folder. It only reads.
Findings are hints with a file and line; the model confirms each one in the code.
"""

import json
import os
import re
import sys
from collections import defaultdict
from pathlib import Path

MARKUP = {".html", ".htm", ".jsx", ".tsx", ".vue", ".svelte", ".astro"}
STYLES = {".css", ".scss", ".less"}
NATIVE = {".dart", ".swift", ".kt"}
# A file that defines the design values: colours written here are the tokens themselves
TOKEN_FILE = re.compile(r"(?i)(^|[/._-])(tokens?|theme|themes|colou?rs?|palette|variables|vars|design-system|tailwind\.config)[/._-]|tailwind\.config\.|(^|/)_?variables\.s?css$")
COLOR_LITERAL = re.compile(r"(?<![\w&])#(?:[0-9a-fA-F]{3,4}|[0-9a-fA-F]{6}|[0-9a-fA-F]{8})\b(?![-\w])|\b(?:rgba?|hsla?|oklch)\(\s*[\d.]+")
TAILWIND_ARBITRARY_COLOR = re.compile(r"\b(?:bg|text|border|from|to|via|fill|stroke|ring)-\[(?:#|rgb|hsl|oklch)")
CSS_RULE = re.compile(r"([^{}@;]+)\{([^{}]+)\}")
SKIP = {"node_modules", ".git", ".next", "dist", "build", "out", ".svelte-kit", ".nuxt", "coverage", "target", ".dart_tool", "Pods", "DerivedData", ".gradle"}
MAX_FILES = 400

TAG_IMG = re.compile(r"<img\b[^>]*?>", re.S)  # lower case only: <Img> is a component
TAG_BUTTON = re.compile(r"<button\b([^>]*)>(.*?)</button>", re.S)
CLICK_DIV = re.compile(r"<(div|span)\b([^>]*\bon(Click|click)\s*=[^>]*)>", re.S)
ENTER = re.compile(r"""\b(e|ev|evt|event)\.(key|code)\s*===?\s*['"]Enter['"]""")
VIEWPORT = re.compile(r"""<meta[^>]+name=["']viewport["'][^>]*>""", re.I)
OUTLINE_NONE = re.compile(r"outline\s*:\s*(none|0)\b", re.I)
POSITIVE_TABINDEX = re.compile(r"""tab[iI]ndex\s*=\s*\{?\s*["']?([1-9]\d*)""")
OFF_GRID = re.compile(r"\b(?:p|px|py|pt|pb|pl|pr|m|mx|my|mt|mb|ml|mr|gap|gap-x|gap-y|space-x|space-y)-\[(\d+)px\]")
AUTOPLAY = re.compile(r"<video\b[^>]*\bautoplay\b[^>]*>", re.S | re.I)
INPUT = re.compile(r"<input\b([^>]*)>", re.S)  # <Input> may be a TypeScript type or a component


def finish(answer):
    print(json.dumps(answer, ensure_ascii=False, indent=1))
    sys.exit(0)


def line_of(text, offset):
    return text.count("\n", 0, offset) + 1


def audit(path, text, rel):
    found = []

    def add(offset, rule, severity, message):
        found.append({"at": f"{rel}:{line_of(text, offset)}", "rule": rule, "severity": severity, "message": message})

    suffix = path.suffix.lower()
    if suffix in MARKUP:
        for m in TAG_IMG.finditer(text):
            if not re.search(r"\balt\s*=", m.group(0)):
                add(m.start(), "img-alt", "error", "Image without alt: describe it, or alt=\"\" when it is decorative.")
        for m in TAG_BUTTON.finditer(text):
            attrs, inner = m.group(1), m.group(2)
            visible = re.sub(r"<[^>]+>", "", inner)
            visible = re.sub(r"\{[^}]*\}", "x", visible).strip()  # JSX expressions may be text
            has_icon = re.search(r"<(svg|i|img|[A-Z]\w*Icon\b|Icon\b)", inner)
            named = re.search(r"aria-label(ledby)?\s*=|title\s*=", attrs) or re.search(r"\bsr-only\b|visually-hidden", inner)
            if has_icon and not visible and not named:
                add(m.start(), "button-name", "error", "Icon-only button without an accessible name: add aria-label.")
        for m in CLICK_DIV.finditer(text):
            attrs = m.group(2)
            if "stopPropagation" in attrs:
                continue
            if not re.search(r"\brole\s*=", attrs) or not re.search(r"tab[iI]ndex", attrs):
                add(m.start(), "clickable-div", "warning", f"<{m.group(1)}> with a click handler but no role and tabIndex: keyboard users cannot reach it. Use a <button>.")
        for m in INPUT.finditer(text):
            attrs = m.group(1)
            if re.search(r"""type\s*=\s*["'](hidden|submit|button|checkbox|radio)["']""", attrs):
                continue
            if not re.search(r"aria-label(ledby)?\s*=|\bid\s*=|title\s*=", attrs):
                add(m.start(), "input-label", "warning", "Input with no id (for a <label for>) and no aria-label: screen readers cannot name it.")
        for m in VIEWPORT.finditer(text):
            if re.search(r"user-scalable\s*=\s*(no|0)|maximum-scale\s*=\s*1(\.0)?\b", m.group(0), re.I):
                add(m.start(), "zoom-blocked", "error", "The viewport blocks zoom (user-scalable=no or maximum-scale=1); people who need larger text cannot zoom.")
        for m in AUTOPLAY.finditer(text):
            if not re.search(r"\bmuted\b", m.group(0), re.I):
                add(m.start(), "autoplay-sound", "warning", "Autoplaying video without muted: browsers block it, and sound that starts by itself is an accessibility problem.")
        for m in POSITIVE_TABINDEX.finditer(text):
            add(m.start(), "tabindex-positive", "warning", f"tabIndex={m.group(1)} changes the tab order; use 0 or -1.")
        for m in OFF_GRID.finditer(text):
            if int(m.group(1)) % 4:
                add(m.start(), "off-grid-spacing", "info", f"Spacing {m.group(1)}px is not on the 4px grid.")
    if suffix in MARKUP or suffix in {".js", ".ts", ".mjs"}:
        for m in ENTER.finditer(text):
            near = text[max(0, m.start() - 300): m.end() + 300]
            text_entry = re.search(r"(?i)textarea|<input|contenteditable|onSubmit|submit\(|send\(|sendMessage|search|prompt|message", near)
            if text_entry and "isComposing" not in near and "229" not in near:
                add(m.start(), "ime-enter", "warning", "Enter handled without checking event.isComposing (or keyCode 229): it submits while a Chinese, Japanese or Korean IME is still composing.")
    if suffix in STYLES or suffix in {".vue", ".svelte", ".html", ".htm"}:
        for m in OUTLINE_NONE.finditer(text):
            start = text.rfind("}", 0, m.start()) + 1
            end = text.find("}", m.end())
            block = text[start: end if end != -1 else len(text)]
            if re.search(r"focus-visible|box-shadow|\bring\b|border-color", block):
                continue
            severity = "warning" if ":focus-visible" in text else "error"
            add(m.start(), "focus-removed", severity, "outline: none in a rule without another focus style: keyboard users cannot see where they are. Give :focus-visible a visible style.")
    if suffix == ".dart":
        for m in re.finditer(r"\bIconButton(\.\w+)?\s*\(", text):
            if not re.search(r"\btooltip\s*:", call_body(text, m.end())):
                add(m.start(), "button-name", "error", "IconButton without tooltip: screen readers have no name for it. Add tooltip: (it is also its semantics label).")
        for m in re.finditer(r"\bImage\.(asset|network|file|memory)\s*\(", text):
            body = call_body(text, m.end())
            if not re.search(r"\bsemanticLabel\s*:|excludeFromSemantics\s*:", body):
                add(m.start(), "img-alt", "warning", "Image without semanticLabel: describe it, or set excludeFromSemantics: true when it is decorative.")
    if suffix == ".swift":
        for m in re.finditer(r"\.onTapGesture\s*(\(|\{)", text):
            add(m.start(), "clickable-view", "warning", ".onTapGesture makes a view tappable without button traits: VoiceOver and the keyboard do not treat it as a button. Use Button.")
    if suffix == ".kt":
        for m in re.finditer(r"\bIconButton\s*\(", text):
            body = text[m.end(): m.end() + 500]
            if re.search(r"\bIcon\s*\([^)]*contentDescription\s*=\s*null", body.split("IconButton(")[0]):
                add(m.start(), "button-name", "error", "IconButton whose Icon has contentDescription = null: the button has no name. Give the Icon a contentDescription.")
    if not TOKEN_FILE.search(rel) and (suffix in MARKUP | STYLES | NATIVE or suffix in {".js", ".ts", ".mjs"}):
        # Colours written into components are a second source next to the tokens
        hits = [m for m in COLOR_LITERAL.finditer(text) if not text[max(0, m.start() - 2): m.start()].endswith("&")]
        hits += list(TAILWIND_ARBITRARY_COLOR.finditer(text))
        if suffix == ".dart":
            hits += list(re.finditer(r"\bColor\(0x[0-9a-fA-F]{8}\)", text))
        if hits:
            first = min(h.start() for h in hits)
            add(first, "hardcoded-color", "info", f"{len(hits)} colour value(s) written in this file: use the project's tokens or theme, so a colour changes in one place.")
    return found


def call_body(text, start, limit=600):
    """The text of a call from its opening parenthesis to the matching one (or `limit` chars)"""
    depth, end = 1, min(len(text), start + limit)
    for i in range(start, end):
        if text[i] == "(":
            depth += 1
        elif text[i] == ")":
            depth -= 1
            if depth == 0:
                return text[start:i]
    return text[start:end]


def repeated_styles(texts):
    """CSS rule blocks with the same declarations under different selectors: one class or token
    would do. `texts` holds (relative path, text) of style files."""
    seen = defaultdict(list)
    for rel, text in texts:
        for m in CSS_RULE.finditer(re.sub(r"/\*.*?\*/", "", text, flags=re.S)):
            selector = " ".join(m.group(1).split())
            declarations = sorted({" ".join(d.split()).lower() for d in m.group(2).split(";") if ":" in d})
            if len(declarations) >= 3 and selector and not selector.startswith(("from", "to", "@")):
                seen[tuple(declarations)].append((rel, line_of(text, m.start()), selector))
    found = []
    for declarations, places in seen.items():
        selectors = {p[2] for p in places}
        if len(places) >= 2 and len(selectors) >= 2:
            rel, line, _ = places[0]
            others = ", ".join(f"{p[2]} ({p[0]}:{p[1]})" for p in places[1:4])
            found.append({"at": f"{rel}:{line}", "rule": "repeated-style", "severity": "warning",
                          "message": f"The same {len(declarations)} declarations are also in {others}: share one class, component or token."})
    return found


def project_inventory(root, files):
    """What the project already has, so new UI reuses it: platform, framework, tokens, components"""
    have = {"platform": [], "framework": [], "token_files": [], "component_folders": []}
    pkg = root / "package.json"
    deps = {}
    if pkg.is_file():
        try:
            data = json.loads(pkg.read_text(encoding="utf-8"))
            deps = {**data.get("dependencies", {}), **data.get("devDependencies", {})}
        except ValueError:
            pass
    for name, label, platform in [
        ("react-native", "React Native", "mobile"), ("expo", "Expo", "mobile"), ("electron", "Electron", "desktop"),
        ("@tauri-apps/api", "Tauri", "desktop"), ("next", "Next.js", "web"), ("react", "React", "web"), ("vue", "Vue", "web"),
        ("svelte", "Svelte", "web"), ("@angular/core", "Angular", "web"), ("astro", "Astro", "web"), ("tailwindcss", "Tailwind CSS", None),
        ("@mui/material", "MUI", None), ("@chakra-ui/react", "Chakra UI", None), ("styled-components", "styled-components", None),
        ("@radix-ui/react-slot", "Radix / shadcn", None), ("bootstrap", "Bootstrap", None)]:
        if name in deps:
            have["framework"].append(label)
            if platform and platform not in have["platform"]:
                have["platform"].append(platform)
    if (root / "pubspec.yaml").is_file():
        have["framework"].append("Flutter"); have["platform"].append("mobile")
    if any(root.glob("*.xcodeproj")) or (root / "Package.swift").is_file():
        have["framework"].append("SwiftUI or UIKit"); have["platform"].append("mobile or macOS")
    if any((root / f).is_file() for f in ("build.gradle.kts", "build.gradle", "app/build.gradle.kts")):
        have["framework"].append("Android (Compose or Views)"); have["platform"].append("mobile")
    if (root / "components.json").is_file():
        have["framework"].append("shadcn/ui")
    for f in files:
        try:
            rel = f.resolve().relative_to(root.resolve()).as_posix()
        except ValueError:
            rel = f.name
        if TOKEN_FILE.search(rel) and len(have["token_files"]) < 10:
            have["token_files"].append(rel)
    for folder, dirs, _ in os.walk(root):
        dirs[:] = sorted(d for d in dirs if d not in SKIP and not d.startswith("."))
        for d in dirs:
            if d in {"components", "ui", "widgets", "design-system"} and len(have["component_folders"]) < 10:
                have["component_folders"].append((Path(folder) / d).relative_to(root).as_posix())
    have["framework"] = sorted(set(have["framework"]))
    return have


def main():
    args = {}
    if len(sys.argv) > 1 and Path(sys.argv[1]).is_file():
        try:
            args = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8")) or {}
        except ValueError:
            args = {}
    target = Path(str(args.get("path") or ".")).expanduser()
    if not target.exists():
        finish({"status": "error", "message": f"No file or folder at {target}."})
    base = target if target.is_dir() else target.parent
    if target.is_file():
        files = [target]
    else:
        files = []
        for folder, dirs, names in os.walk(target):
            dirs[:] = sorted(d for d in dirs if d not in SKIP)
            for name in sorted(names):
                p = Path(folder) / name
                if p.suffix.lower() in MARKUP | STYLES | NATIVE | {".js", ".ts", ".mjs"} and len(files) < MAX_FILES:
                    files.append(p)
    findings, styles = [], []
    for f in files:
        try:
            text = f.read_text(encoding="utf-8", errors="replace")
        except OSError:
            continue
        rel = f.relative_to(base).as_posix() if f != base else f.name
        findings.extend(audit(f, text, rel))
        if f.suffix.lower() in STYLES:
            styles.append((rel, text))
    findings.extend(repeated_styles(styles))
    order = {"error": 0, "warning": 1, "info": 2}
    findings.sort(key=lambda x: order[x["severity"]])
    counts = {k: sum(1 for x in findings if x["severity"] == k) for k in order}
    # The project's root: the folder given, or the nearest one above it with a project file
    root = base
    for folder in [base, *base.parents]:
        if any((folder / m).exists() for m in ("package.json", "pubspec.yaml", "Package.swift", "build.gradle.kts", "build.gradle", ".git")):
            root = folder
            break
    finish({
        "status": "ok",
        "files_checked": len(files),
        "project": project_inventory(root, files),
        "counts": counts,
        "findings": findings[:60],
        "note": ("Only the first 60 findings are shown. " if len(findings) > 60 else "")
        + "These are pattern checks. Confirm each in the code, and check what patterns cannot: contrast, layout on a small screen, real keyboard and screen reader use.",
    })


if __name__ == "__main__":
    main()
