---
title: "Settings in plugins"
description: "How a plugin reads keys and other settings the user gives."
category: "Tutorials"
---

# 2. Settings in plugins

A plugin gets two kinds of input: the function's arguments, which the model fills in, and settings, which the user gives once, such as an API key or the address of their own server.

## Arguments

Anything that changes from call to call is an argument, with a default in the plugin for anything the model leaves out:

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

## Settings: `env`

A setting is named in the top-level `env` of `package.json`, with `info` the user reads and `need`:

```json
"permissions": {
  "network": { "info": "Searches the example service", "allow": true, "access": "strict", "hosts": ["api.example.com"] }
},
"env": {
  "EXAMPLE_API_KEY": { "info": "Your key for the example service", "need": "required" },
  "OWN_SERVER_URL": { "info": "The address of your own server", "need": "optional" }
}
```

The value comes from the system environment variable of that name first. Otherwise the user sets it once:

```bash
cluaiz plugin config my-plugin EXAMPLE_API_KEY
cluaiz plugin config my-plugin OWN_SERVER_URL http://localhost:8080
```

Left out on the command line, a value is asked for and does not show on screen. An app uses `tools.set_settings` with `user_settings`. Values are kept in the Engine's `config/tool_settings.json`, never in the plugin's folder.

## Using a setting

| Plugin type | How it gets the value |
|---|---|
| WebAssembly | It never sees it. It writes `{{secret:EXAMPLE_API_KEY}}` in a request header, or starts a URL with `{{url:OWN_SERVER_URL}}`, and the Engine fills it in during `http_fetch`. See [Network access](../../plugins/wasm-abi-guide.md#network-access) |
| Script | As an environment variable of the same name |

A name not listed in `env` is never filled in. A missing `required` value is asked for at install; when it is still missing, `http_fetch` answers `missing_setting` and the plugin should tell the user the command above.

## What the user can also change

These apply to any tool, and the Engine uses them itself. A plugin never sees them:

| Setting | Changed with |
|---|---|
| On or off, `security_mode`, `execution_mode` | `cluaiz plugin config`, CEP `tools.set_settings` |
| Each permission, allowed or refused | `cluaiz plugin permission <id> <permission> on\|off`, CEP `tools.set_settings` |
