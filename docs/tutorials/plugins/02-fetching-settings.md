---
title: "Settings in plugins"
description: "How a plugin gets configuration today, and what is planned."
category: "Tutorials"
---

# 2. Settings in plugins

**Status: Planned.** The Engine does not pass settings to a plugin. A WebAssembly plugin receives only the function's arguments. Older versions of this page described `system_bindings`, a `settings` block and MsgPack input, none of which exist.

## What to do today

**Make it an argument.** Anything the plugin needs comes through the function's arguments, and the model fills them in:

```json
{
  "name": "round_number",
  "description": "Round a number.",
  "parameters": {
    "type": "object",
    "properties": {
      "value": { "type": "number" },
      "decimals": { "type": "integer", "description": "Decimal places. Default 2." }
    },
    "required": ["value"]
  }
}
```

**Set defaults in the plugin** for anything the model leaves out.

**Use a script plugin** if it must read a configuration file. It has the user's rights. See [Script plugins](../../plugins/build-types.md#script-plugins).

## What the user can already change

These settings apply to any tool, and the Engine uses them itself. A plugin never sees them:

| Setting | Changed with |
|---|---|
| On or off | `cluaiz plugin config`, CEP `tools.set_settings` |
| `security_mode` | Same |
| `execution_mode` | Same |

## The planned way

A plugin would read Engine values through a CEP request, after declaring the method in `package.json`. See [CEP and plugins](03-cep-architecture.md).
