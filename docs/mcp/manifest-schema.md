# MCP package fields

All fields are defined in the [package specification](../package-spec.md). This page lists what matters for an MCP server.

## Example

```json
{
  "id": "filesystem",
  "name": "Filesystem",
  "category": "mcp",
  "hub_type": "mcp",
  "icon": "assets/icon.svg",
  "description": "Reads, writes and searches files in the workspace. Needs Node.js.",
  "license": "MIT",
  "triggers": ["read file", "list files", "search files"],
  "execution": {
    "command": "npx",
    "args": ["-y", "@modelcontextprotocol/server-filesystem", "."],
    "env": {}
  },
  "dependencies": { "plugins": {}, "mcp": {}, "skills": {} },
  "latest_version": "1.0.0",
  "versions": {
    "1.0.0": {
      "updated_at": "2026-11-06T12:00:00Z",
      "files": { "icon": "assets/icon.svg" }
    }
  }
}
```

## Fields an MCP server must get right

| Field | Why it matters |
|---|---|
| `id` | Must equal the folder and the family key |
| `execution.command` | The program to start. It must be on the user's `PATH`, or a full path |
| `execution.args` | Arguments, as a list |
| `execution.env` | Environment values, used as written. `${NAME}` is not expanded |
| `description` | Say what it does and what it needs, such as Node.js |
| `triggers` | A plain list. A `{ "semantic": [...] }` object has no effect here |

Older manifests put `command` and `args` under `versions.<v>`. The Engine still reads them when `execution` is missing. Use `execution`.

Leave out `functions`: the Engine gets the tool list from the server.

## The same server in other agents

Other agents describe a server like this:

```json
{ "mcpServers": { "filesystem": { "command": "npx", "args": ["-y", "@modelcontextprotocol/server-filesystem", "."] } } }
```

`command`, `args` and `env` carry over into `execution` unchanged.

## Secrets

The Engine has no way yet to fill a secret into `env` from the user's environment. Do not put a secret in a published manifest. A server that reads its key from the environment gets what the Engine's own process has.
