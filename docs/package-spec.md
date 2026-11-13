# Package specification (`package.json`)

Every published tool has a `package.json`. This page is the owner of its fields. The "Read by" column says who uses a field today, so you can tell which fields have an effect.

## Minimal examples

These are the fewest fields each type needs. Complete, real manifests with permissions, settings and dependencies, field by field: [plugin](plugins/package-json.md) (`web-search`), [MCP server](mcp/package-json.md) (`browser`), [skill](skills/package-json.md) (`deep-research`).

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
  "dependencies": { "plugins": {}, "mcp": {}, "skills": {} },
  "permissions": {},
  "latest_version": "1.0.0",
  "versions": {
    "1.0.0": {
      "updated_at": "2026-11-06T12:00:00Z",
      "files": { "readme": "README.md", "skill": "SKILL.md", "icon": "assets/icon.svg" }
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
  "dependencies": { "plugins": {}, "mcp": {}, "skills": {} },
  "permissions": {},
  "latest_version": "1.0.0",
  "versions": {
    "1.0.0": {
      "updated_at": "2026-11-06T12:00:00Z",
      "builds_os": ["wasm"],
      "files": { "readme": "README.md", "skill": "SKILL.md", "icon": "assets/icon.svg" }
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
  "execution": { "command": "uvx", "args": ["mcp-server-git@2026.11.4"] },
  "dependencies": { "plugins": {}, "mcp": {}, "skills": {} },
  "permissions": {
    "files": { "info": "Reads the repository and stages, commits, branches and merges in it", "allow": false, "can": ["create", "read", "update", "delete"], "where": "workspace" }
  },
  "latest_version": "1.0.0",
  "versions": {
    "1.0.0": {
      "updated_at": "2026-11-06T12:00:00Z",
      "files": { "readme": "README.md", "skill": "SKILL.md", "icon": "assets/icon.svg" }
    }
  }
}
```

## Documentation (`README.md` vs `SKILL.md`)

Every published tool in `cluaiz-tools` (`skills/`, `plugins/` and `mcp/`) **must** include both files with distinct roles:

| File | Audience | Read by | Purpose |
|---|---|---|---|
| `README.md` | **Human users** | Website (`cluaiz.com/tools`), App catalog, GitHub | Explains what the tool does, its system architecture, a Mermaid flow diagram, inputs/outputs, prerequisites and usage examples. **Never loaded into an AI model's prompt.** |
| `SKILL.md` | **AI models** | Cluaiz Engine (`load_tool`) | Concise, step-by-step operational instructions loaded into the model's context window when the tool is triggered. |

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

The values in `execution.env` are literal. `${NAME}` is not expanded. Settings and keys the user gives go in the top-level [Env](#env).

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
| `command` | The command line. `{python}` is the Python found on this computer (`python` or `python3`), `{tool_dir}` the installed folder, `{file}` holds the input (the arguments as JSON, with the function's name in `_fn`), `{bin}` a path for a compiled output |
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
| `expose` | Engine | For an MCP package: the only tools the model gets, each with the only inputs it may set, such as `{ "open": ["url"] }`. Other calls are refused. Without it, everything the server offers is offered |
`permissions` is described in [Permissions](#permissions), the settings and keys a tool reads in [Env](#env), and `limits` in [Limits](#limits).

## Permissions

A tool lists what it needs to reach. The full standard, with every type, its options and their values, is in [Permissions](reference/permissions.md). In short:

```json
"permissions": {
  "network": { "info": "Reads public holidays", "allow": true, "access": "strict", "hosts": ["date.nager.at"] },
  "files": { "info": "Saves the report in the workspace", "allow": true, "can": ["create", "read", "update"], "where": "workspace" }
}
```

| Type | Options |
|---|---|
| `network` | `access`: `full` or `strict`; `hosts` with `strict` |
| `files` | `can`: `create`, `read`, `update`, `delete`; `where`: `workspace`, `home`, `any` |
| `programs` | `run`: program names, or `["*"]` |
| `system` | `read`: `os_version`, `cpu`, `memory`, `gpu`, `hostname`, `username` |
| `location` | `precision`: `approximate` or `precise` |
| `media` | `use`: `microphone`, `camera`, `screen` |
| `clipboard` | `can`: `read`, `write` |
| `notifications` | None |
| `native` | None |
| `engine` | `methods`: CEP method names. Planned |

Every type has `info` (required) and `allow` (`true` or `false`, absent means `true`). The Engine, not the tool, decides what is sensitive, and a sensitive permission is never allowed until the user allows it. `permissions` is `{}` when the tool needs none.

The date, time, time zone, random numbers and the operating system's name are always available and are not permissions. Memory and time limits go in `limits`, not here.

**Engine status:** the Engine is being changed to this standard; until then it reads the older kinds (`clock`, `files_read`, `files_write`, `run_programs`). See [Removed](reference/permissions.md#removed).

### Limits

```json
"limits": { "memory_mb": 64 }
```

| Field | Meaning |
|---|---|
| `memory_mb` | Lowers the memory of a WebAssembly call below the Engine's 256 MB |

A tool can only lower a limit. See [Limits](reference/permissions.md#limits).

## Env

The settings a tool reads, keys among them. They are not permissions: a key says who the user is to a service, while a permission says what the tool may reach.

```json
"env": {
  "TAVILY_API_KEY": { "info": "A Tavily key, for better search results", "need": "optional" },
  "OWN_SERVER_URL": { "info": "The address of your own server", "need": "optional" }
}
```

| Field | Required | Meaning |
|---|---|---|
| The name | Yes | Upper case letters, digits and `_`, as for an environment variable |
| `info` | Yes | One sentence the user reads when asked for it |
| `need` | No | `required`: the tool cannot work without it, and the user is asked at install. `optional` (the default): the tool works without it |

One rule for every tool type:

- **Where the value comes from:** the system environment variable of that name first, then the value the user saved in Cluaiz with `cluaiz <kind> config <id> <name>`, the App, or `tools.set_settings` (`user_settings`). Values are typed hidden and never written into the tool's folder.
- **What the tool gets:**
  - An MCP server or a skill's script gets the value as an environment variable.
  - A WebAssembly plugin never sees the value. It writes `{{secret:NAME}}` in a request header, or `{{url:NAME}}` at the start of a URL, and the Engine fills it in. This needs the `network` permission.
- **Where a value may go:** with `strict` access, only to the listed hosts. With `full` access, to any public host the tool reaches; the install plan tells the user this.
- **Local addresses:** the Engine refuses local and private addresses, except an address the user typed into one of these settings, such as their own server on `localhost`.
- **Missing:** a missing `required` value is asked for at install; the tool reports it when called. A missing `optional` value is skipped.

## Dependencies

```json
"dependencies": {
  "plugins": { "web-search": { "version": "^1.0.0", "url": "https://raw.githubusercontent.com/cluaiz/cluaiz-tools/main/plugins/web-search/package.json", "need": "required", "info": "Searches and reads web pages" } },
  "mcp": {},
  "skills": {}
}
```

Every entry has this one form: the key is the tool id, `version` is a version range, and `url` is the raw `package.json` of that tool in `cluaiz-tools` (`https://raw.githubusercontent.com/cluaiz/cluaiz-tools/main/<kind>/<id>/package.json`). A bare string such as `"plugins/web-search"` is not valid; `validate_tool` reports it. Keep all three keys `plugins`, `mcp` and `skills`, empty when unused.

| Field | Required | Meaning |
|---|---|---|
| `version` | Yes | A version range |
| `url` | Yes | The tool's raw `package.json` |
| `need` | No | `required`: the tool cannot work without it (the default when absent). `recommended`: it works without it, better with it. `optional`: useful sometimes; never installed on its own |
| `info` | For `required` and `recommended` | One sentence the user reads at install |

### Installing by the user's mode

Before installing, the Engine makes a plan: the tool, each dependency, and each permission. The user's mode decides what happens without asking:

| | `full_access` | `sandboxed` | `strict` |
|---|---|---|---|
| `required` | Installed | Installed, unless it needs a sensitive permission; then asked | Asked |
| `recommended` | Installed | Asked | Asked |
| `optional` | Suggested only | Suggested only | Suggested only |
| A permission that is not sensitive | As its `allow` says | As its `allow` says | Asked |
| A sensitive permission | Asked | Asked | Asked |

- Questions go to the Engine's approvals, so they look the same everywhere:
  - The CLI shows a checklist (`--yes` accepts all, `--no-recommended` takes only what is required).
  - Without a terminal, the CLI installs only what needs no answer and prints the commands for the rest.
  - The App shows a dialog, and chat shows an approval card.
  - API clients use `approvals.list`, `approvals.approve` and `approvals.reject`.
- A question with no answer within 10 minutes counts as no.
- If a `required` dependency is refused, nothing is installed, and the message gives the command to install both.

**Engine status:** the plan, the mode rules and asking work in the CLI and in CEP `tools.install` (through approvals). Tools already installed are kept as they are; a dependency that fails to download leaves the tool installed with an error naming what is missing. Version ranges and `url` are not checked; `extensions` are not installed. The App's dialog is not built yet; until then the App's install waits in `approvals.list`. See [Dependencies](plugins/overview.md#dependencies).

The version ranges planned for checking are `^1.2.3`, `~1.2.3`, an exact `1.2.3` and `>=1.2.0`. A lockfile is planned too and does not exist.

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
