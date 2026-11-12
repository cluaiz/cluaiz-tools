#!/usr/bin/env python3
"""Creates a project workflow: the file `.agents/workflows/<name>.md` that the terminal agent runs
as `/<name>`.

The Engine calls it with a JSON file of arguments and runs it in the open folder. It also reports
the project's test and build commands it can see, so the steps use real commands. An existing
workflow is changed only with "replace": true.
"""

import json
import re
import sys
from pathlib import Path

# The agent's own commands: a workflow with one of these names would never run
BUILT_IN = {
    "model", "mode", "think", "effort", "theme", "clear", "plan", "compact", "undo", "rewind", "diff", "code",
    "init", "memory", "skills", "rules", "sessions", "help", "bye", "exit", "permissions", "yes", "ask", "strict",
}


def finish(answer):
    print(json.dumps(answer, ensure_ascii=False, indent=1))
    sys.exit(0)


def project_commands(root):
    """Test, lint and build commands the project files name."""
    found = []
    pkg = root / "package.json"
    if pkg.is_file():
        try:
            scripts = json.loads(pkg.read_text(encoding="utf-8")).get("scripts") or {}
        except ValueError:
            scripts = {}
        runner = "pnpm" if (root / "pnpm-lock.yaml").is_file() else "yarn" if (root / "yarn.lock").is_file() else "bun" if (root / "bun.lockb").is_file() else "npm"
        for name in ("test", "lint", "typecheck", "check", "build"):
            if name in scripts:
                found.append(f"{runner} run {name}" if name != "test" or runner != "npm" else "npm test")
    if (root / "Cargo.toml").is_file():
        found += ["cargo test", "cargo clippy", "cargo build"]
    if (root / "go.mod").is_file():
        found += ["go test ./...", "go vet ./..."]
    py_tests = any(root.glob("test_*.py")) or any(root.glob("tests/test_*.py")) or any(root.glob("tests/*_test.py"))
    if (root / "pyproject.toml").is_file() or (root / "pytest.ini").is_file() or (root / "setup.cfg").is_file() or py_tests:
        found.append("python -m pytest")
    for make in ("Makefile", "justfile"):
        text = (root / make).read_text(encoding="utf-8", errors="replace") if (root / make).is_file() else ""
        for target in ("test", "check", "lint", "build"):
            if re.search(rf"^{target}\s*:", text, re.M):
                found.append(f"{'make' if make == 'Makefile' else 'just'} {target}")
    return found


def main():
    args = {}
    if len(sys.argv) > 1 and Path(sys.argv[1]).is_file():
        try:
            args = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8")) or {}
        except ValueError:
            args = {}
    root = Path.cwd()
    commands = project_commands(root)
    name = str(args.get("name") or "").strip().lstrip("/").lower()
    name = re.sub(r"\.md$", "", name)
    description = " ".join(str(args.get("description") or "").split())
    steps = args.get("steps")
    if isinstance(steps, str):
        steps = [s for s in steps.splitlines() if s.strip()]
    steps = [re.sub(r"^\s*(\d+[.)]|[-*])\s*", "", str(s)).strip() for s in steps or [] if str(s).strip()]

    problems = []
    if not re.fullmatch(r"[a-z0-9][a-z0-9-]{0,39}", name):
        problems.append("name: lowercase letters, digits and dashes, such as check or release")
    elif name in BUILT_IN:
        problems.append(f"name: /{name} is one of the agent's own commands; pick another name, such as {name}-project")
    if not description:
        problems.append("description: one line saying what the workflow does")
    if not steps:
        problems.append("steps: a list of steps, each one an action with the project's real command")
    if problems:
        finish({"status": "error", "problems": problems, "project_commands": commands})

    folder = root / ".agents" / "workflows"
    target = folder / f"{name}.md"
    if target.exists() and args.get("replace") is not True:
        finish({"status": "exists", "file": f".agents/workflows/{name}.md",
                "current": target.read_text(encoding="utf-8", errors="replace")[:3000],
                "message": "This workflow exists. Show the user the change, and call again with replace true only if they agree."})

    lines = ["---", f"description: {json.dumps(description, ensure_ascii=False)}", "---", "", f"# /{name}", ""]
    lines += [f"{i}. {step}" for i, step in enumerate(steps, 1)]
    # Steps without a real command ("run the tests") still need one: name what the project has
    named = [c for c in commands if any(c in s for s in steps)]
    if commands and not named:
        lines += ["", "Commands this project has: " + ", ".join(f"`{c}`" for c in commands) + "."]
    lines += ["", "Report each command you ran and its result. Say what failed or was not run."]
    folder.mkdir(parents=True, exist_ok=True)
    target.write_text("\n".join(lines) + "\n", encoding="utf-8", newline="\n")

    answer = {"status": "ok", "file": f".agents/workflows/{name}.md", "run_with": f"/{name}",
              "content": target.read_text(encoding="utf-8"),
              "project_commands": commands,
              "next": f"Tell the user to type /{name} in cluaiz agent (text after it is passed on). Run the steps once now if they asked to check it works."}
    if commands and not named:
        answer["note"] = "The steps named no command, so the project's commands were added to the file."
    finish(answer)


if __name__ == "__main__":
    main()
