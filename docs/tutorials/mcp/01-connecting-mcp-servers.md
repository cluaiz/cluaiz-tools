---
title: "Connecting MCP Servers"
description: "How to integrate external Model Context Protocol (MCP) servers with the cluaiz Engine via stdio."
category: "Tutorials"
---

# 1. Connect Model Context Protocol (MCP) Servers

The **Model Context Protocol (MCP)** is a standardized way for AI models to securely connect to external data sources and tools (e.g., GitHub, Slack, local file systems).

Unlike in-memory Plugins (Native C-FFI or WASM) which run directly within the cluaiz Engine execution pipeline, MCP servers are **separate processes** (e.g., Node.js or Python scripts).

> [!TIP]
> **For more details, reference:** [`skill_architecture.md`](file:///c:/Users/Aryan/my/Cluaiz-workspace/Cluaiz-Technologies/cluaiz-tools/docs/architecture/skill_architecture.md)
> 
> For **MCP Servers**, creating a `SKILL.md` file is **OPTIONAL but RECOMMENDED**. Providing a `SKILL.md` teaches the AI exactly how to structure the JSON-RPC arguments for the MCP tool and gives the AI context on why it should call the tool.

---

## The MCP Architecture

Because MCP servers run externally, the Engine spawns them as background processes and communicates via standard input/output (`stdio`) streams using JSON-RPC.

```mermaid
flowchart LR
    A["cluaiz Engine"] -->|stdio JSON-RPC| B["MCP Process Manager"]
    B -->|Spawns (npx / python)| C["MCP Server Process"]
    C -->|Reads External APIs| D["GitHub / Slack"]
    D -->|Returns JSON-RPC| B
    B -->|Context Injection| A
```

## Step 1: Defining the MCP Package Specification (`package.json`)

To register an external MCP server with Cluaiz, create a `package.json`. Notice how `versions` defines `command` and `args`:

```json
{
  "id": "github-mcp-connector",
  "name": "GitHub MCP Connector",
  "category": "mcp",
  "hub_type": "mcp",
  "logo": "/assets/icon.svg",
  "title": "Official GitHub MCP Server",
  "description": "Connects the engine to GitHub via official MCP server.",
  "author": {
    "name": "Cluaiz Community",
    "url": "https://github.com/cluaiz"
  },
  "license": "MIT",
  "dependencies": {
    "plugins": {},
    "mcp": {},
    "skills": {}
  },
  "latest_version": "1.0.0",
  "versions": {
    "1.0.0": {
      "updated_at": "2026-07-01T12:00:00Z",
      "command": "npx",
      "args": [
        "-y",
        "@modelcontextprotocol/server-github"
      ],
      "env": {
        "GITHUB_PERSONAL_ACCESS_TOKEN": "${GITHUB_TOKEN}"
      },
      "files": {
        "icon": "/assets/icon.svg"
      }
    }
  }
}
```

> [!WARNING]
> Ensure that you do not expose sensitive API keys in plaintext in the `package.json` file. Always use the `${ENV_VAR}` syntax to resolve secrets from the host system environment dynamically.
