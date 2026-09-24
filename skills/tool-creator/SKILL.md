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

## 🌐 SECTION 1.1: TARGET DIRECTORY & ENVIRONMENT RESOLUTION PROTOCOL

Cluaiz tools operate under a dual-environment model (Source Repository vs Installed Runtime). Hardcoded static paths are strictly prohibited. The Tool Creator dynamically resolves target directories according to the active environment:

### 1. Dual-Environment Hierarchy
- **Installed Runtime (Host System):**
  - **Windows:** `%USERPROFILE%\.cluaiz\tools\` (e.g. `C:\Users\<Username>\.cluaiz\tools\`)
  - **Linux / macOS:** `$HOME/.cluaiz/tools/` (e.g. `/home/<username>/.cluaiz/tools/`)
  - Subdirectories:
    - `.cluaiz/tools/skills/`
    - `.cluaiz/tools/plugins/`
    - `.cluaiz/tools/mcp/`
- **Development Repository (Source Workspace):**
  - Target Path: `<workspace>/Cluaiz-Technologies/cluaiz-tools/`
  - Subdirectories:
    - `skills/`
    - `plugins/`
    - `mcp/`

### 2. Autonomous Path Resolution Logic
1. When developing tools within the `cluaiz-tools` repository, scaffold into the relative `skills/`, `plugins/`, or `mcp/` root directories.
2. When deploying or exporting for system-wide execution, verify the host operating system:
   - On Windows, expand `%USERPROFILE%` / `$env:USERPROFILE`.
   - On Linux/macOS, expand `$HOME`.
3. Always normalize filesystem paths using standard separators to avoid Windows backslash escaping errors.

---

## 🧠 SECTION 2: PILLAR I — SKILL AUTHORING PROTOCOL

A **Skill** provides the cognitive reasoning framework. It contains NO executable machine binaries.

### Directory Layout:
```
skills/<skill-name>/
├── package.json          ← Tools registry packaging, metadata & semantic triggers
├── SKILL.md              ← YAML Frontmatter + Neural Protocol Instructions
└── assets/
    └── icon.svg          ← 24x24 Vector UI Icon
```

### 1. `package.json` Template (Single Source of Truth):
```json
{
  "id": "kebab-case-skill-name",
  "name": "Human Readable Skill Name",
  "category": "engineering",
  "hub_type": "skill",
  "description": "High-precision summary of cognitive capabilities.",
  "author": {
    "name": "Aryan",
    "url": "https://github.com/cluaiz"
  },
  "license": "Apache-2.0",
  "triggers": {
    "semantic": [
      "primary keyword trigger",
      "secondary action phrase",
      "domain specific intent"
    ]
  },
  "dependencies": {
    "plugins": {},
    "mcp": {},
    "skills": {}
  },
  "latest_version": "1.0.0",
  "versions": {
    "1.0.0": {
      "updated_at": "2026-09-24T12:00:00Z",
      "files": {
        "skill": "SKILL.md",
        "icon": "assets/icon.svg"
      }
    }
  }
}
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
├── assets/
│   └── icon.svg          ← 24x24 Vector UI Icon
└── package.json          ← Tools registry packaging & distribution manifest
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

### 4. `package.json` Plugin Specification (Single Source of Truth):
```json
{
  "id": "kebab-case-plugin-name",
  "name": "Human Readable Plugin Name",
  "category": "utility",
  "hub_type": "plugin",
  "build_type": "wasm",
  "description": "Deterministic in-process compute capabilities.",
  "author": {
    "name": "Aryan",
    "url": "https://github.com/cluaiz"
  },
  "license": "Apache-2.0",
  "tags": ["WASM", "Compute"],
  "dependencies": {
    "plugins": {},
    "mcp": {},
    "skills": {}
  },
  "latest_version": "1.0.0",
  "versions": {
    "1.0.0": {
      "updated_at": "2026-09-24T12:00:00Z",
      "builds_os": ["wasm"],
      "files": {
        "binary": "logic.wasm",
        "icon": "assets/icon.svg"
      }
    }
  }
}
```

---

## 🔌 SECTION 4: PILLAR III — MCP SERVER ENGINEERING PROTOCOL

An **MCP Server** runs as an isolated OS subprocess communicating over stdio JSON-RPC 2.0.

### Directory Layout:
```
mcp/<server-name>/
├── package.json          ← Tools registry packaging, subprocess command & arguments
└── assets/
    └── icon.svg          ← 24x24 Vector UI Icon
```

### 1. `package.json` MCP Specification (Single Source of Truth):
```json
{
  "id": "kebab-case-mcp-name",
  "name": "Human Readable MCP Server Name",
  "category": "mcp",
  "hub_type": "mcp",
  "description": "External tool bridge executing over stdio JSON-RPC 2.0.",
  "author": {
    "name": "Aryan",
    "url": "https://github.com/cluaiz"
  },
  "license": "MIT",
  "tags": ["MCP", "Bridge", "Subprocess"],
  "dependencies": {
    "plugins": {},
    "mcp": {},
    "skills": {}
  },
  "latest_version": "1.0.0",
  "versions": {
    "1.0.0": {
      "updated_at": "2026-09-24T12:00:00Z",
      "command": "npx",
      "args": [
        "-y",
        "@modelcontextprotocol/server-example"
      ],
      "files": {
        "icon": "assets/icon.svg"
      }
    }
  }
}
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
        "skill": "SKILL.md",
        "icon": "assets/icon.svg"
      }
    }
  }
}
```

### 3. Registry Synchronization (`family.json`):
Every newly scaffolded tool must be registered in the appropriate `family.json` to enable instant runtime discovery:
- **For Skills:** Append entry to `skills/family.json`:
  ```json
  "<skill-name>": "<skill-name>/package.json"
  ```
- **For Plugins:** Append entry to `plugins/family.json`:
  ```json
  "<plugin-name>": "<plugin-name>/package.json"
  ```
- **For MCP Servers:** Append entry to `mcp/family.json`:
  ```json
  "<mcp-name>": "<mcp-name>/package.json"
  ```

---

## 🛡️ SECTION 6: MANDATORY PRE-FLIGHT AUDIT CHECKLIST

When generating any tool:
1. **Naming Hygiene:** Folder name, package `id`, and `package.json` `name` must match exactly in lowercase `kebab-case`.
2. **Domain Separation:** Plugins NEVER contain `SKILL.md`. Skills NEVER contain `.wasm`.
3. **Memory Safety:** WASM plugins MUST export `allocate`, `deallocate`, and `execute_cel` with bitpacked pointer returns.
4. **Zero Marketing Words:** Strictly banned from using hype adjectives (Magic, Blazing, Silicon-Native, OS, Universal).
5. **Registry Compatibility:** Valid `package.json` and standalone `assets/icon.svg` MUST exist in the package bundle.
6. **Path Integrity:** Ensure correct directory placement in `skills/`, `plugins/`, or `mcp/` without hardcoded static user paths.

---

## 🚀 SECTION 7: AUTONOMOUS FOUNDRY EXECUTION WORKFLOW

When requested to create or scaffold a new extension:

1. **Step 1: Pillar Classification**
   - Determine whether the requirement is a **Skill** (cognitive reasoning), **Plugin** (deterministic WASM compute), or **MCP Server** (external I/O).
2. **Step 2: Directory Scaffolding**
   - Create the target folder under `skills/<name>/`, `plugins/<name>/`, or `mcp/<name>/`.
   - Create the `assets/` subfolder.
3. **Step 3: Specification & Code Synthesis**
   - Write `package.json` (metadata, semantic triggers, dependencies, and file mappings).
   - Write the implementation (`SKILL.md` for skills, `src/lib.rs` & `Cargo.toml` for plugins, or stdio runner for MCP).
   - Generate `assets/icon.svg` with appropriate color coding.
4. **Step 4: Registry Registration**
   - Insert the new tool's descriptor mapping into `family.json`.
5. **Step 5: Pre-Flight Verification**
   - Validate against the Section 6 checklist before reporting completion.

