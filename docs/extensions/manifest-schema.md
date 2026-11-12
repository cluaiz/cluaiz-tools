# 📋 Extension Manifest Specification (`package.json`)

> **Experimental.** Extensions may or may not ship. The Engine does not install or run them, and no other tool depends on them. This page describes a design, not a working feature.

This document defines the formal schema and configuration fields for **Extensions** within the Cluaiz ecosystem, adhering to the unified manifest standard across all four tool pillars.

---

## 📄 1. Canonical Manifest Example

```json
{
  "id": "my-assistant",
  "name": "My Assistant Companion",
  "category": "companion",
  "hub_type": "extension",
  "build_type": "typescript",
  "icon": "assets/icon.svg",
  "description": "Zero-VRAM interactive canvas desktop companion with lip-sync, expression engine, and drag physics.",
  "author": {
    "name": "Aryan",
    "url": "https://github.com/cluaiz"
  },
  "license": "Apache-2.0",
  "tags": [
    "Companion",
    "Canvas",
    "Voice-Sync",
    "Zero-VRAM"
  ],
  "dependencies": {
    "plugins": {},
    "mcp": {},
    "skills": {
      "learn-cluaiz": {
        "version": "^1.0.0",
        "url": "https://raw.githubusercontent.com/cluaiz/cluaiz-tools/main/skills/learn-cluaiz/package.json"
      }
    },
    "extensions": {
      "browser-voice": {
        "version": "^1.0.0",
        "url": "https://raw.githubusercontent.com/cluaiz/cluaiz-tools/main/extensions/browser-voice/package.json"
      }
    }
  },
  "latest_version": "1.0.0",
  "versions": {
    "1.0.0": {
      "updated_at": "2026-09-26T12:00:00Z",
      "builds_os": [
        "all"
      ],
      "files": {
        "entry": "dist/index.html",
        "icon": "assets/icon.svg",
        "assets": [
          "assets/icon.svg",
          "assets/banner.png",
          "assets/preview1.png"
        ],
        "file_directory": "https://github.com/cluaiz/cluaiz-tools/releases/download/v1.0.0/my-assistant.zip"
      },
      "execution": {
        "modes": [
          "foreground",
          "headless"
        ],
        "run_in_background": true,
        "permissions": [
          "canvas",
          "audio-sync",
          "storage"
        ]
      }
    }
  }
}
```

---

## 🏷️ 2. Top-Level Field Reference

| Field | Type | Required | Description |
| :--- | :---: | :---: | :--- |
| `id` | `string` | **Yes** | Unique package identifier (e.g. `my-assistant`, `browser-voice`). |
| `name` | `string` | **Yes** | Human-readable display title. |
| `category` | `string` | **Yes** | Classification group (`audio`, `companion`, `utility`, `developer-tools`). |
| `hub_type` | `string` | **Yes** | Must be strictly set to `"extension"`. |
| `build_type` | `string` | **Yes** | Build target format (`"typescript"`, `"wasm"`, `"html"`). |
| `icon` | `string` | **Yes** | Relative path to vector icon (e.g. `"assets/icon.svg"`). |
| `description` | `string` | **Yes** | Clear explanation of capabilities and functionality. |
| `author` | `object` | **Yes** | Author details (`name`, `url`). |
| `license` | `string` | **Yes** | Open-source license identifier (e.g. `"Apache-2.0"`, `"MIT"`). |
| `tags` | `array[string]`| **Yes** | Discovery keyword tags for Store search. |

---

## 🔗 3. Dependency Declaration (`dependencies`)

The `dependencies` dictionary declares required capabilities from any of the four ecosystem pillars:

```json
"dependencies": {
  "plugins": {
    "<plugin-id>": { "version": "^1.0.0", "url": "https://..." }
  },
  "mcp": {
    "<mcp-id>": { "version": "^1.0.0", "url": "https://..." }
  },
  "skills": {
    "<skill-id>": { "version": "^1.0.0", "url": "https://..." }
  },
  "extensions": {
    "<extension-id>": { "version": "^1.0.0", "url": "https://..." }
  }
}
```

Every entry has this form: the key is the tool id, `version` is a version range and `url` is the raw `package.json` of that tool in `cluaiz-tools`. A bare string such as `"plugins/web-search"` is not valid, and `validate_tool` reports it.

**Status:** installing a tool installs the plugins, MCP servers and skills it lists that are missing; version ranges are not checked yet, and `extensions` entries are not installed. See [Package specification](../package-spec.md#dependencies).

---

## 📦 4. Versioning & Asset Structure (`versions`)

Every release is tracked under the `versions` map keyed by semantic version (e.g. `"1.0.0"`):

### `files` Object:
- `entry` (`string`): The client UI HTML bundle loaded inside the sandboxed webview (`cluaiz-ext://<id>/`).
- `icon` (`string`): Path to the primary vector SVG logo (`"assets/icon.svg"`).
- `assets` (`array[string]`): Visual preview assets, screenshots, and template files.
- `file_directory` (`string`): Remote download URL for the compiled release ZIP archive.

### `execution` Object (Extension Specific):
- `modes` (`array[string]`):
  - `"foreground"`: Renders UI in the App Launcher or side panel.
  - `"headless"`: Executes silently as an event listener or background daemon without mounting DOM elements.
- `run_in_background` (`boolean`): If `true`, the background worker persists in the System Tray when the main window is closed.
- `permissions` (`array[string]`): Capability tokens requested by the extension:
  - `"canvas"`: 60fps HTML5 Canvas rendering.
  - `"audio-sync"`: Lip-sync and audio waveform synchronization.
  - `"microphone"`: Web Speech STT access.
  - `"audio-output"`: SpeechSynthesis TTS access.
  - `"storage"`: Local key-value cache access via `window.Cluaiz.storage`.
