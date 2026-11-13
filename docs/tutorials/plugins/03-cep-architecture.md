---
title: "CEP and plugins"
description: "What CEP is for a plugin today, and the planned way a plugin will use it."
category: "Tutorials"
---

# 3. CEP and plugins

CEP (Cluaiz Engine Protocol) is how programs talk to the Engine: `POST /v1/cep`, JSON-RPC 2.0. It replaced the old CEL language for this purpose. See [From CEL to CEP](../../architecture/cel-to-cep.md).

## Today

| Direction | How |
|---|---|
| Model calls a plugin | A standard tool call. The Engine runs the plugin. No CEP is involved |
| A program calls a plugin | CEP `tools.call` |
| A plugin calls the Engine | **Not possible through CEP.** It has only `now_utc_ms`, `utc_offset_minutes`, `os_platform` and `http_fetch` |

A direct call, useful for testing:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools.call",
  "params": {
    "kind": "plugin",
    "name": "math",
    "fn": "math_evaluate",
    "input": { "expression": "10 * 5" }
  }
}
```

The plugin receives only `input`, as JSON text. It does not see `fn`.

## Planned

**This is a design. It is not built.** Do not use it yet.

A WebAssembly plugin is closed in, which is what makes it safe, and also what limits it to computing on its input. The plan gives it one way out: a single host function that carries a CEP request.

```rust
#[link(wasm_import_module = "cluaiz")]
extern "C" {
    // Planned. Sends a CEP request and returns the address and length of the reply.
    fn cep(request: *const u8, len: u32) -> u64;
}
```

The plugin declares what it needs:

```json
"permissions": {
  "cep": { "info": "Transcribes audio and reads the files it is given", "allow": true, "methods": ["audio.run", "fs.read"] }
}
```

| Rule | Reason |
|---|---|
| A method not listed is refused | The manifest says what the plugin can do |
| Calls pass through the same approval as other tools | One place for security |
| `system.exec` is refused by default | It would undo the sandbox |
| One function only | A new Engine feature becomes a CEP method, and the plugin interface stays the same |

CEP has methods for files, audio and memory. Network access does not need it: a plugin already reaches the internet through `http_fetch` with the `network` permission.

With this, a plugin could offer a voice note tool while still needing nothing on the machine. `cep` is not a permission kind today, and `validate_tool` reports it.
