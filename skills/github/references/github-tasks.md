# GitHub tasks

## CI failures

- **Getting the log:**
  - The failed steps: `run view <id> --log-failed`.
  - One job's whole log: `run view <id> --job <job-id> --log`.
  - Without `gh`, the REST fallback lists runs, jobs and failed steps but not the log text. Give the user the run's link and ask them to paste the failing part.

| Kind | Looks like | Fix |
|---|---|---|
| Lint or format | `formatting differences found`, `eslint: error`, `ruff` | Run the same tool locally and commit the result |
| Dependencies | `could not find crate`, `npm ERR! 404`, `ModuleNotFoundError` | A lock file that is stale or not committed, or a package renamed or removed |
| One platform only | Passes on Linux, fails on Windows | Path APIs instead of `/` or `\` in strings; `.gitattributes` for `CRLF`; the case of file names |
| Flaky | Passes on a re-run, or fails on one runner only | Say why it looks flaky (timing, order, network). Do not hide it with retries |
| Permissions or secrets | `Resource not accessible by integration`, `401`, an empty secret on a pull request from a fork | The workflow's `permissions:` or the repository's secrets, which the user changes |
| Time or memory | `exceeded the maximum execution time`, `Killed` | Find the slow or large step, then cache, split or shrink it |

## Issues

- **Triage one issue:**
  1. Check for version, OS, steps, and expected and actual results. Draft a question for anything that is missing.
  2. Try to reproduce it with the smallest example, and say whether it reproduced.
  3. Name the part of the project and the files to read first.
  4. Search for duplicates.
  5. Suggest a label and a priority. Label, assign or close only when the user asks.
- **Summarize many issues:** list them with `--json number,title,labels,createdAt,comments` and group them by label or theme. For each group, give the count, the oldest issue and any blockers, and point out issues with no reply.
- **Split a large issue:** write numbered tasks that each work and can be checked on their own, in an order that keeps the project working. Give them as a Markdown checklist. Pull requests link back with `Part of #123`, and the last one uses `Closes #123`.

## Pull requests

- **Without a template, the description covers:**
  - why, and the issue it closes;
  - what changed, and what it deliberately does not change;
  - how it was checked: each command and its result, plus checks by hand. A check that was not run is written as not run;
  - risk: breaking changes and how to roll back.
- **Opening it:**
  - `gh pr create --base <base> --head <branch> --title ... --body-file <file>`.
  - Add `--draft` when the work is not ready.
  - Add reviewers, labels and assignees only when the user names them.
- **After review:**
  - Answer each comment with a change or a reason.
  - Push fixes as new commits unless the repository squashes.
  - Merge only on the user's word, with the repository's method (`--squash`, `--merge` or `--rebase`).

## Releases

- **SemVer:**
  - MAJOR goes up for changes that break users: a removed or renamed API, a changed format, a required new setting.
  - MINOR goes up for new features.
  - PATCH goes up for fixes.
  - Before 1.0.0, many projects treat MINOR as breaking. Check `release list` to see how the project has done it.
- **What changed:**
  - `git log --oneline <last-tag>..HEAD`, or the pull requests merged since the last release.
  - Write the entries for users, not as raw commit messages, and leave out internal changes.
- **Changelog sections:** Added, Changed, Deprecated, Removed, Fixed, Security. Breaking changes come first, with what users must do.
- **Creating the release:**
  - Only when asked, preferably as a draft: `gh release create <tag> --notes-file <file> --draft`.
  - If a tag push starts a release workflow, say so, and push the tag only with the user's yes.
