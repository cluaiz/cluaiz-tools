# 📋 Plugin Package Specification (`package.json`)

This document defines the single-source package specification for Plugins in the Cluaiz Hub ecosystem.

---

## 📄 Complete Plugin `package.json`

```json
{
  "id": "math",
  "name": "Math Evaluator",
  "category": "utility",
  "hub_type": "plugin",
  "build_type": "wasm",
  "logo": "/assets/math.svg",
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

## 🔍 Field Reference Table

| Field | Type | Required? | Description |
| :--- | :---: | :---: | :--- |
| `id` | `string` | **Yes** | Unique plugin ID in lowercase kebab-case. |
| `name` | `string` | **Yes** | Human-readable name. |
| `category` | `string` | **Yes** | Categorization group (`utility`, `research`, `engineering`). |
| `hub_type` | `string` | **Yes** | Must be `"plugin"`. |
| `build_type` | `string` | **Yes** | `"wasm"` for WebAssembly or `"binary"` for native C-FFI. |
| `dependencies` | `object` | **Yes** | Transitive plugins, MCPs, and skills with SemVer ranges and package URLs. |
| `versions` | `object` | **Yes** | Map of version release objects with download URLs and file bundles. |
