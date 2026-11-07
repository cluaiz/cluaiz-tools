# 🛠️ Building, Testing & Publishing Cluaiz Extensions

> **Experimental.** Extensions may or may not ship. The Engine does not install or run them, and no other tool depends on them. This page describes a design, not a working feature.

This guide provides a comprehensive, step-by-step tutorial for third-party developers creating autonomous, decoupled extensions for the Cluaiz ecosystem.

---

## 🎯 What You Will Build

You will create a fully compliant Cluaiz Extension that:
1. Runs inside the port-free sandboxed webview (`cluaiz-ext://`).
2. Interacts with the local AI engine via `window.Cluaiz`.
3. Displays a crisp vector icon in the App Launcher and Extension Store.
4. Packages into a standalone GitHub Release without bloating the host repository.

---

## 📁 1. Project Directory Layout

Initialize a new directory for your extension:

```text
my-custom-extension/
├── package.json               # Manifest & Cluaiz configuration
├── tsconfig.json              # TypeScript compilation rules
├── assets/
│   ├── icon.svg               # REQUIRED: 1:1 Vector logo (<50 KB)
│   └── banner.png             # (Optional) Store preview banner
├── src/
│   ├── index.ts               # Core logic & UI mounting hook
│   └── styles.css             # Component styling
└── dist/                      # Compiled production assets
    ├── index.html             # Sandboxed webview entrypoint
    └── bundle.js              # Minified JavaScript bundle
```

---

## 📋 2. Create the Manifest (`package.json`)

Create `package.json` in your project root:

```json
{
  "id": "my-custom-extension",
  "name": "Data Visualizer Companion",
  "category": "utility",
  "hub_type": "extension",
  "build_type": "typescript",
  "icon": "assets/icon.svg",
  "description": "Interactive data visualization companion for Cluaiz.",
  "author": {
    "name": "Developer Name",
    "url": "https://github.com/developer-handle"
  },
  "license": "MIT",
  "tags": [
    "Extension",
    "Data",
    "Visualization"
  ],
  "dependencies": {
    "plugins": {},
    "mcp": {},
    "skills": {},
    "extensions": {}
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
          "assets/banner.png"
        ],
        "file_directory": "https://github.com/developer-handle/my-custom-extension/releases/download/v1.0.0/extension.zip"
      },
      "execution": {
        "modes": [
          "foreground",
          "headless"
        ],
        "run_in_background": false,
        "permissions": [
          "canvas",
          "storage"
        ]
      }
    }
  }
}
```

---

## 🎨 3. Design the Visual Icon (`assets/icon.svg`)

Every extension must supply an SVG icon:
- **Format:** Strict Vector SVG (`.svg`).
- **Aspect Ratio:** Square `1:1` (e.g. `viewBox="0 0 64 64"`).
- **Size Constraint:** File size MUST be less than 50 KB.
- **Location:** Place at `assets/icon.svg`.

```xml
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64" width="64" height="64">
  <rect width="64" height="64" rx="16" fill="#1e1e2e"/>
  <circle cx="32" cy="32" r="18" fill="none" stroke="#89b4fa" stroke-width="4"/>
  <path d="M24 32h16M32 24v16" stroke="#cdd6f4" stroke-width="3" stroke-linecap="round"/>
</svg>
```

---

## 💻 4. Implement Frontend & Global Bridge

In `src/index.ts`, use the injected `window.Cluaiz` bridge to talk to the local inference engine:

```typescript
// Define bridge interface for strong typing
interface CluaizWindow extends Window {
  Cluaiz?: {
    ai: {
      prompt: (req: { messages: Array<{ role: string; content: string }> }) => Promise<{ text: string }>;
    };
    storage: {
      get: (key: string) => Promise<string | null>;
      set: (key: string, val: string) => Promise<void>;
    };
    events: {
      emit: (event: string, data: unknown) => void;
    };
  };
}

declare const window: CluaizWindow;

export async function initializeExtension() {
  console.log("Initializing My Custom Extension...");

  // Verify Cluaiz bridge presence
  if (!window.Cluaiz) {
    console.error("Cluaiz bridge not detected. Ensure extension runs inside Cluaiz App Shell.");
    return;
  }

  // Example: Query local inference model
  const response = await window.Cluaiz.ai.prompt({
    messages: [
      { role: "user", content: "Generate a summary of active database metrics." }
    ]
  });

  const outputContainer = document.getElementById("output");
  if (outputContainer) {
    outputContainer.textContent = response.text;
  }
}

// Auto-boot when DOM is ready
document.addEventListener("DOMContentLoaded", initializeExtension);
```

---

## 🧪 5. Local Testing in Cluaiz App

You do NOT need to publish your extension to test it:

1. Build your distribution files:
   ```bash
   npm run build
   ```
2. Locate your local Cluaiz data directory:
   - **Windows:** `%USERPROFILE%\.cluaiz\extensions\`
   - **macOS / Linux:** `~/.cluaiz/extensions/`
3. Create a symbolic link or copy your project folder into the extensions directory:
   ```bash
   # Windows (PowerShell Administrator):
   New-Item -ItemType SymbolicLink -Path "$HOME\.cluaiz\extensions\my-custom-extension" -Target "C:\path\to\my-custom-extension"

   # Linux / macOS:
   ln -s /path/to/my-custom-extension ~/.cluaiz/extensions/my-custom-extension
   ```
4. Restart or reload the Cluaiz App Shell. Your extension will immediately appear in the **Installed Extensions Drawer** with its `assets/icon.svg`.

---

## 📦 6. Packaging & Releasing on GitHub

When ready to publish:

1. Create a production ZIP archive containing ONLY runtime assets:
   ```bash
   # Ensure dist/ and assets/ and package.json are in the archive root:
   zip -r extension.zip package.json dist/ assets/ README.md
   ```
2. Compute the cryptographic SHA-256 hash of `extension.zip`:
   ```bash
   # Windows:
   Get-FileHash -Algorithm SHA256 .\extension.zip

   # Linux / macOS:
   sha256sum extension.zip
   ```
3. Create a GitHub Release (e.g. `v1.0.0`) in your repository and attach `extension.zip`.

---

## 🚀 7. Submitting to the Cluaiz Catalog

To make your extension discoverable in the Cluaiz Store:

1. Fork the `cluaiz-tools` repository.
2. Edit `extensions/family.json` to append your remote pointer:
   ```json
   {
     "category": "extensions",
     "name": "Cluaiz Extensions",
     "items": {
       "my-custom-extension": {
         "type": "remote",
         "repo": "developer-handle/my-custom-extension",
         "branch": "main",
         "manifest_path": "package.json"
       }
     }
   }
   ```
3. Submit a Pull Request. Once merged, users worldwide can install your extension with a single click.
