# Cluaiz Hub Documentation Portal

Welcome to the official developer documentation for the **Cluaiz Hub Ecosystem** and the **Cluaiz Inference Engine**.

---

## 🗺️ Documentation Sitemap & Navigation

```
cluaiz-tools/docs/
├── 🏗️ 1. architecture/      ← Full System Architecture & Engine Integration
├── 🧠 2. skills/            ← Cognitive Brain, Prompt Frameworks, & Context Injection
├── ⚡ 3. plugins/           ← In-Process WASM Execution Muscle, C-ABI & Sandboxing
├── 🔌 4. mcp/               ← External Subprocess Bridges & Stdio JSON-RPC 2.0
└── 📚 5. reference/         ← CLI Command Reference & Hub Registry Packaging
```

---

## 🧭 Section Directory

### 🏗️ 1. Architecture (`docs/architecture/`)
- [System Overview](file:///c:/Users/Aryan/my/Cluaiz-workspace/Cluaiz-Technologies/cluaiz-tools/docs/architecture/system-overview.md) — End-to-end system topology, engine boundaries, and security model.
- [Turn Lifecycle](file:///c:/Users/Aryan/my/Cluaiz-workspace/Cluaiz-Technologies/cluaiz-tools/docs/architecture/turn-lifecycle.md) — Single-pass `<TRIGGER:...>` interception, pivot continuations, and turn lifecycles.
- [Registry & Auto-Probe](file:///c:/Users/Aryan/my/Cluaiz-workspace/Cluaiz-Technologies/cluaiz-tools/docs/architecture/registry-and-discovery.md) — `ToolsRegistry`, boot-time filesystem probing, and Prefix Caching.

---

### 🧠 2. Skills (`docs/skills/`)
- [Skills Overview](file:///c:/Users/Aryan/my/Cluaiz-workspace/Cluaiz-Technologies/cluaiz-tools/docs/skills/overview.md) — Cognitive prompt architecture and KV-cache injection.
- [Skill Package Schema](file:///c:/Users/Aryan/my/Cluaiz-workspace/Cluaiz-Technologies/cluaiz-tools/docs/skills/manifest-schema.md) — `package.json` specification and field reference for skills.
- [Authoring Guide](file:///c:/Users/Aryan/my/Cluaiz-workspace/Cluaiz-Technologies/cluaiz-tools/docs/skills/authoring-guide.md) — Writing high-assurance `SKILL.md` prompts and YAML frontmatter.
- [Tutorial: Build Your First Skill](file:///c:/Users/Aryan/my/Cluaiz-workspace/Cluaiz-Technologies/cluaiz-tools/docs/skills/tutorial-first-skill.md) — Step-by-step walkthrough.

---

### ⚡ 3. Plugins (`docs/plugins/`)
- [Plugins Overview](file:///c:/Users/Aryan/my/Cluaiz-workspace/Cluaiz-Technologies/cluaiz-tools/docs/plugins/overview.md) — Wasmtime sandbox, fuel limits, and memory caps.
- [Plugin Package Schema](file:///c:/Users/Aryan/my/Cluaiz-workspace/Cluaiz-Technologies/cluaiz-tools/docs/plugins/manifest-schema.md) — `package.json` specification for plugins.
- [WASM C-ABI & Host ABI Guide](file:///c:/Users/Aryan/my/Cluaiz-workspace/Cluaiz-Technologies/cluaiz-tools/docs/plugins/wasm-abi-guide.md) — Memory exports and Capability-Gated Host hooks.
- [Tutorial: Build Your First Plugin](file:///c:/Users/Aryan/my/Cluaiz-workspace/Cluaiz-Technologies/cluaiz-tools/docs/plugins/tutorial-first-plugin.md) — Writing and compiling a Rust WASM plugin.

---

### 🔌 4. MCP (`docs/mcp/`)
- [MCP Overview](file:///c:/Users/Aryan/my/Cluaiz-workspace/Cluaiz-Technologies/cluaiz-tools/docs/mcp/overview.md) — Model Context Protocol in Cluaiz, stdio JSON-RPC 2.0.
- [MCP Package Schema](file:///c:/Users/Aryan/my/Cluaiz-workspace/Cluaiz-Technologies/cluaiz-tools/docs/mcp/manifest-schema.md) — `package.json` specification for MCP connectors.
- [Tutorial: Connect an MCP Server](file:///c:/Users/Aryan/my/Cluaiz-workspace/Cluaiz-Technologies/cluaiz-tools/docs/mcp/tutorial-connect-mcp.md) — Packaging and testing an MCP server.

---

### 📚 5. Reference (`docs/reference/`)
- [CLI Reference](file:///c:/Users/Aryan/my/Cluaiz-workspace/Cluaiz-Technologies/cluaiz-tools/docs/reference/cli.md) — `cluaiz` CLI command reference.
- [Hub Registry Architecture](file:///c:/Users/Aryan/my/Cluaiz-workspace/Cluaiz-Technologies/cluaiz-tools/docs/reference/hub-registry.md) — `registry.json`, `family.json`, `package.json`, and SVG assets.
