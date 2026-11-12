# Build Feature

A way of working for a new feature or a bug fix: understand the request, plan the smallest change, write a failing test, make it pass, verify, and report only what was actually run. It is instructions for the model, with no program of its own.

## How it works

```mermaid
flowchart TD
    Ask["You ask for a feature or a fix"] --> Understand["Restate it, read the code and the project's rules"]
    Understand --> Plan["Plan the files and the test"]
    Plan --> Test["Write a test that fails"]
    Test --> Build["Change the code until it passes"]
    Build --> Stuck{"Still failing after 3 tries?"}
    Stuck -- "Yes" --> Report["Stop and explain"]
    Stuck -- "No" --> Verify["Full test suite, lint, one real run"]
    Verify --> Done["Report: what changed, what ran, what did not"]
```

## Files

| File | Read when |
|---|---|
| [references/working-details.md](references/working-details.md) | Edge cases and when to ask, step size, undoing your own edits safely |

## Install

```bash
cluaiz skill install build-feature
```

It works best in the terminal agent (`cluaiz agent`), where the model can edit files and run the project's tests with your approval.

## Example prompts

- "Add a --json flag to the export command."
- "Fix: the importer crashes on an empty file."

## Limits

- It follows the project's own test and lint commands; it does not install test frameworks unless you agree.
- The model undoes only its own edits; it never runs `git reset --hard` or `git clean -fd` on its own.
