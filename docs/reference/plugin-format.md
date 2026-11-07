# Plugin format

A plugin is a folder with `package.json` and a program. For what a plugin is and how to choose a kind, see [Plugins](../plugins/overview.md) and [Build types](../plugins/build-types.md).

## Published folder

```
plugins/<id>/
├── package.json        Required
├── assets/icon.svg     Required
├── SKILL.md            Recommended
├── references/         Optional, text read on demand
├── scripts/            For a script plugin
├── Cargo.toml          For a built plugin
└── src/                For a built plugin
```

## Installed folder

```
~/.cluaiz/tools/plugins/<id>/
├── package.json
├── assets/icon.svg
├── SKILL.md
├── references/
├── scripts/
└── <id>.wasm           For a WebAssembly plugin
```

Source files are not installed. Only `package.json`, `README.md`, `SKILL.md`, `scripts/`, `references/` and `assets/` go into the release zip.

## Fields

See [Plugin package fields](../plugins/manifest-schema.md) and the [package specification](../package-spec.md).

## How the Engine finds the program

| Order | Condition | Action |
|---|---|---|
| 1 | `execution` in `package.json` | Runs the script command |
| 2 | A program file in the folder | Runs the first one found: `.wasm` in the sandbox; other files as a process |

The folder should have one program. Files with these extensions count: `.wasm`, `.exe`, `.dll`, `.so`, `.py`, `.js`. Put scripts in `scripts/`, which is not scanned.

| File | Result of a call |
|---|---|
| `.wasm` | **Works** |
| `.exe` | Runs with the input as its first argument, and its output is the answer. **Works on Windows** |
| `.dll`, `.so`, `.dylib` | Fails |
| `.py`, `.js` without `execution` | Fails on Windows |

## Interface

See [WebAssembly interface](../plugins/wasm-abi-guide.md).
