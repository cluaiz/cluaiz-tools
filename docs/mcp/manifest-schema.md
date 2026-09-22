# 📋 MCP Package Specification (`package.json`)

This document defines the single-source package specification for MCP Connectors in the Cluaiz Hub ecosystem.

---

## 📄 Complete MCP `package.json`

```json
{
  "id": "everything",
  "name": "MCP Everything Server",
  "category": "mcp",
  "hub_type": "mcp",
  "logo": "/assets/mcp.svg",
  "title": "Model Context Protocol Reference Server",
  "description": "Official reference MCP server exercising full tool, resource, and prompt protocols.",
  "author": {
    "name": "MCP Steering Group",
    "url": "https://modelcontextprotocol.io"
  },
  "license": "MIT",
  "tags": [
    "MCP",
    "Reference",
    "Bridge"
  ],
  "dependencies": {
    "plugins": {
      "text": {
        "version": "^1.0.0",
        "url": "https://raw.githubusercontent.com/cluaiz/cluaiz-tools/main/plugins/text/package.json"
      }
    },
    "mcp": {
      "filesystem": {
        "version": "^1.0.0",
        "url": "https://raw.githubusercontent.com/cluaiz/cluaiz-tools/main/mcp/filesystem/package.json"
      }
    },
    "skills": {
      "learn-cluaiz": {
        "version": "^1.0.0",
        "url": "https://raw.githubusercontent.com/cluaiz/cluaiz-tools/main/skills/learn-cluaiz/package.json"
      }
    }
  },
  "latest_version": "1.0.0",
  "versions": {
    "1.0.0": {
      "updated_at": "2026-07-01T12:00:00Z",
      "command": "npx",
      "args": [
        "-y",
        "@modelcontextprotocol/server-everything"
      ],
      "files": {
        "icon": "/assets/icon.svg"
      }
    }
  }
}
```

---

## 🔍 Field Reference Table

| Field | Type | Required? | Description |
| :--- | :---: | :---: | :--- |
| `id` | `string` | **Yes** | Unique MCP bridge ID. |
| `name` | `string` | **Yes** | Human-readable title. |
| `hub_type` | `string` | **Yes** | Must be `"mcp"`. |
| `versions.*.command` | `string` | **Yes** | Subprocess executable to spawn (e.g. `npx`, `uvx`, `python`). |
| `versions.*.args` | `array` | **Yes** | Arguments passed to the subprocess. |
