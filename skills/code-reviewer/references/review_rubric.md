# Review rubric

## Severity

| Level | Meaning | Examples |
|---|---|---|
| Critical | Loses or exposes data, lets an attacker in, or crashes in normal use | SQL built from user input; a key in the code; deleting the wrong folder |
| High | Gives wrong results or fails often in a main path | An error swallowed and reported as success; a race on shared state; a process started with no timeout |
| Medium | Fails in an edge case, or makes the code hard to change safely | Empty input not handled; a resource not closed on an error; the same logic in two places |
| Low | Worth fixing, with no wrong behaviour today | A misleading name; a missing check that cannot fail yet |

## What not to report

- Formatting, brace style and naming taste, which formatters and linters handle.
- Rewrites to your preferred style that fix nothing.
- Guesses about code you did not read.

## Before you report a finding

- Find the line that causes it, not only the line where it shows.
- Say the input or state that triggers it.
- Check that your fix compiles in your head and does not break the callers.
