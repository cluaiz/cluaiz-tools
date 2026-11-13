# Authoring WebAssembly Plugins for Cluaiz

WebAssembly (WASM) plugins provide fast, sandboxed compute extensions for the Cluaiz engine. They need nothing installed on the computer and can do only what the host allows.

## 1. Sandbox & Security Model

WASM plugins run inside an isolated WebAssembly sandbox managed by the Cluaiz engine runtime:
- **No Filesystem Access:** Plugins cannot directly read, write, or query the host filesystem. If files are needed, use an MCP server instead.
- **Controlled Host Imports:** Plugins can only call host functions explicitly exposed through the `cluaiz` import module.
- **Network Permissions:** Plugins cannot bind network sockets. Outbound HTTP calls are restricted to HTTPS requests explicitly permitted in `package.json` with the `network` kind (`info`, `allow`, and `access`: `full`, or `strict` with `hosts`; `docs/package-spec.md#permissions`). The user can refuse it; then `http_fetch` answers `permission_off` and the plugin must pass that on as a clear error. The time and system name functions are always available; the time is not a permission (`docs/reference/permissions.md`).
- **Secrets Management:** Sensitive API keys must never be hardcoded. They are named in the top-level `env` and filled in by the Engine.

## 2. ABI & Memory Exchange Contract

The Cluaiz engine exchanges data with WASM plugins via standard linear memory using JSON envelopes. Every plugin exports `memory` and two functions:

### 2.1 `allocate(len: u32) -> *mut u8`
The host calls `allocate` to reserve a byte buffer in the plugin's WebAssembly linear memory before copying input JSON bytes into it.

```rust
#[no_mangle]
pub extern "C" fn allocate(len: u32) -> *mut u8 {
    let mut buffer = Vec::<u8>::with_capacity(len as usize);
    let ptr = buffer.as_mut_ptr();
    core::mem::forget(buffer); // Prevent deallocation before host writes
    ptr
}
```

### 2.2 `execute_cel(start: *const u8, len: u32) -> u64`
The host passes the pointer and length of the input JSON arguments. The plugin processes the input, serializes its JSON response, and packs the return pointer and length into a single `u64`:
`return (ptr << 32) | len`

```rust
#[no_mangle]
pub unsafe extern "C" fn execute_cel(start: *const u8, len: u32) -> u64 {
    let input_bytes = if len > 0 && !start.is_null() {
        core::slice::from_raw_parts(start, len as usize)
    } else {
        &[]
    };
    
    let response_bytes = handle_execution(input_bytes).to_string().into_bytes();
    let ptr = response_bytes.as_ptr() as u64;
    let length = response_bytes.len() as u64;
    core::mem::forget(response_bytes);
    
    (ptr << 32) | length
}
```

## 3. Host Imports (`cluaiz` module)

```rust
#[link(wasm_import_module = "cluaiz")]
extern "C" {
    /// Current UTC time in milliseconds
    fn now_utc_ms() -> i64;
    /// 1 Windows, 2 macOS, 3 Linux, 0 other
    fn os_platform() -> i32;
    /// This computer's offset from UTC, in minutes
    fn utc_offset_minutes() -> i32;
    /// A JSON request in, `(ptr << 32) | len` of a JSON answer out (written through `allocate`).
    /// Refused unless package.json allows the network and the user has not switched it off
    fn http_fetch(ptr: *const u8, len: u32) -> u64;
}
```

The full contract, with limits and the request format of `http_fetch`, is in the cluaiz-tools docs: `docs/plugins/wasm-abi-guide.md`.

## 4. `package.json` for a Plugin

`functions` sit at the top level, not inside a version. Triggers are a **plain list** for plugins. The `.wasm` comes from the release; `versions.<v>.files` lists the readable files only.

```json
{
  "id": "hash-utility",
  "name": "Hash Utility",
  "category": "utility",
  "hub_type": "plugin",
  "build_type": "wasm",
  "icon": "assets/icon.svg",
  "description": "Calculates SHA-256 hashes of text. Use when the user asks for a hash or checksum of a string.",
  "license": "Apache-2.0",
  "triggers": ["sha256", "hash this text", "checksum"],
  "functions": [
    {
      "name": "hash_text",
      "description": "Calculate the SHA-256 hash of a text.",
      "parameters": {
        "type": "object",
        "properties": { "text": { "type": "string", "description": "The text to hash" } },
        "required": ["text"]
      }
    }
  ],
  "latest_version": "1.0.0",
  "versions": {
    "1.0.0": {
      "updated_at": "2026-11-11T09:00:00Z",
      "builds_os": ["wasm"],
      "files": { "readme": "README.md", "skill": "SKILL.md", "icon": "assets/icon.svg" }
    }
  }
}
```

A key for an outside service is named in the top-level `env`, `{"EXAMPLE_API_KEY": {"info": "Your key for the example service", "need": "required"}}`. The Engine takes it from the system environment, or the user sets it with `cluaiz plugin config <id> EXAMPLE_API_KEY`. Use `strict` network access with `hosts` so the key goes only to its service. Never write a key into the files.

The Engine passes only the arguments to `execute_cel`, not the function's name: a plugin with several functions tells them apart by their arguments.

## 5. Build & Optimization Pipeline

Compile to the `wasm32-unknown-unknown` target:

```bash
# Compile release binary
cargo build --release --target wasm32-unknown-unknown

# Optimize binary size using wasm-opt (optional)
wasm-opt -Oz target/wasm32-unknown-unknown/release/hash_utility.wasm -o hash-utility.wasm
```

Copy the result to the installed tool folder as `<tool_id>.wasm` to try it before a release.
