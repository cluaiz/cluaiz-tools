# 🌐 Cluaiz Extensions Architecture

> **Experimental.** Extensions may or may not ship. The Engine does not install or run them, and no other tool depends on them. This page describes a design, not a working feature.

This document defines the architectural specification, execution boundary, and lifecycle of **Extensions** within the Cluaiz ecosystem.

---

## 🧭 1. What is an Extension?

In the Cluaiz ecosystem, an **Extension** represents a **Self-Contained Composite Micro-Application**. While Skills shape cognition and Plugins/MCP provide backend compute, Extensions grant the user and client interface external superpowers without imposing computational overhead on the core inference engine.

An extension is NOT merely a frontend script. It is an autonomous bundle that can package:
1. **Foreground UI:** Sandboxed interactive view rendered via custom protocol (`cluaiz-ext://`).
2. **Headless Background Service:** Silent event-driven worker or scheduler.
3. **Bundled Capabilities:** Extension-specific Skills, MCP tool servers, and Plugins that auto-register into the global Cluaiz toolbelt.

```
┌────────────────────────────────────────────────────────────────────────┐
│                        COMPOSITE EXTENSION ANATOMY                     │
└────────────────────────────────────────────────────────────────────────┘
                                    │
        ┌───────────────────────────┼───────────────────────────┐
        ▼                           ▼                           ▼
┌───────────────┐           ┌───────────────┐           ┌───────────────┐
│ FOREGROUND UI │           │HEADLESS WORKER│           │ BUNDLED TOOLS │
│ (Activity)    │           │ (Daemon)      │           │(Capabilities) │
├───────────────┤           ├───────────────┤           ├───────────────┤
│ • HTML5/Canvas│           │ • Wasm Isolate│           │ • MCP Servers │
│ • Mounts on   │           │ • Background  │           │ • Domain      │
│   user click  │           │   cron/events │           │   Skills      │
│ • RAM: 2–6 MB │           │ • RAM: 1–4 MB │           │ • Rule Plugins│
└───────────────┘           └───────────────┘           └───────────────┘
```

---

## ⚡ 2. The Four Sovereign Pillars Compared

| Architectural Dimension | 🧠 1. Skills | ⚡ 2. Plugins | 🔌 3. MCP Servers | 🌐 4. Extensions |
| :--- | :--- | :--- | :--- | :--- |
| **Execution Medium** | LLM Context Window (KV-Cache) | In-Process WASM Sandbox (Wasmtime) | External OS Child Process (Stdio IPC) | Sandboxed Webview (`cluaiz-ext://`) + Headless Daemon |
| **Primary Domain** | Reasoning patterns, guidelines, runbooks | Deterministic computation, parsing, algorithms | Filesystem, Git, database integrations | Audio input/output, interactive UI widgets, companions, micro-apps |
| **VRAM / RAM Cost** | Context tokens (~500–2,000 tokens) | ~2MB RAM (0 MB VRAM) | Subprocess RAM (10–50MB, 0 MB VRAM) | **0 MB VRAM / 4–15 MB RAM target** |
| **Package Manifest** | `package.json` (`hub_type: "skill"`) | `package.json` (`hub_type: "plugin"`) | `package.json` (`hub_type: "mcp"`) | `package.json` (`hub_type: "extension"`) |
| **Target Audience** | The LLM Cognitive Engine | The Inference Engine Pipeline | The Agentic Tool Orchestrator | The End-User & Client Experience |

---

## 🪶 3. Anti-Bloat Law & Memory Budget (Strict 4–15 MB)

To prevent desktop bloatware and resource starvation, Cluaiz enforces strict memory ceilings across all extensions:

```
┌────────────────────────────────────────────────────────┐
│               EXTENSION MEMORY BUDGET                  │
├───────────────────────────────────┬────────────────────┤
│ Component Type                    │ RAM Allocation     │
├───────────────────────────────────┼────────────────────┤
│ In-Webview UI (TypeScript/Canvas) │ 2 MB – 6 MB        │
│ Headless Daemon (Wasm / Pure TS)  │ 1 MB – 4 MB        │
│ Total Combined Footprint          │ 4 MB – 15 MB Max   │
├───────────────────────────────────┴────────────────────┤
│ ❌ BANNED: Spawning separate Node.js or Python runtime  │
│    processes per extension (exceeds 30–80 MB budget).   │
└────────────────────────────────────────────────────────┘
```

---

## 🔌 4. The `window.Cluaiz` Global Bridge

Extensions execute inside a port-free sandboxed webview (`cluaiz-ext://<id>/`). The App Shell automatically injects a secure, capability-gated communication bridge:

```typescript
interface CluaizBridge {
  // 1. Local AI Inference API
  ai: {
    prompt(request: {
      model?: string;
      messages: Array<{ role: "user" | "assistant" | "system"; content: string }>;
      temperature?: number;
    }): Promise<{ text: string; tokens_evaluated: number }>;
  };

  // 2. Global Tool & MCP Execution
  mcp: {
    callTool(request: {
      server: string;
      tool: string;
      args: Record<string, unknown>;
    }): Promise<unknown>;
  };

  // 3. Persistent Local Storage
  storage: {
    get(key: string): Promise<string | null>;
    set(key: string, value: string): Promise<void>;
    remove(key: string): Promise<void>;
  };

  // 4. Cross-Extension Event Bus
  events: {
    on(event: string, handler: (payload: unknown) => void): void;
    emit(event: string, payload: unknown): void;
  };
}

// Injected into the global window context:
declare const Cluaiz: CluaizBridge;
```

---

## 🌙 5. App Lifecycle & Background Daemon Persistence

When the end-user closes the main application window (clicks "X"), the system transitions to background mode:

```
[User Clicks "X" to Close Window]
              │
              ▼
   Does any extension require background persistence?
   (Declared via `run_in_background: true`)
              │
        ┌─────┴─────────────────────────────────┐
        ▼                                       ▼
      [YES]                                   [NO]
App window hides to System Tray          App process terminates
React UI unmounts (Frees ~100MB RAM)     cleanly (0 MB RAM)
Lightweight Rust core & daemons
remain alive in tray (~8–12 MB total)
```

---

## 📦 6. Extension Directory Structure

An Extension package adheres to the standard Cluaiz Tools directory layout:

```text
cluaiz-tools/extensions/browser-voice/
├── package.json          # Manifest declaring hub_type: "extension" & cluaiz block
├── README.md             # Extension documentation and usage guide
├── assets/
│   ├── icon.svg          # Visual logo for App Launcher & Store (Vector SVG)
│   └── preview.png       # Store card preview banner
├── dist/
│   ├── index.html        # UI entrypoint mounted in sandboxed webview
│   └── bundle.js         # Minified frontend code
├── src/
│   ├── index.ts          # Core client hook and event bindings
│   └── engines/          # Audio / logic components
└── skills/               # (Optional) Bundled cognitive skills
    └── voice-commands/
        └── SKILL.md
```

