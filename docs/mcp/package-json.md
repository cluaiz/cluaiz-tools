# MCP server `package.json`

Every MCP server package has a `package.json` at the root of its folder. The [package specification](../package-spec.md) owns every field; this page walks through one complete, real MCP manifest. The fields that a server most often gets wrong are in [MCP package fields](manifest-schema.md).

## Complete example

This is the manifest of the `browser` MCP server. It uses every part an MCP package can have: permissions, a setting, a list of exposed tools and the command that starts the server.

```json
{
  "id": "browser",
  "name": "Browser",
  "category": "mcp",
  "hub_type": "mcp",
  "description": "Opens web pages in a real browser, reads them, clicks and types, and tests a website: console errors, failed requests and accessibility. Needs Node. Use when a page needs JavaScript, clicks or forms, or to test a website.",
  "author": {
    "name": "Vercel Labs",
    "url": "https://github.com/vercel-labs/agent-browser"
  },
  "license": "Apache-2.0",
  "icon": "assets/icon.svg",
  "tags": [
    "Browser",
    "Web",
    "Testing"
  ],
  "triggers": [
    "open this website",
    "use the browser",
    "click on the page",
    "fill this form",
    "test my website",
    "take a screenshot of the page",
    "open localhost",
    "find errors on my site",
    "check console errors",
    "accessibility check",
    "website",
    "test the website",
    "click",
    "button",
    "console",
    "console errors",
    "script errors",
    "failed requests",
    "form",
    "login page",
    "screenshot"
  ],
  "dependencies": {
    "plugins": {},
    "mcp": {},
    "skills": {}
  },
  "permissions": {
    "network": {
      "info": "Opens the web pages that are chosen, including local addresses",
      "allow": true,
      "access": "full"
    },
    "programs": {
      "info": "Starts a Chromium browser installed on this computer, such as Chrome, Edge or Brave",
      "allow": false,
      "run": [
        "chrome",
        "msedge",
        "brave",
        "chromium"
      ]
    },
    "files": {
      "info": "Saves a screenshot to the path it is given",
      "allow": false,
      "can": [
        "create"
      ],
      "where": "workspace"
    }
  },
  "env": {
    "AGENT_BROWSER_HEADED": {
      "info": "Set to 1 to show the browser window",
      "need": "optional"
    }
  },
  "expose": {
    "agent_browser_open": [
      "url",
      "headed"
    ],
    "agent_browser_snapshot": [
      "interactive",
      "compact"
    ],
    "agent_browser_read": [],
    "agent_browser_click": [
      "selector"
    ],
    "agent_browser_fill": [
      "selector",
      "text"
    ],
    "agent_browser_press": [
      "key"
    ],
    "agent_browser_select": [
      "selector",
      "values"
    ],
    "agent_browser_scroll": [
      "direction",
      "amount"
    ],
    "agent_browser_wait_for_text": [
      "text"
    ],
    "agent_browser_get_text": [
      "selector"
    ],
    "agent_browser_get_url": [],
    "agent_browser_screenshot": [
      "path",
      "fullPage"
    ],
    "agent_browser_back": [],
    "agent_browser_close": [],
    "agent_browser_console": [
      "clear"
    ],
    "agent_browser_errors": [
      "clear"
    ],
    "agent_browser_network_requests": [
      "status"
    ],
    "agent_browser_a11y": [
      "selector",
      "tags"
    ]
  },
  "execution": {
    "command": "npx",
    "args": [
      "-y",
      "agent-browser@0.41.2",
      "mcp",
      "--tools",
      "core,debug,network"
    ],
    "env": {
      "AGENT_BROWSER_CONTENT_BOUNDARIES": "1",
      "AGENT_BROWSER_MAX_OUTPUT": "20000",
      "AGENT_BROWSER_IDLE_TIMEOUT_MS": "300000"
    }
  },
  "latest_version": "1.0.0",
  "versions": {
    "1.0.0": {
      "updated_at": "2026-11-10T04:30:48Z",
      "command": "npx",
      "args": [
        "-y",
        "agent-browser@0.41.2",
        "mcp",
        "--tools",
        "core,debug,network"
      ],
      "env": {
        "AGENT_BROWSER_CONTENT_BOUNDARIES": "1",
        "AGENT_BROWSER_MAX_OUTPUT": "20000",
        "AGENT_BROWSER_IDLE_TIMEOUT_MS": "300000"
      },
      "files": {
        "readme": "README.md",
        "icon": "assets/icon.svg",
        "skill": "SKILL.md"
      }
    }
  }
}
```

## The fields

| Field | Required | Meaning |
|---|---|---|
| `id` | Yes | Lowercase kebab-case. Equal to the folder name and the `family.json` key |
| `name` | Yes | Display name |
| `category` | Yes | `mcp` |
| `hub_type` | Yes | `mcp` |
| `description` | Yes | What it does, what it needs (such as Node), and when to use it ("Use when ...") |
| `author` | No | The server's maker: `name` and `url` |
| `license` | Yes | The server's license |
| `icon` | Yes | Always `assets/icon.svg` |
| `tags` | No | Shown in the App. They do not match an MCP server |
| `triggers` | Yes | A plain list of words and phrases that offer this server to the model |
| `dependencies` | Yes | Other tools, with `need` and `info`. See [Dependencies](#dependencies) |
| `permissions` | Yes | What the server may reach. See [Permissions](#permissions) |
| `env` | No | Settings and keys the server reads. See [Env](#env) |
| `expose` | No | The only tools the model gets, each with the only inputs it may set. Other calls are refused. Without it, every tool the server offers is offered |
| `execution.command` | Yes | The program that starts the server, on the user's `PATH`. Pin the package version in `args`, such as `agent-browser@0.41.2` |
| `execution.args` | Yes | Its arguments, as a list |
| `execution.env` | No | Fixed environment values, used as written. `${NAME}` is not expanded. A value the user gives goes in `env` |
| `latest_version` | Yes | The version installed by default |
| `versions.<v>.updated_at` | Yes | Build time |
| `versions.<v>.files` | Yes | Every file to install: there is no release zip for an MCP package |

There is no `functions` field: the Engine gets the tool list from the server. `versions.<v>.command`, `args` and `env` are an older place for `execution`, read only when `execution` is missing.

## Permissions

`permissions` lists what the tool may reach. The user reads each `info` at install and can allow or refuse each permission later with `cluaiz mcp permission browser [<permission> on|off]`, the App's tool page, or `tools.set_settings`.

| Field | Meaning |
|---|---|
| The key | A type from the fixed list: `network`, `files`, `programs`, `system`, `location`, `media`, `clipboard`, `notifications`, `native`, `engine` |
| `info` | One sentence the user reads: what the tool does with it |
| `allow` | Allowed at install or not. The Engine decides what is sensitive, and a sensitive permission is never allowed until the user allows it |
| The type's options | Fixed per type, such as `access` and `hosts` for `network`, or `can` and `where` for `files` |

Every type, its options and their values are in [Permissions](../reference/permissions.md). The date, time, time zone and the operating system's name are always available and need no permission.

`browser` uses `full` access, because it opens any page. A server that talks to one service uses `strict`, like `context7`:

```json
{
  "network": {
    "info": "Sends library names and questions to the Context7 service",
    "allow": true,
    "access": "strict",
    "hosts": [
      "context7.com"
    ]
  }
}
```

An MCP server is a normal program, so the Engine cannot block what it reaches. While a permission is refused, the Engine asks the user before each call of the server instead. Declare every type the server really uses, as `browser` declares `programs` (it starts Chrome) and `files` with `create` (it saves screenshots).

## Env

`env` names every setting the tool reads, keys among them. It is not a permission: a key says who the user is to a service, a permission says what the tool may reach.

| Field | Meaning |
|---|---|
| The name | Upper case letters, digits and `_`, as for an environment variable |
| `info` | One sentence the user reads when asked for the value |
| `need` | `required`: the tool cannot work without it, and the user is asked at install. `optional` (the default): the tool works without it |

The Engine takes each value from the system environment variable of that name first. Otherwise the user sets it with `cluaiz mcp config browser <NAME>` (asked for hidden when the value is left out), the App, or `tools.set_settings` with `user_settings`. Values are never written into the tool's folder.

The server gets each value as an environment variable when the Engine starts it. A change applies the next time the server starts.

## Dependencies

`dependencies` lists other tools by kind: `plugins`, `mcp` and `skills`. Keep all three keys, empty when unused.

| Field | Meaning |
|---|---|
| `version` | A version range, such as `^1.0.0` |
| `url` | The tool's raw `package.json`: `https://raw.githubusercontent.com/cluaiz/cluaiz-tools/main/<kind>/<id>/package.json` |
| `need` | `required`: cannot work without it. `recommended`: works without it, better with it. `optional`: useful sometimes, only suggested |
| `info` | One sentence the user reads at install. Required for `required` and `recommended` |

What is installed without asking depends on the user's mode (`full_access`, `sandboxed`, `strict`): see [Installing by the user's mode](../package-spec.md#installing-by-the-users-mode). The tool must still work, with a clear message, when a `recommended` or `optional` one is missing.
