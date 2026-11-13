# Package and local registry

Two JSON files describe a tool. They have different owners and different purposes.

| | `package.json` | `tools_registry.json` |
|---|---|---|
| Describes | A tool: what it is and how to get it | This machine: what is installed and how it is set |
| Written by | The tool's author | The Engine |
| Lives | In the tool's folder | `~/.cluaiz/config/` |
| Changes when | The author releases | The user installs, removes or changes a setting |

The published `registry.json` is a third thing: an index of families. See [Registry and discovery](registry-and-discovery.md).

Two more files in `~/.cluaiz/config/` hold the user's answers to what a `package.json` asks for. Neither is in the tool's folder, so an update keeps them:

| File | Holds | Set with |
|---|---|---|
| `tool_permissions.json` | Each tool's permissions the user allowed or refused | `cluaiz <type> permission <id> <permission> on\|off`, `tools.set_settings` (`permissions`) |
| `tool_settings.json` | Values for names in a tool's `env` that are not in the system environment | `cluaiz <type> config <id> <NAME>`, `tools.set_settings` (`user_settings`) |

## `tools_registry.json`

The Engine builds one entry from each folder in `~/.cluaiz/tools/`:

```json
{
  "installed_tools": {
    "math": {
      "id": "math",
      "name": "math",
      "category": "plugin",
      "version": "1.0.0",
      "description": "...",
      "local_dir": "C:/Users/me/.cluaiz/tools/plugins/math",
      "binary_path": "C:/Users/me/.cluaiz/tools/plugins/math/math.wasm",
      "enabled": true,
      "security_mode": "inherit",
      "execution_mode": "auto",
      "default_turns": -1,
      "permissions": [],
      "semantic_triggers": [],
      "trust_annotations": false
    }
  }
}
```

| From the tool's files, refreshed on each scan | Kept from the user |
|---|---|
| `name`, `version`, `description`, `semantic_triggers`, `permissions`, `binary_path`, `local_dir` | `enabled`, `security_mode`, `execution_mode`, `default_turns`, `trust_annotations` |

The manifest gives the first values of `security_mode`, `execution_mode` and `default_turns` when the tool first appears.

`permissions` here holds tags made from the kinds the manifest declares (`net:fetch`, `fs:read`, `fs:write`, `exec:programs`), used for the approval decision.

Do not edit these files by hand. Use the commands above or the CEP method `tools.set_settings`.

## How the Engine reads a package

| Type | Reads |
|---|---|
| Skill | `SKILL.md` frontmatter first, then `package.json` for anything missing |
| Plugin | `package.json`. It reads the frontmatter of `SKILL.md` only when there is no `package.json` |
| MCP server | `package.json` |

At run time the `SKILL.md` body and `functions` are read from the installed folder, so a change to those files applies at once.
