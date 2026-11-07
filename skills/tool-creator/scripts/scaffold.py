#!/usr/bin/env python3
"""Creates a new skill, WebAssembly plugin or MCP package. The Engine calls it with a JSON file of arguments."""

import json
import os
import re
import sys
from datetime import datetime, timezone
from pathlib import Path

HUB_TYPE = {"skills": "skill", "plugins": "plugin", "mcp": "mcp"}

CARGO_TOML = """[package]
name = "cluaiz_plugin_{crate}"
version = "1.0.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
serde_json = "1.0"

[profile.release]
opt-level = "s"
lto = true
panic = "abort"
strip = true
"""

LIB_RS = """use serde_json::{{json, Value}};

#[no_mangle]
pub extern "C" fn allocate(len: u32) -> *mut u8 {{
    let mut buffer = Vec::<u8>::with_capacity(len as usize);
    let start = buffer.as_mut_ptr();
    core::mem::forget(buffer);
    start
}}

/// The Engine passes the function's arguments as JSON. The answer is JSON too.
///
/// # Safety
/// The Engine passes an input it wrote through `allocate`.
#[no_mangle]
pub unsafe extern "C" fn execute_cel(start: *const u8, len: u32) -> u64 {{
    let input = if len > 0 && !start.is_null() {{
        core::slice::from_raw_parts(start, len as usize)
    }} else {{
        &[]
    }};
    let answer = run(input).to_string().into_bytes();
    let (at, length) = (answer.as_ptr() as u64, answer.len() as u64);
    core::mem::forget(answer);
    (at << 32) | length
}}

fn run(input: &[u8]) -> Value {{
    let Ok(args) = serde_json::from_slice::<Value>(input) else {{
        return json!({{ "status": "error", "error": "The input must be JSON" }});
    }};
    let Some(text) = args.get("input").and_then(Value::as_str) else {{
        return json!({{ "status": "error", "error": "Give an \\"input\\"" }});
    }};
    // Replace this with what {name} does
    json!({{ "status": "success", "result": text }})
}}
"""


def fail(message):
    print(json.dumps({"status": "error", "message": message}))
    sys.exit(1)


def read_arguments():
    if len(sys.argv) < 2 or not Path(sys.argv[1]).is_file():
        fail("Expected the path of a JSON file of arguments.")
    try:
        data = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
    except ValueError as error:
        fail(f"The arguments are not valid JSON: {error}")
    if not isinstance(data, dict):
        fail("The arguments must be a JSON object.")
    return data


def tools_home():
    home = os.environ.get("CLUAIZ_HOME")
    if home:
        return Path(home) / "tools"
    return Path(os.environ.get("USERPROFILE") or os.environ.get("HOME") or ".") / ".cluaiz" / "tools"


def repository_root():
    """The cluaiz-tools repository this is running in, if any."""
    current = Path.cwd()
    for folder in [current, *current.parents]:
        if all((folder / kind / "family.json").exists() for kind in HUB_TYPE):
            return folder
    return None


def manifest(spec):
    category, tool_id, triggers = spec["category"], spec["tool_id"], spec["triggers"]
    data = {
        "id": tool_id,
        "name": spec["name"],
        "category": "utility",
        "hub_type": HUB_TYPE[category],
    }
    if category == "plugins":
        data["build_type"] = "wasm"
    data.update({
        "description": spec["description"],
        "author": {"name": spec.get("author") or "Cluaiz Community"},
        "license": "Apache-2.0",
        "tags": [],
        # The Engine reads a skill's triggers from `semantic`, and a plugin's or server's from a plain list
        "triggers": {"semantic": triggers} if category == "skills" else triggers,
        "dependencies": {"plugins": {}, "mcp": {}, "skills": {}},
    })
    files = {"icon": "assets/icon.svg"}
    version = {"updated_at": datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")}
    if category != "mcp":
        files["skill"] = "SKILL.md"
    if category == "plugins":
        version["builds_os"] = ["wasm"]
        data["functions"] = [{
            "name": tool_id.replace("-", "_"),
            "description": spec["description"],
            "parameters": {
                "type": "object",
                "properties": {"input": {"type": "string", "description": "What to work on"}},
                "required": ["input"],
            },
        }]
    if category == "mcp":
        data["execution"] = {"command": spec["command"], "args": spec["args"]}
        version.update(command=spec["command"], args=spec["args"])
    version["files"] = files
    data["latest_version"] = "1.0.0"
    data["versions"] = {"1.0.0": version}
    return data


def skill_text(spec):
    description = spec["description"].replace('"', "'")
    body = spec.get("skill_instructions") or f"# {spec['name']}\n\nDescribe when to use this and exactly what to do, step by step.\n"
    return f'---\nname: {spec["tool_id"]}\ndescription: "{description}"\n---\n\n{body.strip()}\n'


def package_files(spec):
    category = spec["category"]
    files = {
        "package.json": json.dumps(manifest(spec), indent=2) + "\n",
        "assets/icon.svg": spec.get("icon_svg") or (
            f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="24" height="24" fill="none" '
            f'stroke="#FFFFFF" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">\n'
            f'  <circle cx="12" cy="12" r="9"/>\n  <path d="M12 8v8M8 12h8"/>\n</svg>\n'
        ),
    }
    if category != "mcp":
        files["SKILL.md"] = skill_text(spec)
    if category == "plugins":
        files["Cargo.toml"] = CARGO_TOML.format(crate=spec["tool_id"].replace("-", "_"))
        files["src/lib.rs"] = LIB_RS.format(name=spec["name"])
    return files


def write_package(folder, files, overwrite):
    if (folder / "package.json").exists() and not overwrite:
        fail(f"{folder} already exists. Pass overwrite: true to replace it.")
    for relative, text in files.items():
        path = folder / relative
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(text, encoding="utf-8", newline="\n")


def main():
    args = read_arguments()
    category = str(args.get("category", "")).lower()
    if category in ("skill", "plugin"):
        category += "s"
    if category not in HUB_TYPE:
        fail("category must be skills, plugins or mcp.")
    tool_id = str(args.get("tool_id") or "")
    if not re.fullmatch(r"[a-z0-9]+(-[a-z0-9]+)*", tool_id):
        fail("tool_id must be lowercase words joined by hyphens, like my-tool.")
    for field in ("name", "description"):
        if not str(args.get(field) or "").strip():
            fail(f"{field} is required.")
    triggers = args.get("triggers") or [tool_id.replace("-", " ")]
    if isinstance(triggers, str):
        triggers = [part.strip() for part in triggers.split(",") if part.strip()]
    spec = {**args, "category": category, "tool_id": tool_id, "triggers": triggers}
    if category == "mcp":
        if not str(args.get("command") or "").strip():
            fail("An MCP package needs command, the program that starts the server, such as npx.")
        spec["args"] = [str(arg) for arg in args.get("args") or []]

    files = package_files(spec)
    overwrite = args.get("overwrite") is True
    installed = tools_home() / category / tool_id
    root = repository_root()
    in_repository = root / category / tool_id if root else None
    write_package(installed, files, overwrite)
    if in_repository:
        write_package(in_repository, files, overwrite)
        family_path = root / category / "family.json"
        family = json.loads(family_path.read_text(encoding="utf-8"))
        family["items"][tool_id] = {"type": "local", "path": f"{tool_id}/package.json"}
        family_path.write_text(json.dumps(family, indent=2) + "\n", encoding="utf-8", newline="\n")

    report = {
        "status": "success",
        "tool_id": tool_id,
        "installed_at": str(installed),
        "repository_path": str(in_repository) if in_repository else None,
        "files": sorted(files),
    }
    if category == "plugins":
        report["next"] = (
            f"Write the logic in src/lib.rs, then run: cargo build --release --target wasm32-unknown-unknown. "
            f"Copy the .wasm from target/wasm32-unknown-unknown/release/ to {installed / (tool_id + '.wasm')}. "
            "The plugin cannot run until then."
        )
    elif category == "skills":
        report["next"] = "Write the real instructions in SKILL.md."
    print(json.dumps(report, indent=2))


if __name__ == "__main__":
    main()
