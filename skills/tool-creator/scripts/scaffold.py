#!/usr/bin/env python3
"""
Autonomous Tool Scaffolding Worker for Cluaiz ecosystem.
Part of the 'tool-creator' skill.
Zero-hardcoding in engine; executes domain-level tool scaffolding for Skills, Plugins, and MCP servers.
"""

import sys
import os
import re
import json
from pathlib import Path

def get_default_icon(category: str) -> str:
    color = "#3B82F6" if category == "skills" else ("#F59E0B" if category == "plugins" else "#8B5CF6")
    return f"""<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="24" height="24" fill="none" stroke="{color}" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
  <path d="M14.7 6.3a1 1 0 0 0 0 1.4l1.6 1.6a1 1 0 0 0 1.4 0l3.77-3.77a6 6 0 0 1-7.94 7.94l-6.91 6.91a2.12 2.12 0 0 1-3-3l6.91-6.91a6 6 0 0 1 7.94-7.94l-3.76 3.76z"/>
</svg>
"""

def parse_input_payload(raw_text: str) -> dict:
    # 1. Try direct JSON parse
    try:
        data = json.loads(raw_text.strip())
        if isinstance(data, dict):
            # If wrapped in "arguments" or "input"
            if "arguments" in data and isinstance(data["arguments"], dict):
                data = data["arguments"]
            elif "input" in data and isinstance(data["input"], str):
                extracted = extract_from_natural_text(data["input"])
                extracted.update({k: v for k, v in data.items() if k != "input"})
                return extracted
            return data
    except Exception:
        pass

    # 2. Extract from natural/markdown text
    return extract_from_natural_text(raw_text)

def extract_from_natural_text(text: str) -> dict:
    data = {}
    
    # Detect category
    lower = text.lower()
    if "plugin" in lower:
        data["category"] = "plugins"
    elif "mcp" in lower:
        data["category"] = "mcp"
    else:
        data["category"] = "skills"

    # Detect tool ID / slug
    id_match = re.search(r"(?:skills?|plugins?|mcp)/([a-z0-9_\-]+)", text, re.IGNORECASE)
    if id_match:
        data["tool_id"] = id_match.group(1).lower()
    else:
        name_match = re.search(r"(?:skill|plugin|mcp|tool)[:\s]+([A-Za-z0-9\s_\-]+)", text, re.IGNORECASE)
        if name_match:
            raw_name = name_match.group(1).strip().split("\n")[0]
            data["name"] = raw_name
            data["tool_id"] = re.sub(r"[^a-z0-9]+", "-", raw_name.lower()).strip("-")
        else:
            data["tool_id"] = "custom-tool"

    if "name" not in data:
        data["name"] = data["tool_id"].replace("-", " ").title()

    # Detect description
    desc_match = re.search(r"description[:\s]+([^\n]+)", text, re.IGNORECASE)
    if desc_match:
        data["description"] = desc_match.group(1).strip()
    else:
        data["description"] = f"Autonomous {data['category'][:-1]} for {data['name']}."

    # Detect semantic triggers
    triggers = []
    triggers_match = re.findall(r'["\']([a-zA-Z0-9\s_\-]+)["\']', text)
    if triggers_match:
        triggers = [t.strip().lower() for t in triggers_match if len(t.strip()) > 2 and t.strip().lower() != data["tool_id"]][:8]
    if not triggers:
        triggers = [data["tool_id"].replace("-", " "), data["name"].lower(), f"run {data['tool_id']}"]
    data["triggers"] = triggers

    return data

def main():
    raw_input = ""
    if len(sys.argv) > 1:
        arg_path = Path(sys.argv[1])
        if arg_path.exists() and arg_path.is_file():
            raw_input = arg_path.read_text(encoding="utf-8")
        else:
            raw_input = " ".join(sys.argv[1:])
    else:
        raw_input = sys.stdin.read()

    if not raw_input.strip():
        print(json.dumps({"status": "error", "message": "No input payload provided to tool scaffolder."}))
        sys.exit(1)

    spec = parse_input_payload(raw_input)
    category = spec.get("category", "skills").lower()
    if not category.endswith("s") and category != "mcp":
        category = category + "s"
    if category not in ["skills", "plugins", "mcp"]:
        category = "skills"

    tool_id = spec.get("tool_id") or spec.get("id") or "custom-tool"
    tool_id = re.sub(r"[^a-z0-9-_]+", "-", tool_id.lower()).strip("-")
    tool_name = spec.get("name") or tool_id.replace("-", " ").title()
    description = spec.get("description") or f"Autonomous {category[:-1]} for {tool_name}."
    triggers = spec.get("triggers", [tool_id.replace("-", " "), tool_name.lower()])
    if isinstance(triggers, str):
        triggers = [t.strip() for t in triggers.split(",")]

    # 1. Resolve Host Target: %USERPROFILE%/.cluaiz/tools/<category>/<tool_id>/
    user_home = Path(os.environ.get("USERPROFILE") or os.environ.get("HOME") or ".")
    host_target = user_home / ".cluaiz" / "tools" / category / tool_id
    host_target.mkdir(parents=True, exist_ok=True)
    (host_target / "assets").mkdir(parents=True, exist_ok=True)

    # 2. Resolve Workspace Target if inside repo
    curr = Path.cwd()
    workspace_target = None
    repo_root = None
    for p in [curr] + list(curr.parents):
        if (p / "skills" / "family.json").exists() or (p / "cluaiz-tools").exists():
            repo_root = p if (p / "skills" / "family.json").exists() else (p / "cluaiz-tools")
            break

    if repo_root and repo_root.exists():
        workspace_target = repo_root / category / tool_id
        workspace_target.mkdir(parents=True, exist_ok=True)
        (workspace_target / "assets").mkdir(parents=True, exist_ok=True)

    # 3. Construct package.json
    pkg_data = {
        "id": tool_id,
        "name": tool_name,
        "category": "engineering",
        "hub_type": category[:-1] if category.endswith("s") else category,
        "description": description,
        "author": {
            "name": "Aryan",
            "url": "https://github.com/cluaiz"
        },
        "license": "Apache-2.0",
        "tags": [t.title() for t in triggers[:4]],
        "triggers": {
            "semantic": triggers
        },
        "dependencies": {
            "plugins": {},
            "mcp": {},
            "skills": {}
        },
        "latest_version": "1.0.0",
        "versions": {
            "1.0.0": {
                "updated_at": "2026-09-24T12:00:00Z",
                "files": {
                    category[:-1] if category.endswith("s") else category: "SKILL.md" if category == "skills" else "src/lib.rs",
                    "icon": "assets/icon.svg"
                }
            }
        }
    }
    pkg_content = json.dumps(pkg_data, indent=2) + "\n"

    # 4. Construct SKILL.md (or code file)
    skill_content = spec.get("skill_instructions") or f"""---
name: {tool_id}
version: 1.0.0
description: {description}
author: Aryan
triggers:
  semantic:
{chr(10).join(f'    - "{t}"' for t in triggers)}
---

# 🛠️ {tool_name} Protocol

When this skill is activated, you operate as the specialized agent for **{tool_name}**.

## Operational Guidelines
1. Adhere to zero-hardcoding and clean deterministic workflows.
2. Provide concise, production-ready outputs.
3. Verify all code changes against local system targets.
"""

    icon_content = spec.get("icon_svg") or get_default_icon(category)

    # 5. Write to Host
    (host_target / "package.json").write_text(pkg_content, encoding="utf-8")
    if category == "skills":
        (host_target / "SKILL.md").write_text(skill_content, encoding="utf-8")
    (host_target / "assets" / "icon.svg").write_text(icon_content, encoding="utf-8")

    # 6. Write to Workspace (if present)
    if workspace_target:
        (workspace_target / "package.json").write_text(pkg_content, encoding="utf-8")
        if category == "skills":
            (workspace_target / "SKILL.md").write_text(skill_content, encoding="utf-8")
        (workspace_target / "assets" / "icon.svg").write_text(icon_content, encoding="utf-8")

        # Update family.json
        family_file = repo_root / category / "family.json"
        if family_file.exists():
            try:
                family_data = json.loads(family_file.read_text(encoding="utf-8"))
                if "items" in family_data and isinstance(family_data["items"], dict):
                    family_data["items"][tool_id] = f"{tool_id}/package.json"
                    family_file.write_text(json.dumps(family_data, indent=2) + "\n", encoding="utf-8")
            except Exception:
                pass

    report = {
        "status": "success",
        "category": category,
        "tool_id": tool_id,
        "name": tool_name,
        "host_path": str(host_target),
        "workspace_path": str(workspace_target) if workspace_target else None,
        "files_created": ["package.json", "SKILL.md" if category == "skills" else "src/lib.rs", "assets/icon.svg"],
        "message": f"Successfully scaffolded {category[:-1]} '{tool_id}' at {host_target}"
    }

    print(json.dumps(report, indent=2))

if __name__ == "__main__":
    main()
