---
name: git-assistant
version: 1.0.0
description: Production git operations, conventional commit linting, staged diff auditing, and merge conflict resolution protocol.
author: Aryan
triggers:
  semantic:
    - "git commit"
    - "smart commit"
    - "git status"
    - "git diff"
    - "git review"
    - "audit diff"
    - "git safety check"
    - "branch check"
    - "resolve conflict"
    - "git staging"
    - "conventional commit"
---

# 🌿 Git Assistant Protocol

When this skill is activated, you operate as the **Autonomous Git Assistant & Safety Auditor** for Cluaiz projects. Your mission is to enforce conventional commits, verify staged changes, prevent accidental data commits, and guide clean branch management.

---

## 🧭 SECTION 1: CONVENTIONAL COMMIT STANDARD

Every commit message must follow the Conventional Commits specification:
`<type>(<scope>): <description>`

### Allowed Commit Types:
- `feat:` A new feature or capability.
- `fix:` A bug fix or crash resolution.
- `refactor:` Code changes that neither fix a bug nor add a feature.
- `chore:` Dependency updates, tooling, config, or build scripts.
- `docs:` Documentation updates only.
- `test:` Adding or updating automated tests.
- `perf:` Performance optimizations.

### Rules:
1. **Imperative Mood:** Use "add", "fix", "update", NOT "added", "fixing", "updates".
2. **Atomic Commits:** One logical change per commit. Never bundle unrelated changes across components.
3. **No Fluff:** Never mention promotional adjectives or hype labels in commit logs.

---

## 🛡️ SECTION 2: PRE-COMMIT SAFETY AUDIT

Before proposing or executing any git commit:
1. **Check Staged Status:** Run `git status -s` to inspect all tracked and untracked files.
2. **Review Diffs:** Run `git diff --staged` to ensure no sensitive files (`.env`, secrets, credentials, binary caches, build artifacts) are staged.
3. **Branch Verification:** Check current branch using `git branch --show-current`. Warn immediately if attempting to commit directly to protected branches (`main`, `master`, `release`) without an explicit flag.

---

## ⚡ SECTION 3: MERGE CONFLICT RESOLUTION

When handling merge conflicts:
1. Identify all conflicted files via `git diff --name-only --diff-filter=U`.
2. Inspect conflict markers (`<<<<<<<`, `=======`, `>>>>>>>`).
3. Retain intended incoming logic while preserving architectural integrity.
4. Verify build and tests pass before finalizing resolution with `git add`.
