# Working details

## Before coding

- **Edge cases to consider:**
  - empty, null or malformed input;
  - limits and very large input;
  - timeouts and network or disk errors;
  - concurrent writes.
- **Done means:** name the exact command that will prove the change works before you start.

**When to ask:**

| Situation | Do |
|---|---|
| An unclear requirement that changes the design | Stop and ask one focused question |
| A minor detail with a clear convention | Follow the convention, say which you chose, go on |
| Existing tests contradict the request | Point to the test, file and line, before changing it |
| It touches an outside API or schema | Check the current upstream schema before writing types |

## Step size

- Each step does one thing, builds, and passes the tests.
- Generated files and lockfiles do not count toward the roughly 200 changed lines.
- Make no drive-by changes: no reformatting, renaming or import reordering outside the task.

## Undoing your own changes

Stop and undo when:
- three attempts fail without progress;
- the change breaks unrelated parts;
- an assumption about an API or data layout turns out to be wrong.

**Undo only what you wrote.** The user may have had uncommitted work in the same files.

1. Note the output of `git status --short` at the start, and compare with it before undoing.
2. If the start was clean:
   - a file only you changed goes back with `git checkout -- <file>`;
   - a new file you created is removed by its single path.
3. If the start was not clean, undo your own lines with edits.

**Never on your own:** `git reset --hard`, `git clean -fd` or `git checkout .`. Ask whenever you are not sure a change is yours.

**In the Cluaiz terminal agent,** the user can type `/undo` to revert the last step's file changes, or `/rewind` to go back further. Suggest these to the user; you cannot type them yourself.

**After undoing:** say what was tried and why it stopped, offer one or two alternatives, and wait for the user to choose.
