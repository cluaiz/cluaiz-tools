# Use a plugin

Install a plugin, and use it through the model and directly.

You need Cluaiz installed and a model. See the Engine's quickstart for those.

## 1. Install

```bash
cluaiz plugin install math
```

## 2. Check

```bash
cluaiz plugin list
```

It is now in `~/.cluaiz/tools/plugins/math/` as `math.wasm`, with its `package.json`.

## 3. Ask the model

```bash
cluaiz agent -p "What is 1743 * 2291? Use the math tool."
```

Behind the scenes:

1. The Engine matches your message and shows the model a card for `math`.
2. The model calls `load_tool`, then `math_evaluate`, with standard tool calls.
3. The Engine runs `math.wasm` in its sandbox, asking for approval if needed.
4. The model answers with the result.

`--yes` or `--mode full` lets it run without asking.

## 4. Call it directly

With `cluaiz serve` running, send this to `POST /v1/cep`:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools.call",
  "params": { "kind": "plugin", "name": "math", "fn": "math_evaluate", "input": { "expression": "17 * 23" } }
}
```

Add `Authorization: Bearer <token>` when your server requires a token.

## Next

- [Which plugins run today](../plugins/build-types.md#which-plugin-is-which-today)
- [Tutorial: first plugin](../plugins/tutorial-first-plugin.md)
