---
name: code-interpreter
description: "Runs code on this computer and returns its real output. Use it to run code or check a result by running it."
---

# Code Interpreter

Call `execute_code` to run code. Never write out what code would print: run it, and report what it really printed.

```json
{"language": "python", "code": "print(sum(range(101)))"}
```

The answer is JSON with `exit_code`, `stdout` and `stderr`.

## Steps

1. Print the final value. Only what the code prints comes back.
2. Read the answer. If `exit_code` is not 0, read `stderr`, fix the code and run it again. After three failures, tell the user what fails.
3. Report the code you ran, its output, and what it means.

## When a language is not installed

The error says the command was not found. Tell the user, and offer another language that can do the job. Install something only if the user asks.

## Limits

- A run is stopped after 30 seconds, or after `timeout` seconds if you pass it, along with anything it started.
- Code runs in the open folder with the user's rights, and it is not sandboxed. Do not delete or overwrite anything the user did not ask you to.
- Do not start a server or anything that waits forever. Use `run_command` for that, if it is available.

[`references/LANGUAGES.md`](references/LANGUAGES.md) lists the runners for each language, and [`references/safety_sandbox.md`](references/safety_sandbox.md) has more on writing safe runs.
