---
title: "Testing MCP servers locally"
description: "Test a server by itself, then in the Engine."
category: "Tutorials"
---

# 4. Testing MCP servers locally

## Step 1: The server alone

An MCP server is a normal command-line program. Run the exact command from `execution`:

```bash
npx -y @modelcontextprotocol/server-everything
```

It waits for input, which is expected. Send an `initialize` request followed by `tools/list`, each on one line:

```json
{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"test","version":"1"}}}
{"jsonrpc":"2.0","method":"notifications/initialized"}
{"jsonrpc":"2.0","id":2,"method":"tools/list"}
```

You should see JSON answers on standard output. Logs must go to standard error, because anything else on standard output breaks the protocol.

## Step 2: In the Engine

Use a test home so your real installation is untouched, and place the package by hand, as [the tutorial](../../mcp/tutorial-connect-mcp.md) shows. Then:

1. Run `cluaiz mcp list` and see it listed.
2. Call it through CEP `tools.call`.
3. Check how it is found, through `tools.discover`.

## Common errors

| Symptom | Cause |
|---|---|
| `Failed to spawn MCP server` | The command is not on `PATH`. Check Node or `uv` |
| `No execution configuration` or an empty command | `execution.command` is missing |
| A very slow first call | `npx` or `uvx` is downloading the package |
| It reads the wrong folder | It runs in the open workspace, not in its package folder |
| The tool never appears as a card | Add `triggers` as a plain list. Function names help only after the first start |
