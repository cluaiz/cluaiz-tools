#!/usr/bin/env python3
"""Checks what `git commit` would record, before it is recorded.

The Engine calls it with a JSON file of arguments and runs it in the open folder. It only reads:
it never stages, commits or changes a file. Secrets it finds are shown redacted, never in full.
"""

import json
import re
import subprocess
import sys
from pathlib import Path

BLOCKED_PATHS = [
    (re.compile(r"(^|/)\.env(\.[^/]*)?$", re.I), "environment file"),
    (re.compile(r"\.(pem|key|p12|pfx|keystore|jks)$", re.I), "key or certificate file"),
    (re.compile(r"(^|/)id_(rsa|dsa|ecdsa|ed25519)$", re.I), "SSH private key"),
    (re.compile(r"(^|/)(credentials|service[-_]account)[^/]*\.json$", re.I), "credentials file"),
    (re.compile(r"(^|/)(node_modules|target|dist|build|\.next|__pycache__|\.venv|venv)/", re.I), "build output or dependencies"),
    (re.compile(r"\.(gguf|safetensors|onnx|pt|ckpt|sqlite|db|dump|iso|exe|dll|so|dylib)$", re.I), "large binary"),
    (re.compile(r"(^|/)(\.DS_Store|Thumbs\.db)$", re.I), "operating system file"),
]

ENV_EXAMPLE = re.compile(r"(^|/)\.env\.(example|sample|template|dist)$", re.I)

SECRETS = [
    (re.compile(r"-----BEGIN [A-Z ]*PRIVATE KEY-----"), "private key"),
    (re.compile(r"\bAKIA[0-9A-Z]{16}\b"), "AWS access key"),
    (re.compile(r"\b(ghp|gho|ghu|ghs|ghr)_[A-Za-z0-9]{36}\b"), "GitHub token"),
    (re.compile(r"\bgithub_pat_[A-Za-z0-9_]{60,}\b"), "GitHub token"),
    (re.compile(r"\bglpat-[A-Za-z0-9_-]{20}\b"), "GitLab token"),
    (re.compile(r"\bxox[abprs]-[A-Za-z0-9-]{10,}\b"), "Slack token"),
    (re.compile(r"\bsk-ant-[A-Za-z0-9_-]{20,}\b"), "Anthropic key"),
    (re.compile(r"\bsk-(proj-)?[A-Za-z0-9_-]{32,}\b"), "OpenAI-style key"),
    (re.compile(r"\bgsk_[A-Za-z0-9]{40,}\b"), "Groq key"),
    (re.compile(r"\bAIza[0-9A-Za-z_-]{35}\b"), "Google API key"),
    (re.compile(r"\bhf_[A-Za-z0-9]{30,}\b"), "Hugging Face token"),
    (re.compile(r"\b(sk|rk)_live_[A-Za-z0-9]{20,}\b"), "Stripe key"),
]

# `password = "..."` and similar, with a value that looks real
ASSIGNED = re.compile(
    r"(?i)\b([A-Za-z0-9_]*(?:api[_-]?key|secret|password|passwd|token|private[_-]?key|access[_-]?key)[A-Za-z0-9_]*)"
    r"\s*[:=]\s*[\"']([^\"'\s]{12,})[\"']"
)
PLACEHOLDER = re.compile(r"(?i)(your|example|sample|dummy|changeme|placeholder|xxx|\*\*\*|<|\$\{|\{\{|env\(|process\.env|os\.environ|(?:^|[_-])test)")
CONFLICT = re.compile(r"^(<{7}|={7}|>{7})( |$)")
LARGE_BYTES = 5 * 1024 * 1024


def finish(answer):
    print(json.dumps(answer, ensure_ascii=False, indent=1))
    sys.exit(0)


def git(*args):
    run = subprocess.run(["git", *args], capture_output=True, text=True, encoding="utf-8", errors="replace")
    return run.returncode, run.stdout


def redact(value):
    return value[:4] + "…" + f"({len(value)} chars)"


CONVENTIONAL = re.compile(r"^(feat|fix|docs|style|refactor|perf|test|build|ci|chore|revert)(\([^)]+\))?!?: ")
CONVENTION_FILES = ["CONTRIBUTING.md", "AGENTS.md", ".agents/AGENTS.md", "commitlint.config.js", "commitlint.config.cjs",
                    "commitlint.config.mjs", ".commitlintrc", ".commitlintrc.json", ".commitlintrc.yml", ".gitmessage",
                    ".github/pull_request_template.md", ".github/PULL_REQUEST_TEMPLATE.md", ".pre-commit-config.yaml",
                    ".husky", "lefthook.yml", "CODEOWNERS", ".github/CODEOWNERS"]


def conventions(root):
    """How this repository is worked in: the commit style of its history, its convention files,
    hooks, signing, and where the branch stands against its upstream."""
    _, log = git("log", "--no-merges", "--format=%s", "-15")
    subjects = [s for s in log.splitlines() if s]
    style = "unknown"
    if subjects:
        share = sum(1 for s in subjects if CONVENTIONAL.match(s)) / len(subjects)
        style = "conventional commits" if share >= 0.6 else "free form (follow the examples)" if share < 0.2 else "mixed"
    info = {
        "recent_subjects": subjects[:8],
        "commit_style": style,
        "convention_files": [f for f in CONVENTION_FILES if (root / f).exists()],
    }
    hooks = root / ".git" / "hooks"
    active = [h.name for h in hooks.iterdir() if h.is_file() and not h.name.endswith(".sample")] if hooks.is_dir() else []
    if active or (root / ".husky").exists() or (root / ".pre-commit-config.yaml").exists() or (root / "lefthook.yml").exists():
        info["hooks"] = active or ["configured"]
    _, sign = git("config", "--get", "commit.gpgsign")
    if sign.strip() == "true":
        info["signing"] = "commits are signed here: do not turn it off"
    code, upstream = git("rev-parse", "--abbrev-ref", "--symbolic-full-name", "@{u}")
    if code == 0:
        _, counts = git("rev-list", "--left-right", "--count", "HEAD...@{u}")
        ahead, _, behind = counts.strip().partition("	")
        info["upstream"] = {"branch": upstream.strip(), "ahead": int(ahead or 0), "behind": int(behind or 0)}
    else:
        info["upstream"] = None
    return info


def main():
    arguments = {}
    if len(sys.argv) > 1 and Path(sys.argv[1]).is_file():
        try:
            arguments = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8")) or {}
        except ValueError:
            arguments = {}
    code, root = git("rev-parse", "--show-toplevel")
    if code != 0:
        finish({"status": "error", "message": "This folder is not inside a git repository."})
    repo = conventions(Path(root.strip()))
    _, branch = git("branch", "--show-current")
    branch = branch.strip() or "(detached HEAD)"
    _, names = git("diff", "--cached", "--name-only", "--no-renames", "-z")
    staged = [n for n in names.split("\0") if n]
    if not staged:
        _, status = git("status", "--short")
        finish({
            "status": "nothing_staged",
            "branch": branch,
            "message": "Nothing is staged. Stage named files with `git add <file>` first.",
            "unstaged_or_untracked": status.splitlines()[:30],
            "repo": repo,
        })

    problems = []
    for name in staged:
        if ENV_EXAMPLE.search(name):
            continue
        for pattern, kind in BLOCKED_PATHS:
            if pattern.search(name):
                problems.append({"file": name, "problem": kind})
                break
        code, size = git("cat-file", "-s", f":{name}")
        if code == 0 and size.strip().isdigit() and int(size) > LARGE_BYTES:
            problems.append({"file": name, "problem": f"large file ({int(size) // (1024 * 1024)} MB)"})

    _, diff = git("diff", "--cached", "--unified=0", "--no-color", "--no-ext-diff")
    current, line_no = None, 0
    for line in diff.splitlines():
        if line.startswith("+++ "):
            current = line[6:] if line.startswith("+++ b/") else None
            continue
        if line.startswith("@@"):
            match = re.search(r"\+(\d+)", line)
            line_no = int(match.group(1)) if match else 0
            continue
        if not line.startswith("+") or current is None:
            continue
        text = line[1:]
        where = f"{current}:{line_no}"
        line_no += 1
        if CONFLICT.match(text):
            problems.append({"file": where, "problem": "conflict marker left in the file"})
        for pattern, kind in SECRETS:
            found = pattern.search(text)
            if found:
                problems.append({"file": where, "problem": f"{kind}: {redact(found.group(0))}"})
        for key, value in ASSIGNED.findall(text):
            if not PLACEHOLDER.search(value) and not any(p.search(value) for p, _ in SECRETS):
                problems.append({"file": where, "problem": f"{key} has a value that looks real: {redact(value)}"})

    _, stat = git("diff", "--cached", "--numstat", "--no-renames")
    changes = []
    for row in stat.splitlines():
        parts = row.split("	")
        if len(parts) == 3:
            changes.append({"file": parts[2], "added": parts[0], "removed": parts[1]})
    areas = sorted({name.split("/")[0] if "/" in name else "(root)" for name in staged})
    answer = {
        "status": "stop" if problems else "ok",
        "branch": branch,
        "staged_files": len(staged),
        "files": changes[:40] or staged[:40],
        "problems": problems[:40],
        "repo": repo,
    }
    if len(areas) > 2:
        answer["split_hint"] = f"The staged files touch {len(areas)} areas ({', '.join(areas[:6])}): check whether this is one change or several commits."
    if branch in ("main", "master"):
        answer["note"] = f"The branch is {branch}: tell the user before committing."
    answer["next"] = (
        "Do not commit. Show the user each problem and ask how to proceed; for a secret, it must be removed and the key rotated."
        if problems else "Nothing risky was found. Write the commit message, show it to the user, and commit only if they asked."
    )
    finish(answer)


if __name__ == "__main__":
    main()
