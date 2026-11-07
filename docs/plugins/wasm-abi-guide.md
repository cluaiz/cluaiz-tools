# WebAssembly interface

This is the contract between the Engine and a WebAssembly plugin. It is specific to Cluaiz: other agents do not run these files.

## Required exports

| Export | Signature | Purpose |
|---|---|---|
| `memory` | Linear memory | Where input and output live. Rust exports it by default |
| `allocate` | `(len: u32) -> i32` | Reserve `len` bytes for the input and return their address |
| `execute_cel` | `(ptr: i32, len: u32) -> u64` | Do the work. Return `(address << 32) \| length` of the output |

`execute_cel` is a historical name, and has nothing to do with the removed CEL language. See [From CEL to CEP](../architecture/cel-to-cep.md).

Older pages also require `deallocate`. The Engine does not call it, so it is optional. Each call gets a fresh instance, whose memory is dropped afterwards.

## The call

1. The Engine creates a new instance.
2. It calls `allocate` with the input's length, and writes the input there.
3. It calls `execute_cel` with that address and length.
4. It reads the output from the returned address and length.
5. It drops the instance. Nothing stays between calls.

## Input and output

**Input** is the function's arguments as JSON text, exactly as the model wrote them:

```json
{ "expression": "17 * 23" }
```

The function name is not included. A plugin with several functions must tell them apart by its arguments.

**Output** should be JSON text. Plain text is passed on as text.

```json
{ "status": "success", "result": 391.0, "formatted": "391" }
```

Report a failure in the output with a clear message, so the model can act on it. A panic or an instruction limit becomes a tool error.

## Minimal Rust plugin

```rust
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
    let output = run(input).into_bytes();
    let packed = ((output.as_ptr() as u64) << 32) | output.len() as u64;
    std::mem::forget(output);
    packed
}

fn run(input: &[u8]) -> String {
    format!("{{\"bytes\": {}}}", input.len())
}
```

## Host functions

These are the only things a plugin can call outside itself. They are in the `cluaiz` import module.

| Function | Signature | Returns |
|---|---|---|
| `now_utc_ms` | `() -> i64` | Current UTC time in milliseconds |
| `os_platform` | `() -> i32` | 1 Windows, 2 macOS, 3 Linux, 0 other |
| `http_fetch` | `(ptr: i32, len: u32) -> u64` | `(ptr << 32) \| len` of a JSON answer, written through `allocate` |

```rust
#[link(wasm_import_module = "cluaiz")]
extern "C" {
    fn now_utc_ms() -> i64;
    fn os_platform() -> i32;
    fn http_fetch(ptr: *const u8, len: u32) -> u64;
}
```

An import from any other module, including WASI, makes the plugin fail to start. That is why the build target is `wasm32-unknown-unknown`.

A host function for CEP requests is **Planned**. See [CEP and plugins](../tutorials/plugins/03-cep-architecture.md).

## Network access

A plugin calls `http_fetch` with a JSON request. Without `permissions.network_access` in `package.json`, every call is refused.

```json
{
  "method": "POST",
  "url": "https://api.example.com/search",
  "headers": { "Authorization": "Bearer {{secret:EXAMPLE_API_KEY}}" },
  "body": "{\"query\":\"cats\"}",
  "max_bytes": 1048576
}
```

Only `url` is required, and `method` is `GET` or `POST`. The answer is one of these:

```json
{ "status": 200, "final_url": "https://...", "content_type": "text/html", "body": "...", "truncated": false }
{ "code": "blocked", "error": "localhost is a local or private address" }
```

| `code` | Meaning |
|---|---|
| `no_permission` | The manifest does not allow this |
| `missing_setting` | The user has not set this value. No request was made |
| `blocked` | Not https, a local or private address, or a URL with a password |
| `limit` | Too many requests, or out of time |
| `network`, `bad_request` | The request failed, or was not valid |

**Keys.** A plugin never sees a key. It writes `{{secret:NAME}}` in a header, and the Engine fills in the value, only for the host the manifest names. A redirect to another host drops the plugin's headers.

**The user's own address.** A URL starting with `{{url:NAME}}` uses an address from the user's settings. It alone may be local or plain http. Never build one from a model's input.

```json
"permissions": {
  "network_access": true,
  "secrets": { "EXAMPLE_API_KEY": "api.example.com" },
  "user_urls": ["OWN_SERVER_URL"]
}
```

The user sets the values in `config/tool_settings.json` in the Engine's data folder. A plugin reads only the section named after its own folder:

```json
{ "my-plugin": { "EXAMPLE_API_KEY": "...", "OWN_SERVER_URL": "http://localhost:8080" } }
```

## Limits

| Limit | Value | When reached |
|---|---|---|
| Instructions per call | 2,000,000,000 | The call ends with an error |
| Memory per call | 256 MB, or less with `permissions.max_memory_mb` | Allocation fails |
| Requests per call | 16 | `http_fetch` answers `limit` |
| Time for requests in a call | 60 seconds to start the last one, and up to 20 seconds each | `http_fetch` answers `limit` or `network` |
| Answer per request | 5 MB, or less with `max_bytes` | The body is cut and `truncated` is true |
| Redirects per request | 5 | `http_fetch` answers `network` |
