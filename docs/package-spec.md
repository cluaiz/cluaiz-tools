# Cluaiz Hub Package Specification (`package.json`)

This document defines the official package manifest specification for the Cluaiz Hub ecosystem. It serves as the single source of truth for both the cloud registry/installer (`cluaiz install`) and the local engine runtime (`inference-cel`).

---

## 1. Top-Level Identity & Distribution

| Field | Type | Description |
| :--- | :---: | :--- |
| `id` | `string` | Unique package identifier in the ecosystem (e.g. `cluaiz-search`, `code-reviewer`). |
| `name` | `string` | Display name of the package. |
| `category` | `string` | Categorization group (`research`, `engineering`, `utility`, `database`). |
| `hub_type` | `string` | Component type: `"plugin"`, `"skill"`, or `"mcp"`. |
| `author` | `object` | Author name and optional URL/GitHub handle. |
| `license` | `string` | Open-source license identifier (e.g. `"Apache-2.0"`, `"MIT"`). |
| `latest_version` | `string` | The active semantic version string (e.g. `"0.1.1"`). |

---

## 2. Dependency Resolution (`dependencies`)

Cluaiz Hub uses **SemVer caret ranges (`^`) with remote metadata URLs** to achieve safe, decentralized dependency resolution:

```json
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
}
```

### SemVer Operators Specification:

| Symbol | Syntax Example | Meaning (Range) | Resolution Behavior |
| :--- | :---: | :--- | :--- |
| **`^` (Caret) [Recommended]** | `"^1.2.3"` | `>= 1.2.3 < 2.0.0` | Allows backward-compatible bug fixes (`1.2.4`) and minor features (`1.3.0`). Blocks breaking major releases (`2.0.0`). |
| **`~` (Tilde)** | `"~1.2.3"` | `>= 1.2.3 < 1.3.0` | Allows only patch-level bug fixes (`1.2.4`). Blocks minor feature releases (`1.3.0`). |
| **Exact Pinning** | `"1.2.3"` | `== 1.2.3` | Strictly binds to the exact version string. No updates permitted. |
| **Greater Than** | `">= 1.2.0"` | `>= 1.2.0` | Accepts version 1.2.0 or any higher version available. |
| **Wildcard** | `"*"` or `"latest"` | `All versions` | Always resolves to the newest release published on the remote URL. |

### Lockfile Guarantee (`cluaiz-lock.json`):
When `cluaiz install` executes, the resolved version, download URL, and SHA-256 integrity hash are frozen into `cluaiz-lock.json` to guarantee 100% reproducible environments across developer machines and production nodes.

---

## 3. Distribution Matrix (`versions`)

Defines precompiled platform binaries and source assets:

```json
"versions": {
  "0.1.1": {
    "updated_at": "2026-07-01T15:59:00Z",
    "builds_os": ["windows", "macos", "linux"],
    "os": {
      "windows": "https://github.com/.../cluaiz-search_windows_x64.dll",
      "macos": "https://github.com/.../libcluaiz-search_macos_arm64.dylib",
      "linux": "https://github.com/.../libcluaiz-search_linux_x64.so"
    },
    "files": {
      "skill": "/SKILL.md",
      "file_directory": "https://github.com/.../cluaiz-search-files.zip"
    }
  }
}
```

---
