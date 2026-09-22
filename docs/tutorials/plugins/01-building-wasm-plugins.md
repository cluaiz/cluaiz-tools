---
title: "Build Your First WASM Plugin"
description: "How to compile WebAssembly (WASM) plugins for sandboxed execution."
category: "Tutorials"
---

# 1. Build Your First Plugin (WASM / C++)

In the cluaiz ecosystem, a **WASM Plugin** provides secure, sandboxed execution (`type: plugin`). Plugins run entirely within the Engine's WASM sandbox with strict CPU fuel limits and RAM caps.

> [!TIP]
> **For more details, reference:** [`skill_architecture.md`](file:///c:/Users/Aryan/my/Cluaiz-workspace/Cluaiz-Technologies/cluaiz-tools/docs/architecture/skill_architecture.md)
> 
> For **Plugins**, creating a `SKILL.md` file is **OPTIONAL but RECOMMENDED**. While a plugin can execute blindly, providing a `SKILL.md` teaches the AI how to use it properly and gives the AI the power to understand the tool's context.

---

## The WASM Architecture

```mermaid
flowchart TD
    A["cluaiz Engine"] -->|CEL Router Identifies Plugin| B{"Wasmtime VM"}
    B -->|Compiles CxpPayload (MsgPack)| C["Compiled Plugin (.wasm)"]
    C -->|Executes Sandbox Logic| B
    B -->|Returns CString| A
```

> [!NOTE]
> Plugins are highly secure. Since they execute inside the `Wasmtime` VM, they cannot access the host operating system, read local files, or crash the Engine memory unless explicitly granted permission.

## Step 1: The Package Specification (`package.json`)

Every plugin requires a `package.json` to define identity, distribution links, and dependencies:

```json
{
  "id": "cluaiz-math-accelerator",
  "name": "Math Accelerator",
  "category": "utility",
  "hub_type": "plugin",
  "build_type": "wasm",
  "logo": "/assets/icon.svg",
  "title": "Hardware-Accelerated Math Plugin",
  "description": "A high-performance math parsing plugin compiled to WASM.",
  "author": {
    "name": "Cluaiz Engineers",
    "url": "https://github.com/cluaiz"
  },
  "license": "Apache-2.0",
  "dependencies": {
    "plugins": {},
    "mcp": {},
    "skills": {}
  },
  "latest_version": "1.0.0",
  "versions": {
    "1.0.0": {
      "updated_at": "2026-07-01T12:00:00Z",
      "builds_os": ["wasm"],
      "files": {
        "binary": "logic.wasm",
        "icon": "/assets/icon.svg",
        "file_directory": "https://github.com/cluaiz/cluaiz-tools/releases/download/v1.0.0/math-files.zip"
      }
    }
  }
}
```

## Step 2: Scaffold the Rust WASM Project

To build a plugin in Rust, compile to the `wasm32-wasi` target.

```bash
cargo new --lib math_plugin
cd math_plugin
rustup target add wasm32-wasi
```

Update your `Cargo.toml`:
```toml
[package]
name = "math_plugin"
version = "1.0.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
serde = { version = "1.0", features = ["derive"] }
rmp-serde = "1.1" # For MsgPack parsing
```

## Step 3: The Code (MsgPack via C-Pointer)

Inside `src/lib.rs`, expose the `cluaiz_entry` function. Unlike basic strings, the Engine passes a structured `CxpPayload` pointer containing the `MsgPack` serialized data.

```rust
use std::ffi::{c_char, CString};

// The data structure passed by the Engine
#[repr(C)]
pub enum PayloadType { Json, Cdql, WasmBinary, RawBytes, Bincode, MsgPack }

#[repr(C)]
pub struct CxpPayload {
    pub payload_type: PayloadType,
    pub data_ptr: *const u8,
    pub data_len: usize,
}

#[no_mangle]
pub extern "C" fn cluaiz_entry(payload_ptr: *const CxpPayload) -> *mut c_char {
    if payload_ptr.is_null() {
        return std::ptr::null_mut();
    }

    let payload_ref = unsafe { &*payload_ptr };
    let incoming_bytes = unsafe {
        std::slice::from_raw_parts(payload_ref.data_ptr, payload_ref.data_len)
    };

    // Deserialize the MsgPack data passed from the Engine
    // (Logic will be expanded in the Next Tutorial)

    let response = r#"{"status": "success", "message": "WASM execution complete."}"#;
    CString::new(response).unwrap_or_default().into_raw()
}
```

To compile:
```bash
cargo build --target wasm32-wasi --release
```
