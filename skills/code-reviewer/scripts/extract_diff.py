#!/usr/bin/env python3
"""
extract_diff.py - Fast Git Diff & Surrounding Context Extractor for Code Reviews.

Extracts staged or working tree diffs and enriches each changed hunk with
surrounding context lines (default: 25 lines) from the underlying source file.
"""

import sys
import subprocess
import argparse
from pathlib import Path


def run_git_command(args, cwd=None):
    """Executes a git command and returns stdout text."""
    try:
        result = subprocess.run(
            ["git"] + args,
            cwd=cwd,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
            check=True,
            encoding="utf-8",
            errors="replace"
        )
        return result.stdout
    except subprocess.CalledProcessError as e:
        sys.stderr.write(f"Git command failed: git {' '.join(args)}\n{e.stderr}\n")
        return ""
    except FileNotFoundError:
        sys.stderr.write("Git executable not found in PATH.\n")
        return ""


def get_modified_files(staged=False, cwd=None):
    """Returns a list of modified files."""
    args = ["diff", "--name-only"]
    if staged:
        args.append("--staged")
    output = run_git_command(args, cwd=cwd)
    return [line.strip() for line in output.splitlines() if line.strip()]


def extract_context_around_hunk(file_path, start_line, end_line, context_lines=25, cwd=None):
    """Reads surrounding source code lines around a diff hunk."""
    full_path = Path(cwd or ".") / file_path
    if not full_path.is_file():
        return ""

    try:
        lines = full_path.read_text(encoding="utf-8", errors="replace").splitlines()
    except Exception:
        return ""

    total = len(lines)
    ctx_start = max(0, start_line - context_lines - 1)
    ctx_end = min(total, end_line + context_lines)

    numbered_lines = [
        f"{i + 1:4d} | {lines[i]}"
        for i in range(ctx_start, ctx_end)
    ]
    return "\n".join(numbered_lines)


def main():
    parser = argparse.ArgumentParser(description="Extract git diff with surrounding context for code reviews.")
    parser.add_argument("--staged", action="store_true", help="Inspect staged changes instead of working tree")
    parser.add_argument("--context", type=int, default=25, help="Number of surrounding context lines (default: 25)")
    parser.add_argument("--file", type=str, default=None, help="Filter diff to a specific file")
    parser.add_argument("--cwd", type=str, default=".", help="Repository working directory")

    args = parser.parse_args()

    # 1. Fetch raw diff
    diff_args = ["diff", "-U3"]
    if args.staged:
        diff_args.append("--staged")
    if args.file:
        diff_args.extend(["--", args.file])

    raw_diff = run_git_command(diff_args, cwd=args.cwd)
    if not raw_diff.strip():
        print("No uncommitted or staged changes detected in repository.")
        return

    # 2. Print structured payload for code reviewer
    print("=== GIT DIFF FOR CODE REVIEW ===")
    print(raw_diff)
    print("\n=== MODIFIED FILES ===")
    files = get_modified_files(staged=args.staged, cwd=args.cwd)
    for f in files:
        print(f"- {f}")


if __name__ == "__main__":
    main()
