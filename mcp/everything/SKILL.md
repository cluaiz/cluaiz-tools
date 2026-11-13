---
name: everything
description: "The official MCP test server, for checking that MCP tools work. Use only when the user wants to test MCP."
---

# MCP Test Server

Use this only when the user wants to check that MCP works. It does no useful work of its own.

- `echo` with `message` returns the same text.
- `get-sum` with `a` and `b` returns their sum.

If these answer correctly, MCP works. Report what came back. For real arithmetic, use `math_evaluate` if it is installed.
