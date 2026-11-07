---
name: tool-creator
description: "Creates new skills, WebAssembly plugins and MCP packages for Cluaiz. Use when the user wants a new tool."
---

# Tool Creator

Choose the type, call `create_tool`, finish what it reports as `next`, then test the tool.

## 1. Choose the type

| The tool must | Use |
|---|---|
| Teach the model a way of working, with no program | `skills` |
| Compute from its input, or call public web addresses, with nothing to install | `plugins` (WebAssembly) |
| Use files, a database, or other programs, or an MCP server already exists | `mcp` |

A WebAssembly plugin cannot read files or start programs. If the tool must, use MCP.

## 2. Create it

```json
{"category": "plugins", "tool_id": "word-count", "name": "Word Count", "description": "Counts words in text. Use when the user asks how long a text is.", "triggers": ["count words", "word count"]}
```

An MCP package also needs `command` and `args`:

```json
{"category": "mcp", "tool_id": "my-server", "name": "My Server", "description": "What it does.", "command": "npx", "args": ["-y", "my-mcp-package"]}
```

The tool is written to the Engine's tools folder. If the open folder is the `cluaiz-tools` repository, it is also added there and to `family.json`. An existing tool is refused unless you pass `overwrite: true` and the user agreed.

Write a description that says what the tool does and when to use it, and triggers that a user would really say. The model picks tools from these.

## 3. Finish a plugin

The answer's `next` gives the exact steps. Write the logic in `src/lib.rs`, then build:

```bash
cargo build --release --target wasm32-unknown-unknown
```

Copy the `.wasm` file to the installed folder as `<tool_id>.wasm`. Update `functions` in `package.json` if the inputs changed. The Engine passes only the arguments, not the function's name, so a plugin with several functions must tell them apart by its arguments.

The plugin exports `memory`, `allocate(len) -> ptr` and `execute_cel(ptr, len) -> u64`, whose result is `(ptr << 32) | len` of its JSON answer. It can import from the `cluaiz` module: `now_utc_ms`, `utc_offset_minutes`, `os_platform` and, with `permissions.network_access`, `http_fetch`.

## 4. Test it

Call the tool once with good input and once with bad input. Bad input must give a clear error, not a crash. Report what you tested and what you did not. Do not say it works until you have run it.

## Rules

- Never put a key or password in a tool's files. A plugin asks for one through `permissions.secrets`.
- Keep `SKILL.md` short: exact steps and limits, with no claims about what the tool cannot do.
- Keep the family key, the folder name and `id` in `package.json` the same.
