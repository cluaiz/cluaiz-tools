---
name: code-reviewer
description: "Reviews code for bugs, security problems and duplicated logic, in local changes, pasted code, files and pull requests."
---

# Code Reviewer

Find what can break: wrong results, crashes, security holes, lost data, and logic written twice. Leave style to formatters.

## 1. Get the code

| The user gives | Do |
|---|---|
| Nothing, or "my changes" | Run `git status --short`, then `git diff -U25`, or `git diff --staged -U25` for staged changes. With the git tools, call `git_diff_unstaged` or `git_diff_staged` with `context_lines: 25` |
| A file path | Read the whole file |
| Pasted code | Review the code in the message. Do not open files |
| A pull request | Use the diff the user gives or a tool that can fetch it. If none can, ask for the diff |

For a change, read enough of the file around it to see how it is called and what it returns. A diff alone hides most bugs.

## 2. Check

Go through [`references/security_checklist.md`](references/security_checklist.md). The main questions:

- Does it do what it claims, for empty, very large and wrong input?
- Can input from outside reach a shell command, a query, a file path or a page without checks?
- Are errors reported, or hidden behind a fake success?
- Are files, processes, locks and connections released on every path, including errors?
- Is the same logic written in two places?
- Are paths, keys or machine details written into the code?

## 3. Report

Rank each finding by [`references/review_rubric.md`](references/review_rubric.md) and use [`assets/report_template.md`](assets/report_template.md).

- Name the file and line, say what goes wrong and when, and give the fix.
- Report only what you checked. If you are not sure, say so.
- If nothing is wrong, say that. Do not invent findings.
- Do not change files unless the user asks for the fixes.
