# Plugins

A plugin is a program the Engine runs when the model calls one of its functions.

Each folder here is one plugin. `family.json` lists them.

```
math/
├── package.json     Identity, functions and how it is built
├── SKILL.md         Instructions for the model
├── assets/icon.svg  The icon
├── Cargo.toml       Source, for a built plugin
└── src/
```

| Kind | Status |
|---|---|
| WebAssembly (`build_type: "wasm"`) | Works |
| Script (`execution`) | Works |
| Native binary (`build_type: "binary"`) | Planned: the Engine cannot run these yet |

Full documentation:

- [Plugins](../docs/plugins/overview.md)
- [Build types](../docs/plugins/build-types.md)
- [Tutorial: first plugin](../docs/plugins/tutorial-first-plugin.md)
