---
name: git-assistant
description: "Helps with git: clear commit messages, checking staged changes for secrets and build output, and resolving merge conflicts."
---

# Git Assistant

Use git through `run_command`, or through the `git` tools if they are installed. Make a commit, merge or branch change only when the user asks for it.

## Before a commit

1. `git status --short` and `git diff --staged`, to see exactly what will be committed.
2. Stop and tell the user if the staged files include keys or passwords, `.env` files, build output such as `target/`, `dist/` or `node_modules/`, or large binary files.
3. Note the branch with `git branch --show-current`. If it is `main` or `master`, say so before committing.
4. Stage named files. Do not use `git add -A` unless the user wants everything.

## Commit message

Use the Conventional Commits form, `type(scope): summary`:

| Type | For |
|---|---|
| `feat` | A new feature |
| `fix` | A bug fix |
| `refactor` | A change that is neither |
| `docs` | Documentation only |
| `test` | Tests only |
| `perf` | Speed |
| `chore` | Build, tooling and dependencies |

- Write the summary in the imperative, such as "add" or "fix", in under 72 characters.
- One logical change per commit. If the staged changes do two things, suggest two commits.
- Say what changed and why. No hype words.

## Merge conflicts

1. List the files: `git diff --name-only --diff-filter=U`.
2. For each one, read both sides between `<<<<<<<`, `=======` and `>>>>>>>`, and work out what each side meant to do.
3. Keep both intentions where you can. If they truly disagree, ask the user.
4. Remove every marker, build or test if you can, then `git add` the file.

Never use `git push --force`, `git reset --hard` or `git clean` unless the user asks, and say first what they will discard.
