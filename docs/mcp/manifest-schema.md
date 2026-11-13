# MCP package fields

All fields are defined in the [package specification](../package-spec.md). This page lists what matters for an MCP server. A complete, real manifest with permissions, a setting and `expose` is explained in [MCP server package.json](package-json.md).

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
    "args": ["-y", "@modelcontextprotocol/server-filesystem@2.2.0", "."]
  },
  "dependencies": { "plugins": {}, "mcp": {}, "skills": {} },
  "permissions": {
    "files": { "info": "Reads, searches, creates, changes, moves and copies files in the open folder", "allow": false, "can": ["create", "read", "update", "delete"], "where": "workspace" }
  },
  "latest_version": "1.0.0",
  "versions": {
    "1.0.0": {
      "updated_at": "2026-11-06T12:00:00Z",
      "files": { "readme": "README.md", "skill": "SKILL.md", "icon": "assets/icon.svg" }
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
| `execution.args` pins | Pin the package version, such as `server-filesystem@2.2.0`, never `@latest` |
| `execution.env` | Fixed environment values, used as written. `${NAME}` is not expanded |
| `env` | Settings the user gives, keys among them. The server gets each as an environment variable |
| `permissions` | Every kind the server uses. The Engine cannot block a server, so a refused permission means asking before each call |
| `versions.<v>.files` | Every file to install. An MCP package has no release zip |
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

Do not put a secret in a published manifest, and not in `execution.env`, whose values are literal. Name each key the server reads in the top-level `env`, with `info` and `need`. The Engine gives the server the value as an environment variable: the system environment variable of that name first, otherwise the value the user set with `cluaiz mcp config <id> <NAME>`. See [Env](../package-spec.md#env).
