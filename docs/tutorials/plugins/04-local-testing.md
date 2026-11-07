---
title: "Testing plugins locally"
description: "Test the logic with cargo, then the real plugin in the Engine."
category: "Tutorials"
---

# 4. Testing plugins locally

Test in two steps. Do both before publishing.

## Step 1: The logic

Keep the work in a plain function and test it natively. It is fast and needs no WebAssembly.

```rust
fn run(input: &[u8]) -> serde_json::Value {
    // ...
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn adds() {
        assert_eq!(run(br#"{"expression": "1 + 1"}"#)["result"], 2.0);
    }
}
```

```bash
cargo test
```

## Step 2: The real file in the Engine

This catches what native tests cannot: wrong target, missing exports and unavailable imports.

```bash
cargo build --release --target wasm32-unknown-unknown
```

Check the first bytes of the file. They must be `\0asm`:

```bash
head -c 4 target/wasm32-unknown-unknown/release/my_plugin.wasm | xxd
```

Copy it into a test home as in [Tutorial: first plugin](../../plugins/tutorial-first-plugin.md#6-test-it-in-the-engine), then call it through `tools.call`.

Check how the tool is found too. `tools.discover` shows what the matcher would offer:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools.discover",
  "params": { "query": "what is 17 times 23", "explain": true }
}
```

If your plugin is not among the candidates, improve its `description` and `triggers`.

## Common errors

| Error | Cause |
|---|---|
| `Not a valid WebAssembly plugin` | A native build was renamed to `.wasm` |
| `The plugin could not start` | It imports something the Engine does not provide, usually WASI |
| `The plugin has no 'allocate'` or `'execute_cel'` | Missing export or missing `#[no_mangle]` |
| `The plugin has no 'memory'` | Memory is not exported |
| `The plugin failed` | A panic, or the instruction limit was reached |
| `The plugin gave an answer outside its memory` | A wrong address or length, or the output was freed too early |
