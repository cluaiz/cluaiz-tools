# Permissions

This page is the owner of the permission standard: the fixed list of permission types, the fixed options of each, and what the Engine does with them. [Package specification](../package-spec.md#permissions) shows where they go in `package.json`.

**Status:** this is the standard the Engine, `validate_tool` and the official tools are being changed to. Until then the Engine reads the older kinds (`clock`, `files_read`, `files_write`, `run_programs`).

## The rules

1. **The list is fixed.** The Engine owns the permission types and their options. A tool cannot invent a type or an option, because the Engine can only enforce what it knows. `validate_tool` refuses anything else.
2. **Nothing is given by default.** A tool gets only what it declares and the user allows.
3. **The Engine decides what is sensitive,** not the tool. A sensitive permission is never allowed until the user allows it, whatever the manifest says.
4. **The narrowest scope wins.** A tool asks for the least it needs: named hosts rather than any host, the workspace rather than the whole disk, named programs rather than any program.
5. **The user decides last.** The user can refuse any permission, or narrow its scope, at install or later. A refusal is kept when the tool updates.
6. **One line per permission.** Each permission is one object with `info`, `allow` and at most two options of its own.

This follows how other systems do it: Deno gives nothing by default and scopes each permission (`--allow-net=github.com`, `--allow-run=git`); Android lets the system, not the app, decide which permissions are dangerous; Chrome extensions take permission names only from a fixed list and list hosts separately.

## The shape

```json
"permissions": {
  "<type>": { "info": "What the tool does with it", "allow": true, "<option>": "<value>" }
}
```

Every type has these two fields:

| Field | Required | Values | Meaning |
|---|---|---|---|
| `info` | Yes | One sentence | What the tool does with it. The user reads this at install |
| `allow` | No | `true` or `false` | Allowed at install or not. Absent means `true`. A sensitive permission is always `false` until the user allows it |

`permissions` is `{}` when the tool needs none.

## The types

| Type | Covers | Options | Sensitive |
|---|---|---|---|
| [`network`](#network) | The internet | `access`, `hosts` | No |
| [`files`](#files) | Files and folders | `can`, `where` | By option |
| [`programs`](#programs) | Starting other programs | `run` | Yes |
| [`system`](#system) | Facts about this computer | `read` | By option |
| [`location`](#location) | Where the user is | `precision` | Yes |
| [`media`](#media) | Microphone, camera, screen | `use` | Yes |
| [`clipboard`](#clipboard) | The clipboard | `can` | By option |
| [`notifications`](#notifications) | Showing a notice to the user | None | No |
| [`native`](#native) | Loading native code into the Engine's process | None | Yes |
| [`engine`](#engine) | Engine methods through CEP | `methods` | By method |

### `network`

Requests to the internet.

| Option | Required | Values |
|---|---|---|
| `access` | Yes | `full`: any public host. `strict`: only the hosts in `hosts` |
| `hosts` | With `strict` | Host names, such as `api.example.com`, or `*.example.com` for its subdomains |

- Only `https`, and only public addresses. Local and private addresses are refused, except an address the user typed into one of the tool's [settings](../package-spec.md#env).
- Every redirect is checked again.
- The user can narrow `full` to `strict`, or widen `strict` to `full`.

```json
"network": { "info": "Reads public holidays", "allow": true, "access": "strict", "hosts": ["date.nager.at"] }
```

### `files`

Files and folders, as the four separate actions of create, read, update and delete.

| Option | Required | Values |
|---|---|---|
| `can` | Yes | A list of `create`, `read`, `update`, `delete` |
| `where` | Yes | `workspace`: the open folder. `home`: the user's home folder. `any`: anywhere the user's account can reach |

- Sensitive: everything except `read` in the `workspace`.
- A path outside `where` is refused, and so is a path that leaves it through `..` or a link.

```json
"files": { "info": "Saves the report in the workspace", "allow": true, "can": ["create", "read", "update"], "where": "workspace" }
```

### `programs`

Starting other programs.

| Option | Required | Values |
|---|---|---|
| `run` | Yes | Program names, such as `git` and `gh`, or `["*"]` for any program |

- Always sensitive. A started program has the user's rights, so `["*"]` should be rare.
- The name is the program, not a path. The Engine finds it on the user's `PATH`.

```json
"programs": { "info": "Runs git to read the changes under review", "allow": false, "run": ["git"] }
```

### `system`

Facts about this computer beyond the operating system's name.

| Option | Required | Values |
|---|---|---|
| `read` | Yes | A list of `os_version`, `cpu`, `memory`, `gpu`, `hostname`, `username` |

- Sensitive: `hostname` and `username`, because they identify the user.

### `location`

Where the user is, beyond the time zone.

| Option | Required | Values |
|---|---|---|
| `precision` | Yes | `approximate` (the city) or `precise` |

Always sensitive.

### `media`

Recording devices.

| Option | Required | Values |
|---|---|---|
| `use` | Yes | A list of `microphone`, `camera`, `screen` |

Always sensitive.

### `clipboard`

| Option | Required | Values |
|---|---|---|
| `can` | Yes | A list of `read`, `write` |

- Sensitive: `read`, because the clipboard often holds passwords.

### `notifications`

Showing a notice to the user, such as a reminder. No options. Not sensitive.

### `native`

Loading a native library (`.dll`, `.so`, `.dylib`) into the Engine's process, for a native binary plugin. No options. Always sensitive, because native code is outside any sandbox.

### `engine`

Calling Engine methods through CEP, such as `memory.search`.

| Option | Required | Values |
|---|---|---|
| `methods` | Yes | CEP method names |

- The Engine decides which methods are sensitive. `system.exec` is always refused.
- A script that calls the Engine's API at `/v1/cep`, as `deep-research` and `speech` do, declares the methods it uses.
- **Planned:** there is no host function for CEP in a WebAssembly plugin yet. See [From CEL to CEP](../architecture/cel-to-cep.md).

## Not permissions

These are always available, as in Deno and WASI, because reading them harms no one:

| Always available | Why |
|---|---|
| The date, time and time zone | Every program needs them, and they say nothing about the user beyond the time zone |
| Random numbers | Needed for ids and security |
| The operating system's name | Needed to choose the right behaviour |

Settings and keys are not permissions either. A key says who the user is to a service; a permission says what the tool may reach. Keys go in [`env`](../package-spec.md#env), and a tool gets only the names listed there.

## Limits

Limits are not permissions. The Engine sets the highest value; a tool may ask for less, never more.

| Limit | Engine's value | A tool may lower it with |
|---|---|---|
| Instructions per WebAssembly call (fuel) | 2,000,000,000 | Not yet |
| Memory per WebAssembly call | 256 MB | `limits.memory_mb` |
| Internet requests per call | 16 | Not yet |
| Time for a script call | 30 seconds | A `timeout` field in the input |

## What the Engine does

| Tool type | An allowed permission | A refused permission |
|---|---|---|
| WebAssembly plugin | The matching host function works | The host function answers `permission_off`, with the command that allows it. The Engine enforces this itself |
| Script, MCP server | The tool runs | The Engine asks the user before each call. These run as normal programs, so the Engine cannot block their access by itself |

A WebAssembly plugin can only reach what has a host function:

| Type | Host function | Status |
|---|---|---|
| `network` | `http_fetch` | **Works** |
| `files` | Planned | Planned |
| `system` | Planned | Planned |
| `notifications` | Planned | Planned |
| `location`, `media`, `clipboard`, `engine` | Planned | Planned |

A script or an MCP server can be held to its permissions only by the operating system's own sandbox (AppContainer on Windows, the App Sandbox on macOS, Landlock or bubblewrap on Linux). The Engine does not use one yet.

## At install and later

| | `full_access` | `sandboxed` | `strict` |
|---|---|---|---|
| Not sensitive | As `allow` says | As `allow` says | Asked |
| Sensitive | Asked | Asked | Asked |

The user changes a permission with `cluaiz <kind> permission <id> [<type> on|off]`, the App's tool page, or CEP `tools.set_settings`. The choice is kept in the Engine's `config/tool_permissions.json`, not in the tool's folder.

## On another operating system

Each type maps to the permission the operating system has, so Cluaiz can ask the system for exactly what a tool declared:

| Type | Android | iOS | Flatpak (Linux) |
|---|---|---|---|
| `network` | `INTERNET` | No permission | `--share=network` |
| `files` | Storage access | Files picker | `--filesystem=...` |
| `location` | `ACCESS_COARSE_LOCATION` or `ACCESS_FINE_LOCATION` | Location | Location portal |
| `media` | `RECORD_AUDIO`, `CAMERA` | Microphone, Camera | `--socket=pulseaudio`, `--device=...` |
| `notifications` | `POST_NOTIFICATIONS` | Notifications | Notification portal |
| `clipboard` | No permission | Paste prompt | Clipboard access |

## Removed

| Removed | Use |
|---|---|
| `clock` | Nothing: the time is always available |
| `files_read`, `files_write` | `files` with `can` and `where` |
| `run_programs` | `programs` with `run` |
| `network_access`, `file_system` | `network`, `files` |
| `secrets`, `user_urls` | [`env`](../package-spec.md#env) |
| `why`, `default` | `info`, `allow` |
| `max_memory_mb`, `max_cpu_time_ms` inside `permissions` | `limits` |

## Sources

- [Deno permissions](https://docs.deno.com/runtime/reference/permissions/)
- [Android runtime permissions](https://source.android.com/docs/core/permissions/runtime_perms)
- [Chrome extensions: declare permissions](https://developer.chrome.com/docs/extensions/develop/concepts/declare-permissions)
- [Flatpak sandbox permissions](https://docs.flatpak.org/en/latest/sandbox-permissions.html)
- [WASI interfaces](https://wasi.dev/interfaces)
