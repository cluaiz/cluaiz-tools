#!/usr/bin/env python3
"""Collects what a code review needs: the changes, how big they are, which files carry the most
risk, the project's own context files and check commands, and risky added lines.

The Engine calls it with a JSON file of arguments and runs it in the open folder. It only reads.
Hints and risk tags are where to look first, not findings: the model still reads the code.
"""

import json
import re
import subprocess
import sys
from pathlib import Path

RULES = [
    # (name, pattern, file suffixes or None for all, skip in test files)
    ("debug leftover", re.compile(r"\bdbg!\s*\(|\bconsole\.(log|debug)\s*\(|\bdebugger\b|\bbreakpoint\(\)|\bpdb\.set_trace\(|\bfmt\.Print(ln|f)?\(|\bSystem\.out\.print|\bprintStackTrace\(\)"), None, True),
    ("unwrap or expect that can panic", re.compile(r"\.(unwrap|expect)\s*\("), (".rs",), True),
    ("panic or todo macro", re.compile(r"\b(todo|unimplemented|panic)!\s*\("), (".rs",), True),
    ("force unwrap that can crash", re.compile(r"\btry!|!!(\.|\s|$)|as!\s"), (".swift", ".kt", ".kts"), True),
    ("panic in library code", re.compile(r"\bpanic\("), (".go",), True),
    ("Go error ignored", re.compile(r"^\s*_\s*(,\s*_)?\s*=\s*\w+[\w.]*\(|,\s*_\s*:?=\s*\w+[\w.]*\("), (".go",), True),
    ("unsafe C string or memory call", re.compile(r"\b(strcpy|strcat|sprintf|vsprintf|gets|scanf)\s*\(|\balloca\s*\("), (".c", ".h", ".cc", ".cpp", ".cxx", ".hpp", ".hh", ".m", ".mm"), False),
    ("raw new or delete", re.compile(r"\bnew\s+\w+(\[|\(|;)|\bdelete(\[\])?\s+\w"), (".cc", ".cpp", ".cxx", ".hpp", ".hh"), True),
    ("shell built from a string", re.compile(r"shell\s*=\s*True|\bos\.system\s*\(|\bos\.popen\s*\(|\bexecSync\s*\(\s*`|\bexec\s*\(\s*`|\bsh\s+-c\b|Command::new\(\s*\"(sh|bash|cmd|powershell)\"|Runtime\.getRuntime\(\)\.exec\(|exec\.Command\(\s*\"(sh|bash|cmd)\"|\bsystem\s*\(\s*[a-zA-Z_]|\bpopen\s*\(\s*[a-zA-Z_]"), None, False),
    ("eval of dynamic code", re.compile(r"\beval\s*\(|\bnew Function\s*\(|\bexec\s*\(\s*[a-zA-Z_]"), (".js", ".ts", ".jsx", ".tsx", ".py", ".mjs", ".cjs", ".php", ".rb"), False),
    ("unsafe deserialization", re.compile(r"\bpickle\.loads?\(|\byaml\.load\((?![^)]*SafeLoader)|\bunserialize\(|\bObjectInputStream\(|\bBinaryFormatter\b|\bMarshal\.load\(|\bjsonpickle\.decode\("), None, False),
    ("raw HTML from data", re.compile(r"dangerouslySetInnerHTML|\.innerHTML\s*=|\.outerHTML\s*=|v-html=|\|\s*safe\b|\{\{\{|bypassSecurityTrust|Html\.Raw\("), None, False),
    ("SQL built with string formatting", re.compile(r"(?i)\b(select|insert|update|delete)\b[^\n]{0,80}(\+\s*[a-z_]|\$\{|%s|\{\}|format\(|f\"|\"\s*\.\s*\$)"), None, False),
    ("weak hash or random for security", re.compile(r"(?i)\b(md5|sha1)\s*\(|hashlib\.(md5|sha1)\(|Math\.random\(\)[^\n]{0,40}(token|secret|password|key|id)|\brandom\.(random|randint|choice)\([^\n]{0,40}(token|secret|password)"), None, True),
    ("hardcoded local address", re.compile(r"\b(localhost|127\.0\.0\.1|0\.0\.0\.0):\d{2,5}\b"), None, True),
    ("hardcoded user path", re.compile(r"(?i)\b[A-Z]:\\\\?Users\\\\?|/home/[a-z][a-z0-9_-]+/|/Users/[A-Za-z][A-Za-z0-9_-]+/"), None, False),
    ("error swallowed", re.compile(r"except\s*:\s*(pass)?$|except Exception\s*:\s*pass|catch\s*\(\s*[\w.]*\s*\w*\s*\)\s*\{\s*\}|catch\s*\{\s*\}|let _ = .*\?;|\.ok\(\);|catch\s*\(\s*\.\.\.\s*\)\s*\{\s*\}"), None, False),
    ("TLS check turned off", re.compile(r"verify\s*=\s*False|rejectUnauthorized\s*:\s*false|danger_accept_invalid_certs\(true\)|InsecureSkipVerify:\s*true|CURLOPT_SSL_VERIFYPEER,\s*(0|false)|ServerCertificateValidationCallback"), None, False),
    ("possible secret", re.compile(r"-----BEGIN [A-Z ]*PRIVATE KEY-----|\bAKIA[0-9A-Z]{16}\b|\b(ghp|gho|ghs)_[A-Za-z0-9]{36}\b|\bgithub_pat_\w{60,}|\bsk-(ant-|proj-)?[A-Za-z0-9_-]{32,}\b|\bgsk_[A-Za-z0-9]{40,}\b|\bAIza[0-9A-Za-z_-]{35}\b|\bhf_[A-Za-z0-9]{30,}\b|\bxox[abprs]-[A-Za-z0-9-]{10,}"), None, False),
    ("conflict marker", re.compile(r"^(<{7}|={7}|>{7})( |$)"), None, False),
    ("new TODO", re.compile(r"\b(TODO|FIXME|XXX|HACK)\b"), None, False),
]
TEST_FILE = re.compile(r"(^|/)(tests?|__tests__|spec|e2e|testdata)/|[._-](test|spec)\.[a-z]+$|_test\.(go|py|rs)$|(^|/)test_[^/]+\.py$|Tests?\.(cs|java|kt|swift)$")
SECRET_VALUE = re.compile(r"[A-Za-z0-9_\-]{16,}")

LANGUAGES = {
    ".py": "Python", ".rs": "Rust", ".go": "Go", ".js": "JavaScript", ".mjs": "JavaScript", ".cjs": "JavaScript",
    ".jsx": "JavaScript", ".ts": "TypeScript", ".tsx": "TypeScript", ".vue": "Vue", ".svelte": "Svelte",
    ".c": "C", ".h": "C/C++", ".cc": "C++", ".cpp": "C++", ".cxx": "C++", ".hpp": "C++", ".hh": "C++",
    ".java": "Java", ".kt": "Kotlin", ".kts": "Kotlin", ".swift": "Swift", ".m": "Objective-C", ".mm": "Objective-C",
    ".cs": "C#", ".php": "PHP", ".rb": "Ruby", ".dart": "Dart", ".scala": "Scala", ".sql": "SQL",
    ".sh": "Shell", ".bash": "Shell", ".ps1": "PowerShell", ".html": "HTML", ".css": "CSS", ".scss": "CSS",
}
# Paths whose changes usually carry more risk; the first match names why
RISK_PATHS = [
    ("auth or permissions", re.compile(r"(?i)auth|login|session|token|permission|acl|rbac|oauth|password|credential")),
    ("security or crypto", re.compile(r"(?i)secur|crypt|sign|cert|secret|sanitiz|escape")),
    ("money or billing", re.compile(r"(?i)pay|billing|invoice|price|charge|wallet|ledger")),
    ("data schema or migration", re.compile(r"(?i)migrat|schema|\.sql$|models?/|entities/|prisma")),
    ("public API", re.compile(r"(?i)(^|/)(api|routes?|handlers?|controllers?|endpoints?|proto)/|openapi|\.proto$|graphql")),
    ("concurrency", re.compile(r"(?i)thread|async|worker|queue|lock|pool|scheduler|concurren")),
    ("build, CI or deployment", re.compile(r"(?i)(^|/)\.github/|dockerfile|docker-compose|\.gitlab-ci|jenkinsfile|makefile|(^|/)deploy|k8s|helm|terraform|\.tf$")),
    ("configuration", re.compile(r"(?i)config|settings|\.env|\.ya?ml$|\.toml$|\.ini$")),
]
LOW_VALUE = re.compile(r"(?i)(^|/)(package-lock\.json|yarn\.lock|pnpm-lock\.yaml|cargo\.lock|poetry\.lock|go\.sum|composer\.lock|gemfile\.lock)$|\.min\.(js|css)$|(^|/)(dist|build|vendor|node_modules|generated|__generated__)/|\.snap$|\.svg$|\.png$|\.jpg$|(^|/)__pycache__/|\.py[co]$|\.class$|\.o$")
# What a reviewer reads to know what the project is and what it wants
CONTEXT_FILES = [
    "AGENTS.md", ".agents/AGENTS.md", ".agent/AGENTS.md", "CLAUDE.md", "README.md", "CONTRIBUTING.md",
    "ARCHITECTURE.md", "DESIGN.md", "PRD.md", "SPEC.md", "ROADMAP.md", "PLAN.md", "SECURITY.md", "STYLEGUIDE.md",
    ".github/pull_request_template.md", ".github/PULL_REQUEST_TEMPLATE.md", ".editorconfig",
]
CONTEXT_FOLDERS = [".agents/rules", ".agent/rules", ".agents/memory", ".agents/plans", "docs", "doc", "docs/adr", "adr", ".github/workflows"]
CONTEXT_PATTERN = re.compile(r"(?i)prd|spec|plan|roadmap|requirement|architecture|design|adr|decision|rule|memory|\.ya?ml$")


def finish(answer):
    print(json.dumps(answer, ensure_ascii=False, indent=1))
    sys.exit(0)


def git(*args):
    run = subprocess.run(["git", *args], capture_output=True, text=True, encoding="utf-8", errors="replace", stdin=subprocess.DEVNULL)
    return run.returncode, run.stdout, run.stderr.strip()


def number(value, default, low, high):
    try:
        return max(low, min(high, int(value)))
    except (TypeError, ValueError):
        return default


def project_context(root):
    """The project's own files that say what it is, what it wants and how it is checked."""
    files = [name for name in CONTEXT_FILES if (root / name).is_file()]
    for folder in CONTEXT_FOLDERS:
        place = root / folder
        if place.is_dir():
            found = sorted(str(p.relative_to(root)).replace("\\", "/") for p in place.iterdir()
                           if p.is_file() and (folder != "docs" and folder != "doc" or CONTEXT_PATTERN.search(p.name)))
            files += found[:15]
    return files


def check_commands(root):
    """Test, lint, type and build commands the project files name."""
    found = []
    pkg = root / "package.json"
    if pkg.is_file():
        try:
            scripts = json.loads(pkg.read_text(encoding="utf-8")).get("scripts") or {}
        except ValueError:
            scripts = {}
        runner = "pnpm" if (root / "pnpm-lock.yaml").is_file() else "yarn" if (root / "yarn.lock").is_file() else "bun" if (root / "bun.lockb").is_file() else "npm"
        found += [f"{runner} run {name}" for name in ("test", "lint", "typecheck", "check", "build") if name in scripts]
    if (root / "Cargo.toml").is_file():
        found += ["cargo test", "cargo clippy"]
    if (root / "go.mod").is_file():
        found += ["go test ./...", "go vet ./..."]
    if any((root / f).is_file() for f in ("pyproject.toml", "pytest.ini", "setup.cfg", "tox.ini")) or (root / "tests").is_dir():
        found.append("python -m pytest")
    if (root / "CMakeLists.txt").is_file():
        found.append("cmake --build build && ctest --test-dir build")
    if (root / "pom.xml").is_file():
        found.append("mvn test")
    if any((root / f).is_file() for f in ("build.gradle", "build.gradle.kts")):
        found.append("./gradlew test")
    if any(root.glob("*.sln")) or any(root.glob("*.csproj")):
        found.append("dotnet test")
    if (root / "Package.swift").is_file():
        found.append("swift test")
    if (root / "pubspec.yaml").is_file():
        found.append("flutter test")
    for make in ("Makefile", "justfile"):
        text = (root / make).read_text(encoding="utf-8", errors="replace") if (root / make).is_file() else ""
        found += [f"{'make' if make == 'Makefile' else 'just'} {t}" for t in ("test", "check", "lint") if re.search(rf"^{t}\s*:", text, re.M)]
    return found


def risk_of(path):
    return next((why for why, pattern in RISK_PATHS if pattern.search(path)), None)


def main():
    args = {}
    if len(sys.argv) > 1 and Path(sys.argv[1]).is_file():
        try:
            args = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8")) or {}
        except ValueError:
            args = {}
    code, top, _ = git("rev-parse", "--show-toplevel")
    if code != 0:
        finish({"status": "error", "message": "This folder is not inside a git repository. Ask for the code or a diff, or read the files directly.",
                "project_context": project_context(Path.cwd()), "check_commands": check_commands(Path.cwd())})
    root = Path(top.strip())

    base = str(args.get("base") or "").strip()
    only = str(args.get("file") or "").strip().replace("\\", "/")
    context = number(args.get("context_lines"), 10, 0, 50)
    max_chars = number(args.get("max_chars"), 6000, 500, 40000)

    if base:
        code, merge_base, err = git("merge-base", base, "HEAD")
        if code != 0:
            finish({"status": "error", "message": f"`{base}` is not a branch or commit here: {err}"})
        target = [merge_base.strip()]
        what = f"changes on this branch since it left {base}, plus uncommitted changes"
        _, log, _ = git("log", "--no-merges", "--format=%h %s", f"{merge_base.strip()}..HEAD", "-20")
    else:
        code, _, _ = git("rev-parse", "--verify", "HEAD")
        target = ["HEAD"] if code == 0 else ["--cached"]
        what = "uncommitted changes (staged and unstaged)"
        _, log, _ = git("log", "--no-merges", "--format=%h %s", "-5") if code == 0 else (0, "", "")
    paths = ["--", only] if only else []

    _, numstat, _ = git("diff", "--numstat", "--no-renames", *target, *paths)
    files = []
    for line in numstat.splitlines():
        parts = line.split("\t")
        if len(parts) == 3:
            added, removed, name = parts
            entry = {"file": name, "added": added, "removed": removed}
            if TEST_FILE.search(name):
                entry["kind"] = "test"
            elif LOW_VALUE.search(name):
                entry["kind"] = "generated or lock file"
            elif risk_of(name):
                entry["risk"] = risk_of(name)
            files.append(entry)
    _, untracked, _ = git("ls-files", "--others", "--exclude-standard", *paths)
    new_files = [u for u in untracked.splitlines() if u][:30]
    if not files and not new_files:
        finish({"status": "nothing", "message": f"No {what} found." + ("" if base else " To review a branch, pass base, such as main.")})

    def changed(entry):
        return sum(int(n) for n in (entry["added"], entry["removed"]) if n.isdigit())

    # Riskiest first, then the most changed; tests and generated files last
    files.sort(key=lambda e: ("kind" in e, "risk" not in e, -changed(e)))
    lines = sum(changed(e) for e in files)
    size = "small" if lines <= 200 and len(files) <= 5 else "medium" if lines <= 1000 and len(files) <= 20 else "large"
    source = [e["file"] for e in files if "kind" not in e]
    tests = [e["file"] for e in files if e.get("kind") == "test"] + [f for f in new_files if TEST_FILE.search(f) and not LOW_VALUE.search(f)]
    languages = sorted({LANGUAGES[Path(f).suffix.lower()] for f in [e["file"] for e in files] + new_files if Path(f).suffix.lower() in LANGUAGES})

    _, diff, _ = git("diff", f"-U{context}", "--no-color", "--no-ext-diff", "--no-renames", *target, *paths)
    flags, current, line_no = [], None, 0
    for line in diff.splitlines():
        if line.startswith("+++ "):
            current = line[6:] if line.startswith("+++ b/") else None
            continue
        if line.startswith("@@"):
            match = re.search(r"\+(\d+)", line)
            line_no = int(match.group(1)) if match else 0
            continue
        if line.startswith("-") or current is None:
            continue
        if line.startswith("+"):
            text = line[1:]
            is_test = bool(TEST_FILE.search(current))
            for name, pattern, suffixes, skip_tests in RULES:
                if suffixes and not current.endswith(suffixes):
                    continue
                if skip_tests and is_test:
                    continue
                if pattern.search(text):
                    shown = text.strip()[:120]
                    if name == "possible secret":
                        shown = SECRET_VALUE.sub(lambda m: m.group(0)[:4] + "…", shown)
                    flags.append({"at": f"{current}:{line_no}", "hint": name, "line": shown})
        line_no += 1

    answer = {
        "status": "ok",
        "reviewing": what,
        "size": {"files": len(files) + len(new_files), "lines_changed": lines, "review": size},
        "languages": languages,
        "files": files[:60],
        "new_untracked_files": new_files,
        "commits": [c for c in log.splitlines() if c][:20],
        "project_context": project_context(root),
        "check_commands": check_commands(root),
        "hints": flags[:40],
        "hints_note": "Hints and risk tags show where to look first; they are not findings. Read the code around each one before reporting it.",
    }
    if source and not tests:
        answer["tests_note"] = "Code changed but no test file changed or was added: check whether the new behaviour is tested."
    if len(diff) > max_chars:
        answer["diff"] = diff[:max_chars]
        answer["diff_cut"] = f"The diff has {len(diff)} characters; only the first {max_chars} are shown. Review file by file: call again with file set to one path, riskiest first (files are listed in that order)."
    else:
        answer["diff"] = diff
    if new_files:
        answer["untracked_note"] = "New files are not in the diff: read them directly."
    finish(answer)


if __name__ == "__main__":
    main()
