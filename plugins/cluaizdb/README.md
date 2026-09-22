# 🧩 cluaiz DB Plugin (Native Bridge)

This plugin is the **Native Muscle** for the cluaiz Neural Database.

## Overview
`cluaiz-db` is a **Native Plugin** that acts as the FFI bridge between the `cluaiz` Inference Engine and the `cluaizd` (Zero-Copy LMDB) Database Engine. 

By placing this inside `cluaiz Tools`, we decouple storage dependencies from the Core Engine. The Core Engine dynamically loads this plugin's `.dll` (`cluaizd_engine.dll`) at runtime to achieve low-latency memory-mapped FFI database injection.

## Project Structure
- `native/` - The Rust C-FFI crate that statically links against `cluaizd` and `engine-lmdb`. It outputs a `.dll` that the Core Engine dynamically loads.
- `SKILL.md` - The Brain prompt that teaches the AI how to use this Database via CDQL.

---

## 🧠 CEL & FFI Execution Lifecycle (`execute_cel` Demo)

Here is a detailed walk-through of how a raw CEL command is parsed, translated, and executed through the dynamic `execute_cel` FFI boundary.

### 1. The CEL Expression Input
When the LLM or a user runs a database operation, they write standard CEL syntax:
```cel
use plugin::cluaiz-db -> save(memory_id: "user_session_42", payload: "User logged in from Windows IP")
```

### 2. Translation to JSON Payload Envelope
The cluaiz Engine compiles this CEL AST and serializes it into a standard JSON payload (`CelPayload`). This is what gets sent over the C-FFI pointer boundary to `execute_cel`:

```json
{
  "action": "save",
  "memory_id": "user_session_42",
  "payload": "User logged in from Windows IP",
  "vector": [0.12, -0.45, 0.78, 0.05],
  "shard_index": null,
  "query": null
}
```

### 3. FFI Call Execution
The host engine dynamically resolves the exported symbol `execute_cel` from `cluaizd_engine.dll` and executes it, passing the JSON string pointer:
```rust
// FFI Invocation Signature:
// pub extern "C" fn execute_cel(payload_ptr: *const c_char) -> *mut c_char;
```

---

## 🛠️ Building & Compiling

To build the dynamic muscle library for this plugin, run:

```bash
cd native
cargo build --release
```
