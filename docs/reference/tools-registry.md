# Registry files

The formats of `registry.json` and `family.json`. [Registry and discovery](../architecture/registry-and-discovery.md) explains how they are used. `package.json` is in the [package specification](../package-spec.md).

## Layout

```
cluaiz-tools/
├── registry.json
├── skills/
│   ├── family.json
│   └── code-reviewer/
│       ├── package.json
│       ├── SKILL.md
│       └── assets/icon.svg
├── plugins/
│   ├── family.json
│   └── math/
│       ├── package.json
│       ├── SKILL.md
│       ├── assets/icon.svg
│       ├── Cargo.toml
│       └── src/
├── mcp/
│   ├── family.json
│   └── filesystem/
│       ├── package.json
│       └── assets/icon.svg
└── extensions/
    └── family.json
```

## `registry.json`

```json
{
  "version": "1.0.0",
  "routing": {
    "plugins": "plugins/family.json",
    "skills": "skills/family.json",
    "mcp": "mcp/family.json",
    "extensions": "extensions/family.json"
  }
}
```

| Field | Meaning |
|---|---|
| `version` | Version of this file's format |
| `routing` | Each family and the path to its `family.json` |

The Engine uses `plugins`, `skills` and `mcp`. `extensions` is experimental.

## `family.json`

```json
{
  "category": "plugins",
  "name": "Cluaiz Plugins",
  "items": {
    "math": { "type": "local", "path": "math/package.json" },
    "image-optimizer": {
      "type": "remote",
      "repo": "your-name/your-plugin",
      "branch": "main",
      "manifest_path": "package.json"
    }
  }
}
```

| Field | Meaning |
|---|---|
| `category` | The family |
| `name` | Display name |
| `items` | Each tool, by id |

### Item forms

| Form | Fields | Status |
|---|---|---|
| Short | `"math": "math/package.json"` | **Works** |
| Local | `type: "local"`, `path` | **Works** |
| Remote | `type: "remote"`, `repo`, `branch`, `manifest_path` | **Planned**. See [Tools from other repositories](../architecture/decoupled-ecosystem.md) |

`path` is relative to the family's folder.

## Icon

Every tool must have `assets/icon.svg`. See [Package specification](../package-spec.md#icon-required).

## Pointing the Engine at another registry

The default is `https://raw.githubusercontent.com/cluaiz/cluaiz-tools/main/registry.json`. A `package.json` in the Engine's config folder can set another address, which is useful for testing a registry fork.
