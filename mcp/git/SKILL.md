---
name: git
description: "Reads a Git repository, and can stage, commit, branch and merge in it. Use for Git work in a repository on this computer."
---

# Git

Every call needs `repo_path`. Use `.` for the open folder.

| To | Call |
|---|---|
| See the state | `git_status`, `git_branch` with `branch_type: "local"`, `git_remote` |
| See changes | `git_diff_unstaged`, `git_diff_staged`, `git_diff` with a `target` |
| See history | `git_log` with `max_count`, `git_show` with a `revision` |
| Save work | `git_add` with `files`, then `git_commit` with a `message` |
| Branches | `git_create_branch`, `git_checkout`, `git_merge` |
| Unstage everything | `git_reset`. It does not change the files |

## Rules

- Commit, merge or change branch only if the user asked for it.
- Before a commit, check `git_status` and `git_diff_staged`. Stage named files, and do not commit keys, `.env` files or build output.
- Merging is fast-forward only unless you pass `fast_forward_only: false`. Ask the user first, and tell them about any conflict.
- There is no push here, so say that a commit stays on this computer.
