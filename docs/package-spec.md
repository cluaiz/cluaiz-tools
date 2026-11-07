# Package specification (`package.json`)

Every published tool has a `package.json`. This page is the owner of its fields. The "Read by" column says who uses a field today, so you can tell which fields have an effect.

## Minimal examples

**Skill**

```json
{
  "id": "git-assistant",
  "name": "Git Assistant",
  "category": "engineering",
  "hub_type": "skill",
  "description": "Helps with commit messages, branches and merge conflicts. Use for Git workflow questions.",
  "license": "Apache-2.0",
  "icon": "assets/icon.svg",
  "triggers": { "semantic": ["commit message", "merge conflict"] },
  "latest_version": "1.0.0",
  "versions": {
    "1.0.0": {
      "updated_at": "2026-11-06T12:00:00Z",
      "files": { "skill": "SKILL.md", "icon": "assets/icon.svg" }
    }
  }
}
```

**WebAssembly plugin**

```json
{
  "id": "math",
  "name": "Math",
  "category": "utility",
  "hub_type": "plugin",
  "build_type": "wasm",
  "description": "Evaluates math expressions exactly. Use for any calculation.",
  "license": "Apache-2.0",
  "icon": "assets/icon.svg",
  "triggers": ["calculate", "solve equation", "trigonometry"],
  "functions": [
    {
      "name": "math_evaluate",
      "description": "Evaluate a math expression exactly.",
      "parameters": {
        "type": "object",
        "properties": { "expression": { "type": "string" } },
        "required": ["expression"]
      }
    }
  ],
  "latest_version": "1.0.0",
  "versions": {
    "1.0.0": {
      "updated_at": "2026-11-06T12:00:00Z",
      "builds_os": ["wasm"],
      "files": { "skill": "SKILL.md", "icon": "assets/icon.svg" }
    }
  }
}
```

**MCP server**

```json
{
  "id": "git",
  "name": "Git",
  "category": "mcp",
  "hub_type": "mcp",
  "description": "Inspects Git repositories: status, log and diff.",
  "license": "MIT",
  "icon": "assets/icon.svg",
  "triggers": ["git log", "git diff"],
  "execution": { "command": "uvx", "args": ["mcp-server-git"] },
  "latest_version": "1.0.0",
  "versions": {
    "1.0.0": {
      "updated_at": "2026-11-06T12:00:00Z",
      "files": { "icon": "assets/icon.svg" }
    }
  }
}
```

## Identity

| Field | Required | Read by | Meaning |
|---|---|---|---|
| `id` | Yes | Engine, CI, App | Lowercase kebab-case. Equal to the folder name and the `family.json` key |
| `name` | Yes | Engine, App | Display name |
| `title` | No | Engine, App | Short line for cards. The Engine also matches on it |
| `description` | Yes | Engine, App | What it does and when to use it. The main thing the model and the matcher see |
| `category` | Yes | App, website | A grouping such as `utility` or `engineering`. The Engine ignores it |
| `hub_type` | Yes | App, website | `skill`, `plugin`, `mcp` or `extension`. The Engine uses the family instead |
| `author` | No | App, website | `{ "name", "url" }` |
| `license` | Yes | App, website | A license identifier |
| `tags` | No | Engine for skills, App | Skills are also matched on these |
| `documentation` | No | App, website | Link to documentation |

## Icon (required)

| Field | Required | Read by | Meaning |
|---|---|---|---|
| `icon` | Yes | App, website | Always `assets/icon.svg` |
| `assets` | No | App, website | Optional `banner` and `screenshots` |

Every tool must include `assets/icon.svg`: a pure, standalone SVG document with a 24 by 24 `viewBox`, at most 50 KB. Other image formats are not accepted. The Engine does not check this. Reviewers do.

## What the model sees

| Field | Types | Read by | Meaning |
|---|---|---|---|
| `functions` | Plugin, skill | Engine | Functions the model can call: `name`, `description`, `parameters` (JSON schema). `tools` is accepted too |
| `triggers` | Plugin, MCP | Engine | A plain list of words and phrases that match this tool |
| `discovery.semantic_triggers` | Plugin, MCP | Engine | Same as above, the older form |
| `triggers.semantic` | Skill | Engine | A list under an object, for skills |

**Plugins and MCP servers must use the plain list.** A `triggers.semantic` object or `tags` has no effect for them, and neither do triggers in `SKILL.md` once there is a `package.json`. See [matching](architecture/engine-integration.md#matching-which-tools-are-offered).

An MCP server has no `functions` here: the Engine gets the list from the server.

## Running

| Field | Types | Read by | Meaning |
|---|---|---|---|
| `build_type` | Plugin | Engine, CI | `wasm`, `binary` or `none`. See [Build types](plugins/build-types.md) |
| `github_action` | All | CI | `false` makes CI skip this package |
| `execution` | MCP | Engine | `command`, `args`, `env`: how to start the server |
| `execution` | Plugin, skill | Engine | A script command. A different shape: see below |

`execution` differs by type.

**MCP server** runs a program with arguments:

```json
"execution": { "command": "npx", "args": ["-y", "some-server"], "env": { "MODE": "fast" } }
```

The values in `env` are literal. `${NAME}` is not expanded.

**Plugin or skill** runs a command line with the input in a file. `args` is not read:

```json
"execution": {
  "type": "script_runner",
  "scratch_dir": "scratch",
  "default_language": "python",
  "command": "python \"{tool_dir}/scripts/run.py\" \"{file}\""
}
```

| Field | Meaning |
|---|---|
| `type` | `script_runner`, the only type |
| `command` | The command line. `{tool_dir}` is the installed folder, `{file}` holds the input, `{bin}` is a path for a compiled output |
| `languages` | Instead of `command`: a map such as `{ "python": { "cmd": "python \"{file}\"", "ext": "py" } }`, chosen by the input's `language` field |
| `default_language` | The language used when the input names none |
| `scratch_dir` | Folder in the workspace for temporary files. It defaults to `scratch` |

## Trust and behaviour

These set first values. The user can change them later.

| Field | Read by | Meaning |
|---|---|---|
| `security_mode` | Engine | `inherit` or `require_approval`. `always_allow` is reduced to `require_approval` |
| `execution_mode` | Engine | `auto`, `manual` or `confirm_destructive` |
| `default_turns` | Engine | Answers a loaded tool stays when unused. `-1` means no limit |
| `permissions.network_access` | Engine | `true` lets a WebAssembly plugin use `http_fetch`, and records a network permission, which leads to approval in `sandboxed` mode |
| `expose` | Engine | For an MCP package: the only tools the model gets, each with the only inputs it may set, such as `{ "open": ["url"] }`. Other calls are refused. Without it, everything the server offers is offered |
| `permissions.secrets` | Engine | A setting's name and the one host it may be sent to |
| `permissions.user_urls` | Engine | Settings that hold an address the user chose |
| `permissions.file_system` | Engine, plugins | Anything but `none` records a file permission, the same way |
| `permissions.max_memory_mb` | Engine | Lowers the memory limit of a WebAssembly plugin |
| `permissions.max_cpu_time_ms` | Nothing yet | **Planned** |

`permissions` tells the Engine when to ask. For a WebAssembly plugin, the only access it grants is the network rules above. See [Network access](plugins/wasm-abi-guide.md#network-access).

## Dependencies

```json
"dependencies": {
  "plugins": { "text": { "version": "^1.0.0", "url": "https://raw.githubusercontent.com/cluaiz/cluaiz-tools/main/plugins/text/package.json" } },
  "mcp": {},
  "skills": {}
}
```

**Status: Planned.** The Engine does not install these or check them. Today this is a record for people and for the catalog. Say in your `SKILL.md` what the user must install. See [Dependencies](plugins/overview.md#dependencies).

The planned version ranges are `^1.2.3`, `~1.2.3`, an exact `1.2.3` and `>=1.2.0`. A lockfile is planned too and does not exist.

## Versions

| Field | Required | Read by | Meaning |
|---|---|---|---|
| `latest_version` | Yes | Engine, CI | The version to install by default |
| `versions.<v>.updated_at` | Yes | App, website | Build time |
| `versions.<v>.changelog` | No | CI | Text for the release page |
| `versions.<v>.builds_os` | Binary | CI | `windows`, `macos`, `linux`; `wasm` for a WebAssembly plugin |
| `versions.<v>.os.<system>` | Binary | Engine | Full address of the binary for `windows`, `macos`, `linux` |
| `versions.<v>.files.file_directory` | No | Engine | Full address of the zip. It defaults to the standard release address |
| `versions.<v>.files.*` | Yes, for a skill or MCP server without a release | Engine | Paths of files in the folder, as names or lists. `icon` and `skill` are common. Installed one by one when no zip exists |
| `versions.<v>.command`, `args`, `env` | No | Engine, MCP | An older place for `execution`. Use `execution` |

Standard release addresses are in [Build pipeline](architecture/build-pipeline.md#release-names). Paths must stay inside the package: no leading `/` is needed, and `..` is refused.

`versions.<v>.files.binary` is not read: the program is always `<id>.wasm` after install.

## App mini-app metadata — private P6 contract

App extension runtime metadata has a separate private schema. The App reference is [Mini-app contract](../../cluaiz-app/src/extensions/CONTRACT.md); its validator is `cluaiz-app/src/extensions/contract.ts`.

An external extension release can declare its complete contract under `versions[exactVersion].app`. Its canonical `publisher.app-name`, exact version and package-relative `runtime.entry` must match the explicit host identity mapping, selected version and `files.entry`. Short catalogue IDs remain catalogue identities; author names and repository URLs do not assign a publisher or permission. A catalogue entry cannot claim compiled `bundled` provenance.

P6 only validates supplied metadata and reports host compatibility. Existing extension descriptors without `.app` return an incomplete-metadata error; they are not automatically migrated, installed or launched. The current App advertises bundled workspace modules only. External webview/native/WASM execution, package integrity/install/update and Engine-scoped authority remain later implementation slices. Existing execution permission flags do not become App SDK grants.
