# Tutorial: first plugin

Build `hasher`, a WebAssembly plugin that returns the SHA-256 hash of a text.

## 1. Create the project

```bash
rustup target add wasm32-unknown-unknown
cargo new --lib hasher
cd hasher
```

`Cargo.toml`:

```toml
[package]
name = "hasher"
version = "1.0.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
serde_json = "1"
sha2 = "0.10"
hex = "0.4"
```

## 2. Write the plugin

`src/lib.rs`:

```rust
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

#[no_mangle]
pub extern "C" fn allocate(len: u32) -> *mut u8 {
    let mut buffer = Vec::with_capacity(len as usize);
    let address = buffer.as_mut_ptr();
    std::mem::forget(buffer);
    address
}

#[no_mangle]
pub unsafe extern "C" fn execute_cel(address: *const u8, len: u32) -> u64 {
    let input = std::slice::from_raw_parts(address, len as usize);
    let output = run(input).to_string().into_bytes();
    let packed = ((output.as_ptr() as u64) << 32) | output.len() as u64;
    std::mem::forget(output);
    packed
}

fn run(input: &[u8]) -> Value {
    let arguments: Value = match serde_json::from_slice(input) {
        Ok(value) => value,
        Err(error) => return json!({ "status": "error", "message": error.to_string() }),
    };
    let Some(text) = arguments["text"].as_str() else {
        return json!({ "status": "error", "message": "'text' is required" });
    };
    json!({ "status": "success", "sha256": hex::encode(Sha256::digest(text)) })
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn hashes_the_text() {
        let answer = run(br#"{"text": "hello"}"#);
        assert_eq!(answer["sha256"], "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824");
    }

    #[test]
    fn a_missing_field_is_an_error() {
        assert_eq!(run(b"{}")["status"], "error");
    }
}
```

Keep the work in a normal function, as `run` is here. `cargo test` can then test it without WebAssembly.

## 3. Build

```bash
cargo test
cargo build --release --target wasm32-unknown-unknown
```

The file is `target/wasm32-unknown-unknown/release/hasher.wasm`.

## 4. Write `package.json`

```json
{
  "id": "hasher",
  "name": "Hasher",
  "category": "utility",
  "hub_type": "plugin",
  "build_type": "wasm",
  "icon": "assets/icon.svg",
  "description": "Computes the SHA-256 hash of a text. Use when the user asks for a hash or checksum.",
  "license": "Apache-2.0",
  "triggers": ["sha256", "hash", "checksum"],
  "functions": [
    {
      "name": "hash_text",
      "description": "Compute the SHA-256 hash of a text.",
      "parameters": {
        "type": "object",
        "properties": { "text": { "type": "string" } },
        "required": ["text"]
      }
    }
  ],
  "dependencies": { "plugins": {}, "mcp": {}, "skills": {} },
  "permissions": {},
  "latest_version": "1.0.0",
  "versions": {
    "1.0.0": {
      "updated_at": "2026-11-06T12:00:00Z",
      "builds_os": ["wasm"],
      "files": { "readme": "README.md", "skill": "SKILL.md", "icon": "assets/icon.svg" }
    }
  }
}
```

`permissions` is `{}` because the plugin uses only its input. A plugin that reaches the internet declares `network` there; see [Plugin package.json](package-json.md#permissions).

## 5. Add the icon and `README.md`

Write `README.md` for human users (explaining the plugin's purpose, WebAssembly execution flow with a Mermaid diagram, function inputs/outputs and usage), write `SKILL.md` with brief operational instructions for the AI model, and create `assets/icon.svg`, a pure SVG with a 24 by 24 `viewBox`:

```xml
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
  <path d="M4 9h16M4 15h16M10 3 8 21M16 3l-2 18"/>
</svg>
```

## 6. Test it in the Engine

Install it by hand in a test home, so your real installation stays untouched.

**PowerShell**

```powershell
$env:CLUAIZ_HOME = "$PWD\test-home"
New-Item -ItemType Directory -Force "$env:CLUAIZ_HOME\tools\plugins\hasher\assets" | Out-Null
Copy-Item package.json "$env:CLUAIZ_HOME\tools\plugins\hasher\"
Copy-Item assets\icon.svg "$env:CLUAIZ_HOME\tools\plugins\hasher\assets\"
Copy-Item target\wasm32-unknown-unknown\release\hasher.wasm "$env:CLUAIZ_HOME\tools\plugins\hasher\hasher.wasm"
cluaiz plugin list
```

**Linux and macOS**

```bash
export CLUAIZ_HOME="$PWD/test-home"
mkdir -p "$CLUAIZ_HOME/tools/plugins/hasher/assets"
cp package.json "$CLUAIZ_HOME/tools/plugins/hasher/"
cp assets/icon.svg "$CLUAIZ_HOME/tools/plugins/hasher/assets/"
cp target/wasm32-unknown-unknown/release/hasher.wasm "$CLUAIZ_HOME/tools/plugins/hasher/hasher.wasm"
cluaiz plugin list
```

Start `cluaiz serve` in that same shell, then send this to `POST /v1/cep`:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools.call",
  "params": { "kind": "plugin", "name": "hasher", "fn": "hash_text", "input": { "text": "hello" } }
}
```

The result holds the same hash as the test.

## 7. Publish

1. Put the folder at `plugins/hasher/`, without `target/`.
2. Add it to `plugins/family.json`:

   ```json
   "hasher": { "type": "local", "path": "hasher/package.json" }
   ```

3. Push. CI builds `hasher_v1.0.0.wasm` and the zip, and releases them under `plugin-hasher-v1.0.0`.
4. Test with `cluaiz plugin install hasher` in a clean `CLUAIZ_HOME`.
