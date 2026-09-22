---
name: tool-creator
version: 1.0.0
description: Autonomous tool scaffolding, compilation, and validation foundry for Skills, WASM Plugins, and MCP Servers in the Cluaiz ecosystem.
author: Aryan
triggers:
  semantic:
    - "create plugin"
    - "build mcp"
    - "create skill"
    - "new tool"
    - "scaffold tool"
    - "tool foundry"
    - "generate tool"
    - "build tool"
---

# 🛠️ Tool Creator Protocol

When this skill is activated, you operate as the **Tool Creator** for the Cluaiz ecosystem. Your mission is to autonomously design, scaffold, implement, compile, and validate production-grade extensions across the **Three Sovereign Pillars**: Skills, Plugins, and MCP Servers.

---

## 🧭 SECTION 1: THE THREE PILLARS DECISION MATRIX

Before generating any code or configuration, classify the user's requirement into the exact sovereign architectural pillar:

| Requirement Profile | Correct Target Pillar | Core Technology | Isolation & Sandbox |
|---|---|---|---|
| Domain rules, reasoning instructions, code review protocols, prompt frameworks | **🧠 1. Skill** | Cognitive Prompt (`SKILL.md`) + `SkillRouter` | Injected into active LLM KV-Cache context |
| Deterministic math, hashing, regex parsing, local vector compute, sub-millisecond offline logic | **⚡ 2. Plugin** | Rust compiled to WASM (`wasm32-unknown-unknown`) / Native C-ABI | `wasmtime` sandbox with RAM cap & CPU fuel |
| Filesystem I/O, Git operations, remote databases, external CLI binaries | **🔌 3. MCP Server** | External Child Process (Node.js/Python) | Subprocess stdio pipes with 30s timeout guard |

---

## 🧠 SECTION 2: PILLAR I — SKILL AUTHORING PROTOCOL

A **Skill** provides the cognitive reasoning framework. It contains NO executable machine binaries.

### Directory Layout:
```
skills/<skill-name>/
├── manifest-skill.yaml   ← Router triggers, permissions, dependencies
├── SKILL.md              ← YAML Frontmatter + Neural Protocol Instructions
├── assets/
│   └── icon.svg          ← 24x24 Vector UI Icon
└── package.json          ← Hub registry descriptor
```

### 1. `manifest-skill.yaml` Template:
```yaml
# =====================================================================
# cluaiz SKILL MANIFEST (manifest-skill.yaml)
# =====================================================================
name: "kebab-case-skill-name"
version: "1.0.0"
description: "High-precision summary of cognitive capabilities."
author: "Aryan"
type: "skill"

discovery:
  semantic_triggers:
    - "primary keyword trigger"
    - "secondary action phrase"
    - "domain specific intent"
  brain_manual: "SKILL.md"

dependencies:
  plugins: []  # Optional: WASM plugins required (e.g. ["text", "math"])
  mcp: []      # Optional: MCP servers required (e.g. ["git", "filesystem"])

permissions:
  filesystem: true
  network: false
  level: "ReadOnly" # "ReadOnly" | "ReadWrite" | "None"

execution:
  execution_mode: "auto" # "auto" (trigger-activated) | "manual" (user-pinned)
  default_turns: 2       # Turn lifetime before KV-cache auto-purge (-1 for persistent)
```

### 2. `SKILL.md` Authoring Formula:
```markdown
---
name: <skill-name>
version: 1.0.0
description: High-precision summary of cognitive capabilities.
author: Aryan
triggers:
  semantic:
    - "primary keyword trigger"
    - "secondary action phrase"
---

# 🛡️ Protocol Title

When this skill is active, you operate as [Specific Professional Role]. Your mission is [Precise Mission].

## 📋 Core Architectural Rules
1. **[Rule 1 Title]:** [Actionable rule].
2. **[Rule 2 Title]:** [Actionable rule].

## 📑 Output Formatting Contract
Always format responses strictly using this structured template:
### 🔍 Summary
- **Target:** `...`
- **Result:** `...`

### 🚨 Detailed Findings
1. **[Finding Title]** (`path/to/file:L123`)
   - **Details:** `...`
```

---

## ⚡ SECTION 3: PILLAR II — WASM PLUGIN ENGINEERING PROTOCOL

A **Plugin** is pure in-process compute muscle compiled to WebAssembly with zero ambient authority.

### Directory Layout:
```
plugins/<plugin-name>/
├── Cargo.toml            ← Rust WASM crate configuration
├── src/
│   └── lib.rs            ← C-ABI memory exports & compute logic
├── logic.wasm            ← Compiled WebAssembly binary
├── manifest-plugin.yaml  ← RAM/Fuel caps, envelope, triggers
├── assets/
│   └── icon.svg          ← 24x24 Vector UI Icon
└── package.json          ← Hub registry descriptor
```

### 1. `Cargo.toml` Configuration:
```toml
[package]
name = "kebab-case-plugin-name"
version = "1.0.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
rmp-serde = "1.3" # High-performance MsgPack serialization
```

### 2. Production Rust C-ABI Implementation (`src/lib.rs`):
```rust
use serde::{Deserialize, Serialize};

// Linker Host Hooks exposed in "cluaiz" namespace
extern "C" {
    pub fn now_utc_ms() -> i64;
    pub fn os_platform() -> i32; // 1=Windows, 2=macOS, 3=Linux, 0=Unknown
}

#[derive(Deserialize)]
struct InputPayload {
    input: String,
}

#[derive(Serialize)]
struct OutputPayload {
    status: String,
    result: String,
    timestamp_ms: i64,
}

/// Allocates linear memory buffer for incoming payload
#[no_mangle]
pub extern "C" fn allocate(len: u32) -> *mut u8 {
    let mut buf = Vec::with_capacity(len as usize);
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    ptr
}

/// Deallocates linear memory buffer to prevent leaks
#[no_mangle]
pub unsafe extern "C" fn deallocate(ptr: *mut u8, len: u32) {
    if !ptr.is_null() {
        let _ = Vec::from_raw_parts(ptr, len as usize, len as usize);
    }
}

/// Core CEL execution entry point
#[no_mangle]
pub unsafe extern "C" fn execute_cel(ptr: *const u8, len: u32) -> u64 {
    let input_bytes = std::slice::from_raw_parts(ptr, len as usize);
    
    // Parse input (JSON or MsgPack)
    let output = match serde_json::from_slice::<InputPayload>(input_bytes) {
        Ok(parsed) => {
            let current_time = now_utc_ms();
            let res = OutputPayload {
                status: "success".to_string(),
                result: format!("Processed: {}", parsed.input),
                timestamp_ms: current_time,
            };
            serde_json::to_vec(&res).unwrap_or_else(|_| b"{\"status\":\"error\"}".to_vec())
        }
        Err(e) => format!("{{\"status\":\"error\",\"message\":\"{}\"}}", e).into_bytes(),
    };

    let out_len = output.len() as u64;
    let out_ptr = output.as_ptr() as u64;
    std::mem::forget(output);

    // Bitpack pointer and length into single 64-bit return value: (ptr << 32) | len
    (out_ptr << 32) | (out_len & 0xFFFFFFFF)
}
```

### 3. Compilation & Artifact Generation:
```bash
cargo build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/<plugin_name>.wasm logic.wasm
```

### 4. `manifest-plugin.yaml` Specification:
```yaml
# =====================================================================
# cluaiz PLUGIN MANIFEST (manifest-plugin.yaml)
# =====================================================================
name: "kebab-case-plugin-name"
version: "1.0.0"
description: "Deterministic in-process compute capabilities."
author: "Aryan"
type: "plugin"

discovery:
  semantic_triggers:
    - "calculate"
    - "compute logic"
  cel_grammar: "use plugin::kebab-case-plugin-name -> execute(...)"

activation:
  lazy_load: true
  trigger_on:
    - "on_command:use plugin::kebab-case-plugin-name"

permissions:
  max_memory_mb: 16       # RAM hard cap enforced via ResourceLimiter
  max_cpu_time_ms: 500    # Fuel instruction timeout
  network_access: false   # Zero ambient authority
  vram_kv_inject: false
  prefix_caching: false
  file_system: "none"     # "none" | "read_only" | "read_write"

system_bindings: []

settings:
  mode: { type: "enum", options: ["fast", "precise"], default: "fast", desc: "Execution mode" }

execution:
  envelope: "WASM"        # "WASM" | "NATIVE"
  binary_path: "logic.wasm"
  entry_point: "execute_cel"
  payload_format: "JSON"  # "JSON" | "MsgPack"

execution_mode: "auto"
default_turns: 1
```

---

## 🔌 SECTION 4: PILLAR III — MCP SERVER ENGINEERING PROTOCOL

An **MCP Server** runs as an isolated OS subprocess communicating over stdio JSON-RPC 2.0.

### Directory Layout:
```
mcp/<server-name>/
├── manifest-mcp.yaml     ← Subprocess command, args, environment, permissions
├── assets/
│   └── icon.svg          ← 24x24 Vector UI Icon
└── package.json          ← Hub registry descriptor
```

### 1. `manifest-mcp.yaml` Specification:
```yaml
# =====================================================================
# cluaiz MCP MANIFEST (manifest-mcp.yaml)
# =====================================================================
name: "kebab-case-mcp-name"
version: "1.0.0"
description: "External tool bridge executing over stdio JSON-RPC 2.0."
author: "Aryan"
type: "mcp"

discovery:
  semantic_triggers:
    - "mcp trigger phrase"
  cel_grammar: "use mcp::kebab-case-mcp-name -> call_tool(...)"

activation:
  lazy_load: true
  trigger_on:
    - "on_command:use mcp::kebab-case-mcp-name"

permissions:
  max_memory_mb: null     # Subprocess manages its own memory
  max_cpu_time_ms: 30000  # 30-second stdio timeout guard
  network_access: true
  allowed_hosts: []
  prefix_caching: false
  file_system: "read_write"

system_bindings: []

execution:
  command: "npx"
  args:
    - "-y"
    - "@modelcontextprotocol/server-example"
  env:
    NODE_ENV: "production"

execution_mode: "manual"
default_turns: 3
```

---

## 🎨 SECTION 5: ASSET & REGISTRY SPECIFICATION

### 1. `assets/icon.svg` Requirements:
- Pure standalone `<svg>` element with `viewBox="0 0 24 24"`.
- Minimalist line strokes (`stroke-width="2"`, `fill="none"`).
- Color-coded by pillar:
  - 🧠 **Skills:** `#3B82F6` (Blue) or `#10B981` (Green)
  - ⚡ **Plugins:** `#F59E0B` (Amber) or `#EC4899` (Pink)
  - 🔌 **MCP Servers:** `#8B5CF6` (Purple)

### 2. `package.json` Descriptor:
```json
{
  "name": "kebab-case-tool-name",
  "version": "1.0.0",
  "description": "Accurate description matching manifest",
  "category": "skill", // "skill" | "plugin" | "mcp"
  "author": "Aryan",
  "license": "Apache-2.0",
  "latest_version": "1.0.0",
  "versions": {
    "1.0.0": {
      "files": {
        "manifest": "manifest-skill.yaml",
        "skill": "SKILL.md",
        "binary": "logic.wasm",
        "icon": "assets/icon.svg"
      }
    }
  }
}
```

---

## 🛡️ SECTION 6: MANDATORY PRE-FLIGHT AUDIT CHECKLIST

When generating any tool:
1. **Naming Hygiene:** Folder name, manifest `name`, and `package.json` `name` must match exactly in lowercase `kebab-case`.
2. **Domain Separation:** Plugins NEVER contain `SKILL.md`. Skills NEVER contain `.wasm`.
3. **Memory Safety:** WASM plugins MUST export `allocate`, `deallocate`, and `execute_cel` with bitpacked pointer returns.
4. **Zero Marketing Words:** Strictly banned from using hype adjectives (Magic, Blazing, Silicon-Native, OS, Universal).
5. **Registry Compatibility:** Valid `package.json` and standalone `assets/icon.svg` MUST exist in the package bundle.
