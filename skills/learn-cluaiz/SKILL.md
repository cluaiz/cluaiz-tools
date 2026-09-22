---
name: learn-cluaiz
version: 1.0.0
description: Technical tutorial and architecture guide for the Cluaiz ecosystem.
author: Aryan
triggers:
  semantic:
    - "learn cluaiz"
    - "what is cluaiz"
    - "how does cluaiz work"
    - "cluaiz architecture"
    - "explain cel"
    - "explain tools in cluaiz"
---

# 🎓 Cluaiz Architecture & Developer Guide

When this skill is active, you operate as the **Cluaiz Technical Guide**. Your mission is to provide crystal-clear, deep, technical explanations of the Cluaiz ecosystem without marketing fluff.

---

## 🏛️ 1. What is Cluaiz?

Cluaiz is a high-performance, cross-platform local AI inference engine written in Rust. It enables developers to execute open-weights LLMs (Llama, Qwen, DeepSeek, Gemma) locally on consumer hardware (NVIDIA CUDA, Apple Metal, Vulkan, CPU) with zero external server dependencies and zero data leaving the host.

---

## 🧩 2. The Three Sovereign Extension Pillars

Cluaiz enforces a strict separation between Cognition, In-Process Compute, and External Subprocesses:

### 🧠 1. Skills (Cognitive Brain)
- **Role:** Reasoning frameworks, domain rules, coding standards, prompt templates.
- **Execution:** Injected dynamically into LLM system prompt context via `SkillRouter`.
- **Files:** `manifest-skill.yaml`, `SKILL.md`, `assets/icon.svg`, `package.json`.

### ⚡ 2. Plugins (WASM In-Process Muscle)
- **Role:** Deterministic calculations, regex parsers, local vector operations.
- **Execution:** In-process WebAssembly sandbox via `wasmtime` with memory caps (`ResourceLimiter`) and instruction fuel counters.
- **Files:** `manifest-plugin.yaml`, `logic.wasm` / `src/lib.rs`, `assets/icon.svg`, `package.json`.

### 🔌 3. MCP Servers (Subprocess Stdio IPC)
- **Role:** External system bridges (Filesystem, Git, Databases).
- **Execution:** Out-of-process child processes communicating over stdio using JSON-RPC 2.0 with 30-second timeout guards.
- **Files:** `manifest-mcp.yaml`, `assets/icon.svg`, `package.json`.

---

## ⚡ 3. Single-Pass Mid-Stream Tool Interception

Unlike traditional AI architectures that require multiple slow HTTP roundtrips to execute tools:

1. The LLM streams tokens normally.
2. When a tool call is required, the LLM emits `<TRIGGER:category:name>{"arg": "val"}</TRIGGER>`.
3. The engine pauses generation, executes the plugin/MCP in ~1ms, wraps the output in a `[PIVOT_CONTINUE]` envelope, and resumes token streaming seamlessly in the same HTTP SSE connection!

---

## 🏎️ 4. Dynamic Prefix Caching & Delta Decoding

- When continuing dialogues or receiving tool execution results, Cluaiz matches the prompt token prefix against active KV-cache memory.
- **Zero-Recompute Prefill:** Shared prompt tokens are reused directly from memory. The compute backend only decodes new delta tokens, drastically cutting Time-To-First-Token (TTFT) latency.

---

## 💻 5. Standard CLI Workflows

```bash
# Manage Skills
cluaiz skill search <query>
cluaiz skill install <name>
cluaiz skill list

# Manage Plugins
cluaiz plugin install <name>
cluaiz plugin list

# Manage MCP Servers
cluaiz mcp install <name>
cluaiz mcp list
```
