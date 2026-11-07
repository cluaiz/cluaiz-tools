---
title: "Settings in MCP servers"
description: "How an MCP server gets configuration from the environment."
category: "Tutorials"
---

# 2. Settings in MCP servers

An MCP server reads its settings from its environment, as any program does.

## What works

| Source | Reaches the server | Status |
|---|---|---|
| `execution.env` in `package.json` | Yes, used as written | **Works** |
| The environment of the Engine's process | Yes, it is inherited | **Works** |
| Command-line values in `execution.args` | Yes | **Works** |
| `cluaiz mcp config` | Edits the installed `package.json` | **Works** |

```json
"execution": {
  "command": "npx",
  "args": ["-y", "some-server"],
  "env": { "MODE": "fast" }
}
```

## What does not exist

| Described in older pages | Today |
|---|---|
| `${GITHUB_TOKEN}` expansion in `env` | **Planned.** It is passed as the literal text |
| `system_bindings` | Not available |
| `SYSTEM_BOOSTER__*` and `SETTINGS__*` variables | Not available |

## Secrets

Never put a real key in a published `package.json`. Until expansion exists, a server that needs a key can read it from the environment, and the user sets that variable before starting the Engine:

```bash
export GITHUB_PERSONAL_ACCESS_TOKEN="..."
cluaiz serve
```

Say which variable is needed in the description and in `SKILL.md`.
