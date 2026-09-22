# 🔌 Cluaiz Tools: Plugins (Unified Tool Ecosystem)

Welcome to the **Plugins** directory of the Cluaiz Tools.

## What is a Plugin?
A Plugin represents the execution layer of the Cluaiz Ecosystem. It encompasses both:
1. **Sandboxed Tools (`WASM`):** Lightweight, fully isolated WebAssembly binaries executing with strict CPU fuel limits and RAM caps.
2. **Native C-FFI Plugins (`NATIVE`):** High-performance bare-metal C-FFI binaries (`.dll`, `.so`, `.dylib`) for deep hardware, VRAM, and database subsystems.

## Anatomy of a Plugin
Every folder in this directory follows this standard structure:
```text
cluaiz-search/               <-- Plugin Name
├── manifest-plugin.yaml     <-- Capabilities, envelope (WASM/NATIVE), memory limits, and permissions
├── SKILL.md                 <-- [Optional Brain] Teaches the AI how and when to use this plugin
├── package.json             <-- Versioning, OS binaries, and metadata
└── src/ (or native/)        <-- Source code (Rust/C++)
```

## How It Works
1. When installed (`cluaiz plugin install <name>`), the Engine downloads and places the package into `~/.cluaiz/plugins/<name>`.
2. If `SKILL.md` is present, the Engine injects the tool prompt into the AI's context.
3. The AI outputs a CEL query (e.g., `use plugin::cluaiz-search { "query": "..." }`).
4. The Engine executes the plugin in its declared envelope (`WASM` sandbox or `NATIVE` C-FFI) with sub-millisecond latency.
