# Reference MCP package.json

Every Model Context Protocol (MCP) server package distributed through Cluaiz Tools contains a `package.json` manifest at its root. This manifest defines runtime execution parameters, categories, tags, and cross-package dependencies.

---

## Complete Example Manifest

Below is the complete reference `package.json` for the official reference **MCP Everything Server**:

```json
{
  "id": "everything",
  "name": "MCP Everything Server",
  "category": "mcp",
  "hub_type": "mcp",
  "icon": "assets/icon.svg",
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

## Key Manifest Sections

### 1. Identity & Metadata
* **`id`**: Unique kebab-case slug for the server package (e.g. `everything`, `filesystem`, `github`).
* **`hub_type`**: Set to `"mcp"` to indicate an external Model Context Protocol server adapter.
* **`category`**: Primary grouping in the Cluaiz catalog.
* **`icon`**: Path to the 24x24 SVG package icon.

### 2. Execution Specification
Inside `versions[version]`:
* **`command`**: Executable binary to launch (e.g. `npx`, `python`, `node`).
* **`args`**: Array of command-line arguments passed to the server process upon initialization.
* **`files`**: Static assets bundled with the package.

### 3. Cross-Ecosystem Dependencies
MCP servers can declare peer dependencies on Cluaiz plugins, skills, or other MCP servers in the `dependencies` block. The Cluaiz runtime resolves and links these during session startup.

