---
name: build-feature
description: "A way of working for any code change bigger than a line or two, in any language: understand the task and the project, plan small steps, write a failing test, find the root cause of bugs before fixing, make the change, and verify before saying it is done. Use when the user asks to add a feature, fix a bug, refactor, or change behaviour in code."
---

# Build Feature

Links such as `references/...` are files of this skill. Read them with `read_tool_file`. In the terminal agent, `read_item` with `build-feature` gives the folder; then call `read_file` on the folder plus the link.

## Understand

- Restate the goal in one or two sentences: the input, what happens, and the output. Also say what the change will not do. If something that changes the result is unclear, ask one short question. Otherwise go on and say what you assumed.
- Read the code you will change and the code that calls it.
- Read what the project decided: `AGENTS.md`, `CONTRIBUTING.md`, `.agents/rules/`, memory, and a PRD, spec or plan for this work. The `workflow-management` skill explains the `.agents/` folder and how to size a task.
- Find how the project builds and tests. The CI workflow (`.github/workflows/*.yml`) is the most reliable list. Otherwise look in the project files:
  - `package.json` scripts, run with the tool of its lockfile;
  - `cargo test` and `cargo clippy`, with `-p <crate>` in a workspace;
  - `python -m pytest`, because plain `pytest` may not find the project's modules;
  - `go test ./...` and `go vet ./...`;
  - `Makefile` or `justfile` targets.
- Run one command per call. Windows PowerShell has no `&&`.

## Plan

- Name the files to change, what changes in each, and how you will check it.
- Keep the change as small as the task allows. Do not rename, reformat or "improve" code the task does not need.
- Split large work into steps that each build and pass. A good step is about 200 changed lines; the usual order is types, then logic with tests, then the entry point, then an end-to-end test and docs.
- Edge cases and when to ask: [references/working-details.md](references/working-details.md).

## Bugs: find the cause first

1. Reproduce the bug: run it, or read the error and the stack trace.
2. Trace it back to where the wrong value first appears, not where it shows up. Check what changed recently (`git log -p` on the file).
3. Form one hypothesis and test it, one change at a time.

Do not stack guesses. A fix without a known cause usually moves the bug somewhere else.

## Test first

- **For a bug:** write a test that fails because of the bug, and watch it fail.
- **For a feature:** write tests for the main behaviour and for an edge case such as empty input or a failure.
- Follow the project's test style, framework and location.
- If the project has no tests, write the smallest check you can run, and say so.

## Build and verify

- Make the change and run the test after each step.
- After three failed attempts, stop. Report what you tried and what you think is wrong, with one or two other approaches. Undo only your own edits: [references/working-details.md](references/working-details.md).
- When the tests pass:
  1. Run the whole suite, plus the linter or type check if the project has one.
  2. Run the program once the way a user would.
  3. Read your own `git diff` and remove debug prints, leftovers and unrelated changes.
- For a larger change, review it the way the `code-reviewer` skill does.

## Report

- Say what changed, file by file.
- Quote the commands you ran and their results.
- Say what you did not test.
- Do not say it works unless a test or a run showed it.

Commit only when the user asks, using the `git-assistant` skill. If a decision should outlive the conversation, such as a test command or a design choice, propose it as project memory (`remember`).
