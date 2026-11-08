# Reference Plugin package.json

Every WebAssembly or native plugin distributed through Cluaiz Tools contains a `package.json` manifest at its root. This manifest defines runtime sandbox bindings, categories, supported operating environments, and compiled artifacts.

---

## Complete Example Manifest

Below is the complete reference `package.json` for the official **Math Evaluator** WebAssembly plugin:

```json
{
  "id": "math",
  "name": "Math Evaluator",
  "category": "utility",
  "hub_type": "plugin",
  "build_type": "wasm",
  "icon": "assets/icon.svg",
  "title": "Deterministic Math Plugin",
  "description": "High-precision arithmetic, calculus, and mathematical expression evaluator for Cluaiz.",
  "author": {
    "name": "Aryan",
    "url": "https://github.com/cluaiz"
  },
  "license": "Apache-2.0",
  "tags": [
    "Math",
    "Calculus",
    "Precision",
    "WASM"
  ],
  "dependencies": {
    "plugins": {
      "text": {
        "version": "^1.0.0",
        "url": "https://raw.githubusercontent.com/cluaiz/cluaiz-tools/main/plugins/text/package.json"
      }
    },
    "mcp": {
      "fetch": {
        "version": "^1.0.0",
        "url": "https://raw.githubusercontent.com/cluaiz/cluaiz-tools/main/mcp/fetch/package.json"
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
      "builds_os": [
        "wasm"
      ],
      "files": {
        "binary": "logic.wasm",
        "icon": "/assets/icon.svg",
        "file_directory": "https://github.com/cluaiz/cluaiz-tools/releases/download/v1.0.0/math-files.zip"
      }
    }
  }
}
```

---

## Key Manifest Sections

### 1. Identity & Build Target
* **`id`**: Unique kebab-case slug for the plugin (e.g. `math`, `text`, `csv-parser`).
* **`hub_type`**: Must be set to `"plugin"`.
* **`build_type`**: `"wasm"` for isolated WebAssembly components or `"native"` for OS-specific host libraries.

### 2. Version & Files
Inside `versions[version]`:
* **`builds_os`**: Target architectures (`["wasm"]` for portable modules).
* **`files.binary`**: The compiled WebAssembly bytecode file (e.g. `logic.wasm`) executed inside the Cluaiz engine.
* **`files.file_directory`**: Distribution archive URL for release downloads.

### 3. Cross-Tool Dependencies
Plugins declare their runtime requirements across peer plugins, MCP servers, and skills within the `dependencies` block.

