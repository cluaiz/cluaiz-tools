# Plugins

A plugin is a program the Engine runs when the model calls one of its functions. Its purpose is to give the model exact results and real abilities, and to work immediately after install.

The usual case is a WebAssembly plugin. It is one file for every operating system, needs nothing else on the machine, and runs inside the Engine in a closed sandbox.

## What a plugin contains

| Item | Required | Purpose |
|---|---|---|
| `package.json` | Yes | Identity, how to get it, and the functions it offers. See [Package fields](manifest-schema.md) |
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
3. It runs the program, passing the function's arguments as JSON text.
4. The result goes back to the model.

**The program is not told which function was called.** To offer several actions, either give one function an `action` field or publish several plugins. The `text` plugin uses `action`.

The WebAssembly sandbox has no access to files, network or other programs. It has two host functions: the time and the operating system. See [WebAssembly interface](wasm-abi-guide.md).

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

A package can record what it needs under `dependencies`. See [Package specification](../package-spec.md#dependencies).

**Status: Planned. The Engine does not install or check these today.**

| Situation | What happens today |
|---|---|
| A plugin lists a dependency | Nothing. The tool installs alone |
| A dependency is missing | The model finds the tool is not available when it tries |
| Two tools need the same dependency | Each user installs it once by hand |

What authors should do today:

1. Record real dependencies in `dependencies`. Do not list things the tool does not use.
2. Say in `SKILL.md` and in the catalog description what must be installed.
3. Have the tool work, with a clear message, if an optional one is missing.

The planned behaviour: on install, read `dependencies`, install what is missing from its address, and stop with a clear error if a range cannot be met.

A separate, working case: when an MCP or tool folder lists npm packages as plain text versions (`"name": "1.2.3"`), or has `requirements.txt`, the Engine prepares a private runtime for it. These are language packages, and have nothing to do with Cluaiz tool dependencies.

## Publishing and testing

- [Tutorial: first plugin](tutorial-first-plugin.md)
- [Local testing](../tutorials/plugins/04-local-testing.md)
- [Build pipeline](../architecture/build-pipeline.md)
