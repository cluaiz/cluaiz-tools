---
title: "Settings in MCP servers"
description: "How an MCP server gets configuration from the environment."
category: "Tutorials"
---

# 2. Settings in MCP servers

An MCP server reads its settings from its environment, as any program does. A `package.json` has two places for them.

| Place | For | Reaches the server |
|---|---|---|
| `execution.env` | Fixed values that are the same for every user, such as `MODE=fast` | As written. `${NAME}` is not expanded |
| Top-level `env` | Values each user gives, such as a key or a connection string | As an environment variable, from the system environment or `cluaiz mcp config` |
| `execution.args` | Fixed command-line values | As written |

```json
"execution": {
  "command": "npx",
  "args": ["-y", "@upstash/context7-mcp@4.5.1"]
},
"env": {
  "CONTEXT7_API_KEY": { "info": "A free Context7 key, for a higher rate limit", "need": "optional" }
}
```

## Where a value comes from

1. The system environment variable of that name, if it is set.
2. Otherwise the value the user saved: `cluaiz mcp config context7 CONTEXT7_API_KEY` (asked for hidden when the value is left out), the App, or CEP `tools.set_settings` with `user_settings`.

The value is kept in the Engine's `config/tool_settings.json`, not in the server's folder. A change applies the next time the server starts. A missing `required` value is asked for at install.

## Secrets

Never put a real key in a published `package.json`, and never in `execution.env`. Name it in `env`, and say in `SKILL.md` what the model should tell the user when it is missing.

## What does not exist

| Described in older pages | Today |
|---|---|
| `${GITHUB_TOKEN}` expansion in `execution.env` | Not supported. Use the top-level `env` |
| `user_env` | Replaced by the top-level `env` |
| `system_bindings`, `SYSTEM_BOOSTER__*`, `SETTINGS__*` | Not available |
