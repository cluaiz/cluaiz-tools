# Plugins

A plugin is a program the Engine runs when the model calls one of its functions. Its purpose is to give the model exact results and real abilities, and to work immediately after install.

The usual case is a WebAssembly plugin. It is one file for every operating system, needs nothing else on the machine, and runs inside the Engine in a closed sandbox.

## What a plugin contains

| Item | Required | Purpose |
|---|---|---|
| `package.json` | Yes | Identity, functions, permissions, settings (`env`) and dependencies. See [Plugin package.json](package-json.md) and [Package fields](manifest-schema.md) |
| `assets/icon.svg` | Yes | The icon. See [Package specification](../package-spec.md#icon-required) |
| A program | Yes | `<id>.wasm` from CI, or scripts run through `execution` |
| `SKILL.md` | Recommended | Instructions the model gets when it loads the plugin |
| `references/` | No | Text files the model can read on demand |
| `src/`, `Cargo.toml` | For a built plugin | Source code. Not installed on the user's machine |

Published folder and installed folder are different:

```
plugins/math/                    ~/.cluaiz/tools/plugins/math/
├── package.json                 ├── package.json
├── SKILL.md                     ├── SKILL.md
├── assets/icon.svg              ├── assets/icon.svg
├── Cargo.toml                   └── math.wasm
└── src/lib.rs
```

## Plugin kinds

| Kind | When | Status |
|---|---|---|
| WebAssembly | It computes from its input only | **Works** |
| Script | It needs a language runtime or files, and no compile step | **Works** |
| Native binary | Heavy native work | **Planned** |

[Build types](build-types.md) explains how to choose and when anything gets built.

## How it runs

1. The model calls a function listed in `functions`.
2. The Engine finds the plugin that owns the function and asks for approval if needed.
3. It runs the program, passing the function's arguments as JSON text, with the permissions the user allowed.
4. The result goes back to the model.

**A WebAssembly program is not told which function was called.** It tells them apart by their arguments, as `time` does, or by a field such as `action`, as `text` does. A script gets the function's name in `_fn`.

The WebAssembly sandbox has no access to files or other programs. It can call the Engine's host functions: the date, time, time zone and operating system's name, which are always available, and `http_fetch` for the internet, with the `network` permission. See [WebAssembly interface](wasm-abi-guide.md).

## Permissions and settings

A plugin declares every permission it uses in `permissions`, each with `info` and `allow`, and every key or setting it reads in `env`. The user sees both at install and can refuse a permission later. For a WebAssembly plugin the Engine enforces a refused permission itself; for a script it asks before each call. See [Plugin package.json](package-json.md#permissions) and [Package specification](../package-spec.md#permissions).

## What can be inside one plugin

| Question | Answer | Status |
|---|---|---|
| Several functions? | Yes. List them under `functions`. They share one program, as above | **Works** |
| A `SKILL.md`? | Yes. It teaches the model this plugin | **Works** |
| Reference files? | Yes | **Works** |
| Another plugin? | No. One package installs into one folder with one program | Not supported |
| An MCP server? | No. It would not be started. Publish it as its own MCP package | Not supported |
| A separate skill to register? | No. Only the `SKILL.md` at its root is used. Publish it as its own skill | Not supported |

To combine tools today, publish each one separately and describe the combination in a skill. The model then uses them together:

```markdown
To summarise a meeting:
1. Call `audio_transcribe` from the `voice` plugin.
2. Call `git_log` from the `git` MCP server if the meeting mentions a release.
```

## Dependencies

A package lists the tools it needs under `dependencies`, each marked `required`, `recommended` or `optional`, with an `info` the user reads. What is installed without asking depends on the user's mode (`full_access`, `sandboxed`, `strict`); the rest is asked for at install. The rules are in [Package specification](../package-spec.md#dependencies).

What authors should do:

1. List only real dependencies, with the right `need`: `required` only when the tool cannot work without it.
2. Write an `info` the user understands.
3. Have the tool work, with a clear message, when a `recommended` or `optional` one is missing.

A separate, working case: when an MCP or tool folder lists npm packages as plain text versions (`"name": "1.2.3"`), or has `requirements.txt`, the Engine prepares a private runtime for it. These are language packages, and have nothing to do with Cluaiz tool dependencies.

## Publishing and testing

- [Tutorial: first plugin](tutorial-first-plugin.md)
- [Local testing](../tutorials/plugins/04-local-testing.md)
- [Build pipeline](../architecture/build-pipeline.md)
