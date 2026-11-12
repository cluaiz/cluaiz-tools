#!/usr/bin/env python3
"""Checks a Cluaiz tool package (skill, plugin or MCP server) against what the Engine and the
cluaiz-tools registry need: the manifest, the file list the installer uses, SKILL.md for the
model, README.md for people, links, the icon, functions and scripts.

Used by scaffold.py for `validate_tool`, and on its own: python validate.py <tool folder>
It only reads.
"""

import json
import os
import re
import sys
from pathlib import Path

# The files `read_tool_file` can read (agents/src/tool_search.rs `is_text_file`)
READABLE = {"md", "txt", "json", "yaml", "yml", "toml", "py", "js", "ts", "sh", "ps1", "rs", "html", "css", "csv", "xml"}
SKIP_DIRS = {"scratch", "node_modules", "target", "__pycache__", ".git"}
STANDARD_FIELDS = {"name", "description", "license", "compatibility", "metadata", "allowed-tools"}
ENGINE_FIELDS = {"version", "triggers", "execution_mode", "default_turns", "security_mode"}
HYPE = re.compile(
    r"(?i)\b(100%|zero[- ]latency|bulletproof|blazing|magic(al)?|industrial|sovereign(ty)?|world[- ]class|"
    r"cutting[- ]edge|state[- ]of[- ]the[- ]art|revolutionary|guaranteed?|enterprise[- ]grade)\b"
)
LINK = re.compile(r"\[[^\]]*\]\(([^)\s]+)\)")
HUB = {"skill": "skills", "plugin": "plugins", "mcp": "mcp"}


def tools_home():
    home = os.environ.get("CLUAIZ_HOME")
    if home:
        return Path(home) / "tools"
    return Path(os.environ.get("USERPROFILE") or os.environ.get("HOME") or ".") / ".cluaiz" / "tools"


def find_folder(target):
    """A path to a tool folder, or a tool id looked up in the open folder's repository, then in the installed tools."""
    path = Path(target).expanduser()
    if (path / "package.json").is_file():
        return path.resolve()
    if re.fullmatch(r"[a-z0-9]+(-[a-z0-9]+)*", target):
        roots = [Path.cwd(), *Path.cwd().parents]
        for root in roots:
            for kind in HUB.values():
                if (root / kind / target / "package.json").is_file():
                    return (root / kind / target).resolve()
        for kind in HUB.values():
            if (tools_home() / kind / target / "package.json").is_file():
                return (tools_home() / kind / target).resolve()
    return None


def front_matter(text):
    match = re.match(r"^---\r?\n(.*?)\r?\n---\r?\n?(.*)$", text, re.S)
    if not match:
        return None, text
    fields, key = {}, None
    for line in match.group(1).splitlines():
        found = re.match(r"^([A-Za-z_-]+):\s*(.*)$", line)
        if found:
            key = found.group(1)
            fields[key] = found.group(2).strip().strip('"').strip("'")
        elif key and line.strip():
            fields[key] += " " + line.strip()
    return fields, match.group(2)


def listed_files(value, found):
    if isinstance(value, str):
        if value not in found:
            found.append(value)
    elif isinstance(value, list):
        for item in value:
            listed_files(item, found)
    elif isinstance(value, dict):
        for item in value.values():
            listed_files(item, found)
    return found


def disk_files(folder):
    out = []
    for path in sorted(folder.rglob("*")):
        rel = path.relative_to(folder)
        if any(part in SKIP_DIRS or part.startswith(".") for part in rel.parts):
            continue
        if path.is_file():
            out.append(rel.as_posix())
    return out


def check_links(name, text, folder, errors, warnings):
    in_code = False
    for number, line in enumerate(text.splitlines(), 1):
        if line.strip().startswith("```"):
            in_code = not in_code
            continue
        if in_code:
            continue
        for target in LINK.findall(line):
            if target.startswith(("http://", "https://", "mailto:", "#")):
                continue
            if target.startswith("file:") or re.match(r"^[A-Za-z]:[\\/]", target) or target.startswith("/"):
                errors.append(f"{name}:{number}: link `{target}` is an absolute path; use a path relative to the tool folder, such as references/guide.md")
                continue
            relative = target.split("#")[0]
            if relative and not (folder / relative).exists():
                errors.append(f"{name}:{number}: link `{target}` points to a file that does not exist")


def validate(folder):
    errors, warnings = [], []
    folder = Path(folder)
    try:
        pkg = json.loads((folder / "package.json").read_text(encoding="utf-8"))
    except (OSError, ValueError) as error:
        return [f"package.json cannot be read: {error}"], warnings, {}

    tool_id = pkg.get("id", "")
    for field in ("id", "name", "category", "hub_type", "description", "license", "latest_version", "versions"):
        if not pkg.get(field):
            errors.append(f"package.json: `{field}` is missing")
    if pkg.get("icon") != "assets/icon.svg":
        warnings.append("package.json: `icon` should be \"assets/icon.svg\" (the App and the website read it)")
    if tool_id and tool_id != folder.name:
        errors.append(f"package.json: id `{tool_id}` must equal the folder name `{folder.name}`")
    if tool_id and not re.fullmatch(r"[a-z0-9]+(-[a-z0-9]+)*", tool_id):
        errors.append(f"package.json: id `{tool_id}` must be lowercase words joined by hyphens")
    hub = pkg.get("hub_type", "")
    if hub and hub not in HUB and hub != "extension":
        errors.append(f"package.json: hub_type `{hub}` must be skill, plugin, mcp or extension")
    description = str(pkg.get("description") or "")
    if description and not re.search(r"(?i)\buse\b", description):
        warnings.append("package.json: the description should also say when to use the tool (\"Use when ...\")")

    family = folder.parent / "family.json"
    if family.is_file():
        try:
            items = json.loads(family.read_text(encoding="utf-8")).get("items", {})
            if tool_id not in items:
                errors.append(f"{folder.parent.name}/family.json does not list `{tool_id}`")
        except ValueError:
            errors.append(f"{folder.parent.name}/family.json is not valid JSON")

    # The file list: a tool without a release zip is installed from exactly these files
    latest = pkg.get("latest_version")
    version = (pkg.get("versions") or {}).get(latest) if latest else None
    if latest and version is None:
        errors.append(f"package.json: versions has no entry for latest_version `{latest}`")
    version = version or {}
    listed = listed_files(version.get("files") or {}, [])
    listed = [f for f in listed if not f.startswith(("http://", "https://"))]
    on_disk = disk_files(folder)
    for path in listed:
        if ".." in Path(path).parts or path.startswith("/"):
            errors.append(f"versions.{latest}.files: `{path}` must stay inside the folder")
        elif not (folder / path).is_file():
            errors.append(f"versions.{latest}.files lists `{path}`, which does not exist")
    build_only = re.compile(r"^(src/|Cargo\.(toml|lock)$|package-lock\.json$|tests?/)")
    for path in on_disk:
        if path != "package.json" and path not in listed and not (hub == "plugin" and build_only.match(path)):
            errors.append(f"`{path}` is not in versions.{latest}.files, so it is not installed")

    # People and the model
    if not (folder / "README.md").is_file():
        errors.append("README.md is missing (the documentation people read)")
    else:
        readme = (folder / "README.md").read_text(encoding="utf-8", errors="replace")
        if "```mermaid" not in readme:
            warnings.append("README.md has no Mermaid diagram of how the tool works")
        check_links("README.md", readme, folder, errors, warnings)
        for number, line in enumerate(readme.splitlines(), 1):
            if HYPE.search(line):
                warnings.append(f"README.md:{number}: hype word `{HYPE.search(line).group(0)}`; say what it does and what was measured")
    skill_path = folder / "SKILL.md"
    if not skill_path.is_file():
        errors.append("SKILL.md is missing (the instructions the model reads)")
    else:
        text = skill_path.read_text(encoding="utf-8", errors="replace")
        fields, body = front_matter(text)
        if fields is None:
            errors.append("SKILL.md has no front matter between --- lines")
        else:
            if hub == "skill" and fields.get("name") != tool_id:
                errors.append(f"SKILL.md: name `{fields.get('name')}` must equal the id `{tool_id}`")
            skill_description = fields.get("description", "")
            if not skill_description or len(skill_description) > 1024:
                errors.append(f"SKILL.md: description must be 1 to 1024 characters (it is {len(skill_description)})")
            elif not re.search(r"(?i)\buse\b", skill_description):
                warnings.append("SKILL.md: the description should say when to use the skill (\"Use when ...\")")
            ignored = sorted(set(fields) - STANDARD_FIELDS - ENGINE_FIELDS)
            if ignored:
                warnings.append(f"SKILL.md: front matter fields the Engine ignores: {', '.join(ignored)}")
        lines = body.count("\n") + 1
        if lines > 500:
            errors.append(f"SKILL.md body has {lines} lines; the limit is 500")
        elif lines > 150:
            warnings.append(f"SKILL.md body has {lines} lines; small models do better under about 150. Move details to references/")
        if "```mermaid" in body:
            warnings.append("SKILL.md has a Mermaid diagram; it costs the model tokens. Keep diagrams in README.md")
        check_links("SKILL.md", body, folder, errors, warnings)
        for target in LINK.findall(body):
            ext = target.rsplit(".", 1)[-1].lower() if "." in target else ""
            if not target.startswith("http") and ext and ext not in READABLE:
                warnings.append(f"SKILL.md links `{target}`, which read_tool_file cannot read (.{ext})")
        for number, line in enumerate(body.splitlines(), 1):
            if HYPE.search(line):
                warnings.append(f"SKILL.md:{number}: hype word `{HYPE.search(line).group(0)}`")

    readable = [p for p in on_disk if p.rsplit(".", 1)[-1].lower() in READABLE and p not in ("SKILL.md", "package.json")]
    if len(readable) > 40:
        warnings.append(f"{len(readable)} text files; the model is shown only the first 40")

    # Icon
    icon = folder / "assets" / "icon.svg"
    if not icon.is_file():
        errors.append("assets/icon.svg is missing")
    else:
        svg = icon.read_text(encoding="utf-8", errors="replace")
        if "<svg" not in svg or "</svg>" not in svg:
            errors.append("assets/icon.svg is not an SVG document")
        if 'viewBox="0 0 24 24"' not in svg:
            warnings.append("assets/icon.svg should have viewBox=\"0 0 24 24\"")
        if icon.stat().st_size > 50 * 1024:
            errors.append("assets/icon.svg is larger than 50 KB")

    # Matching
    triggers = pkg.get("triggers")
    if hub == "skill":
        if not (isinstance(triggers, dict) and isinstance(triggers.get("semantic"), list) and triggers["semantic"]):
            warnings.append("A skill's triggers go in triggers.semantic, a list of phrases a user would say")
    elif hub in ("plugin", "mcp") and not isinstance(triggers, list):
        errors.append("A plugin's or MCP server's triggers must be a plain list; triggers.semantic has no effect for them")

    # Dependencies: one format everywhere (docs/package-spec.md#dependencies)
    deps = pkg.get("dependencies")
    if deps is not None:
        if not isinstance(deps, dict):
            errors.append("package.json: dependencies must be an object with plugins, mcp and skills")
        else:
            for kind, entries in deps.items():
                if kind not in ("plugins", "mcp", "skills", "extensions"):
                    errors.append(f"dependencies.{kind}: only plugins, mcp, skills and extensions are allowed")
                    continue
                if not isinstance(entries, dict):
                    errors.append(f"dependencies.{kind} must be an object of tool ids")
                    continue
                for dep_id, spec in entries.items():
                    if not (isinstance(spec, dict) and isinstance(spec.get("version"), str) and isinstance(spec.get("url"), str)):
                        errors.append(f"dependencies.{kind}.{dep_id} must be {{\"version\": \"^1.0.0\", \"url\": \"https://raw.githubusercontent.com/cluaiz/cluaiz-tools/main/{kind}/{dep_id}/package.json\"}}")

    # Functions and the script that runs them
    functions = pkg.get("functions") or pkg.get("tools") or []
    names = []
    for index, function in enumerate(functions):
        name = function.get("name") or (function.get("function") or {}).get("name")
        params = function.get("parameters") or (function.get("function") or {}).get("parameters")
        if not name:
            errors.append(f"functions[{index}] has no name")
            continue
        names.append(name)
        if not (function.get("description") or (function.get("function") or {}).get("description")):
            errors.append(f"function `{name}` has no description")
        if not isinstance(params, dict) or params.get("type") != "object":
            errors.append(f"function `{name}`: parameters must be a JSON schema object")
    if len(names) != len(set(names)):
        errors.append("two functions have the same name")
    execution = pkg.get("execution")
    if functions and hub == "skill" and not execution:
        errors.append("a skill with functions needs execution (script_runner) to run them")
    if isinstance(execution, dict) and hub != "mcp":
        command = str(execution.get("command") or "")
        for script in re.findall(r"\{tool_dir\}/([^\"'\s]+)", command):
            if not (folder / script).is_file():
                errors.append(f"execution.command runs `{script}`, which does not exist")
        if command and "{file}" not in command:
            errors.append("execution.command must pass the arguments file with {file}")
    if hub == "skill" and functions and (folder / "SKILL.md").is_file():
        body = (folder / "SKILL.md").read_text(encoding="utf-8", errors="replace")
        for name in names:
            if name not in body:
                warnings.append(f"SKILL.md never names the function `{name}`; small models use a function more often when the instructions say when to call it")

    # Scripts must at least compile
    for path in on_disk:
        if path.endswith(".py"):
            try:
                compile((folder / path).read_text(encoding="utf-8"), path, "exec")
            except (SyntaxError, ValueError, UnicodeDecodeError) as error:
                errors.append(f"{path} does not compile: {error}")
        elif path.endswith((".flow.yaml", ".flow.yml", ".flow.json")):
            warnings.append(f"{path}: check it with `cluaiz flow validate {path}`")

    return errors, warnings, pkg


def validate_command(args):
    target = str(args.get("path") or args.get("tool_id") or "").strip()
    if not target:
        print(json.dumps({"status": "error", "message": "Give path (a tool folder) or tool_id."}))
        sys.exit(1)
    folder = find_folder(target)
    if folder is None:
        print(json.dumps({"status": "error", "message": f"No tool package (a folder with package.json) found for `{target}`."}))
        sys.exit(1)
    errors, warnings, pkg = validate(folder)
    print(json.dumps({
        "status": "fail" if errors else "pass",
        "tool": pkg.get("id"),
        "folder": str(folder),
        "errors": errors,
        "warnings": warnings,
        "next": "Fix every error, then validate again." if errors else "No errors. Read the warnings, then test the tool: call it once with good input and once with bad input.",
    }, indent=1, ensure_ascii=False))


if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: validate.py <tool folder or id>")
        sys.exit(2)
    validate_command({"path": sys.argv[1]})
