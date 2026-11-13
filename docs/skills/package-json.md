# Skill `package.json`

Every skill has a `package.json` next to its `SKILL.md`. The [package specification](../package-spec.md) owns every field; this page walks through one complete, real skill manifest. The fields that a skill most often gets wrong are in [Skill package fields](manifest-schema.md), and `SKILL.md` itself is in [Skill format](../reference/skill-format.md).

## Complete example

This is the manifest of the `deep-research` skill. It uses every part a skill can have: functions run by a script, permissions and dependencies. A skill with instructions only leaves out `execution`, `functions` and `permissions` other than `{}`.

```json
{
  "id": "deep-research",
  "name": "Deep Research",
  "title": "Multi-step web research with disk notes and cited Markdown reports",
  "category": "research",
  "hub_type": "skill",
  "description": "Plans a research question, searches and reads web pages onto disk, checks citations, and writes a cited Markdown (.md) report in the workspace. Use for deep research, detailed comparisons, or a cited report saved as a file.",
  "author": {
    "name": "Cluaiz",
    "url": "https://github.com/cluaiz"
  },
  "license": "Apache-2.0",
  "icon": "assets/icon.svg",
  "tags": [
    "Research",
    "Deep Research",
    "Citations",
    "Report",
    "Markdown"
  ],
  "triggers": {
    "semantic": [
      "deep research",
      "research report",
      "write a report",
      "investigate",
      "literature review",
      "compare in detail",
      "research and cite",
      "detailed research",
      "comprehensive report",
      "research this topic",
      "save research report"
    ]
  },
  "dependencies": {
    "plugins": {
      "web-search": {
        "version": "^1.0.0",
        "url": "https://raw.githubusercontent.com/cluaiz/cluaiz-tools/main/plugins/web-search/package.json",
        "need": "recommended",
        "info": "Searches with more providers and the user's keys; without it the skill searches DuckDuckGo itself"
      }
    },
    "mcp": {
      "browser": {
        "version": "^1.0.0",
        "url": "https://raw.githubusercontent.com/cluaiz/cluaiz-tools/main/mcp/browser/package.json",
        "need": "optional",
        "info": "Searches and reads pages in a real browser when the search services fail or are blocked"
      },
      "fetch": {
        "version": "^1.0.0",
        "url": "https://raw.githubusercontent.com/cluaiz/cluaiz-tools/main/mcp/fetch/package.json",
        "need": "optional",
        "info": "Reads pages that need no JavaScript"
      }
    },
    "skills": {}
  },
  "permissions": {
    "network": {
      "info": "Searches and downloads the pages the research needs",
      "allow": true,
      "access": "full"
    },
    "files": {
      "info": "Saves the sources and the report in the workspace, and reads them back",
      "allow": false,
      "can": [
        "create",
        "read",
        "update"
      ],
      "where": "workspace"
    },
    "engine": {
      "info": "Searches through the web-search plugin when it is installed",
      "allow": true,
      "methods": [
        "tools.call"
      ]
    }
  },
  "execution": {
    "type": "script_runner",
    "scratch_dir": "scratch",
    "default_language": "python",
    "command": "{python} \"{tool_dir}/scripts/research.py\" \"{file}\""
  },
  "functions": [
    {
      "name": "research_start",
      "description": "Start a research project in the workspace folder. Creates research/<slug>/ (or uses a custom output .md path) and returns the project id and context budget.",
      "parameters": {
        "type": "object",
        "properties": {
          "question": {
            "type": "string",
            "description": "The main question or topic to research"
          },
          "mode": {
            "type": "string",
            "enum": [
              "quick",
              "standard",
              "deep"
            ],
            "description": "quick (fast overview), standard (default balanced research), or deep (shows plan first and checks contradictions)"
          },
          "output": {
            "type": "string",
            "description": "Optional .md file path in the workspace where the final report should be saved"
          },
          "context_tokens": {
            "type": "integer",
            "description": "Optional context size in tokens (such as 4096, 8192, or 16384)"
          }
        },
        "required": [
          "question"
        ]
      }
    },
    {
      "name": "research_plan",
      "description": "Save 2 to 7 sub-questions for the research project. In deep mode, show the returned plan to the user before searching.",
      "parameters": {
        "type": "object",
        "properties": {
          "id": {
            "type": "string",
            "description": "The research project id from research_start"
          },
          "sub_questions": {
            "type": "array",
            "items": {
              "type": "string"
            },
            "description": "List of 2 to 7 specific sub-questions to answer"
          },
          "confirmed": {
            "type": "boolean",
            "description": "In deep mode, set true after the user approves the plan"
          }
        },
        "required": [
          "id",
          "sub_questions"
        ]
      }
    },
    {
      "name": "research_search",
      "description": "Search the web for one sub-question, deduplicate URLs, and register sources as S1, S2, and so on.",
      "parameters": {
        "type": "object",
        "properties": {
          "id": {
            "type": "string",
            "description": "The research project id"
          },
          "sub_question": {
            "type": "integer",
            "description": "Sub-question number (1-based)"
          },
          "query": {
            "type": "string",
            "description": "Focused web search query"
          }
        },
        "required": [
          "id",
          "sub_question",
          "query"
        ]
      }
    },
    {
      "name": "research_read",
      "description": "Fetch a source (such as S1 or a URL), save the full page to disk under sources/Sn.md, and return the top relevant facts with exact quotes.",
      "parameters": {
        "type": "object",
        "properties": {
          "id": {
            "type": "string",
            "description": "The research project id"
          },
          "source": {
            "type": "string",
            "description": "Source id from research_search (such as S1) or a direct URL"
          },
          "sub_question": {
            "type": "integer",
            "description": "Sub-question number (1-based) to rank passages against"
          }
        },
        "required": [
          "id",
          "source",
          "sub_question"
        ]
      }
    },
    {
      "name": "research_status",
      "description": "Read disk state to recover progress after context truncation: shows facts collected per sub-question, gaps left, and the next recommended step.",
      "parameters": {
        "type": "object",
        "properties": {
          "id": {
            "type": "string",
            "description": "The research project id"
          }
        },
        "required": [
          "id"
        ]
      }
    },
    {
      "name": "research_write",
      "description": "Save a section or assemble the final cited Markdown report (.md) on disk, verify every [Sn] citation against saved pages, and return the report file path.",
      "parameters": {
        "type": "object",
        "properties": {
          "id": {
            "type": "string",
            "description": "The research project id"
          },
          "section": {
            "type": "integer",
            "description": "Optional 1-based section number to save before finalizing"
          },
          "text": {
            "type": "string",
            "description": "Optional Markdown prose for that section with [Sn] citations"
          },
          "sections": {
            "type": "object",
            "description": "Optional map of section numbers ('1', '2', ...) to Markdown prose with [Sn] citations to save multiple sections in one call"
          },
          "output": {
            "type": "string",
            "description": "Optional .md path in the workspace for the final report"
          },
          "finalize": {
            "type": "boolean",
            "description": "Set true to build the complete report.md, verify all [Sn] quotes against saved sources, and return the saved .md file path"
          }
        },
        "required": [
          "id",
          "finalize"
        ]
      }
    }
  ],
  "latest_version": "1.0.0",
  "versions": {
    "1.0.0": {
      "updated_at": "2026-11-11T17:15:00Z",
      "files": {
        "readme": "README.md",
        "skill": "SKILL.md",
        "icon": "assets/icon.svg",
        "references": [
          "references/research-method.md"
        ],
        "scripts": [
          "scripts/research.py"
        ]
      }
    }
  }
}
```

## The fields

| Field | Required | Meaning |
|---|---|---|
| `id` | Yes | Lowercase kebab-case. Equal to the folder name, the `family.json` key and `name` in `SKILL.md` |
| `name` | Yes | Display name |
| `title` | No | A short line for cards. The Engine also matches on it |
| `category` | Yes | A grouping for the App and the website. The Engine ignores it |
| `hub_type` | Yes | `skill` |
| `description` | Yes | What it does and when to use it ("Use when ..."). Keep it the same as in `SKILL.md` |
| `author` | No | `name` and `url` |
| `license` | Yes | A license identifier |
| `icon` | Yes | Always `assets/icon.svg` |
| `tags` | No | For a skill, these are matched too |
| `triggers.semantic` | Yes | Words and phrases that offer this skill. For a skill they sit under an object |
| `dependencies` | Yes | Other tools, with `need` and `info`. See [Dependencies](#dependencies) |
| `permissions` | Yes | What its script may reach. `{}` for a skill with instructions only. See [Permissions](#permissions) |
| `env` | No | Settings and keys its script reads. See [Env](#env) |
| `execution` | With functions | How the script runs. See below |
| `functions` | With a script | What the model can call: `name`, `description` and `parameters` as JSON schema |
| `latest_version` | Yes | The version installed by default |
| `versions.<v>.updated_at` | Yes | Build time |
| `versions.<v>.files` | Yes | **Every file**, by group: `readme`, `skill`, `icon`, `references`, `scripts`. A skill without a release is installed from this list, so a file not listed is not installed |

## Execution

```json
{
  "type": "script_runner",
  "scratch_dir": "scratch",
  "default_language": "python",
  "command": "{python} \"{tool_dir}/scripts/research.py\" \"{file}\""
}
```

| Field | Meaning |
|---|---|
| `type` | `script_runner`, the only type |
| `command` | The command line. `{python}` is the Python found on this computer (`python` or `python3`), `{tool_dir}` the installed folder, `{file}` the file with the call's input |
| `default_language` | The language used when the input names none |
| `scratch_dir` | The workspace folder for temporary files |

The input file holds the call's arguments as JSON, with the function's name in `_fn`. The script prints its answer.

## Permissions

`permissions` lists what the tool may reach. The user reads each `info` at install and can allow or refuse each permission later with `cluaiz skill permission deep-research [<permission> on|off]`, the App's tool page, or `tools.set_settings`.

| Field | Meaning |
|---|---|
| The key | A type from the fixed list: `network`, `files`, `programs`, `system`, `location`, `media`, `clipboard`, `notifications`, `native`, `engine` |
| `info` | One sentence the user reads: what the tool does with it |
| `allow` | Allowed at install or not. The Engine decides what is sensitive, and a sensitive permission is never allowed until the user allows it |
| The type's options | Fixed per type, such as `access` and `hosts` for `network`, or `can` and `where` for `files` |

Every type, its options and their values are in [Permissions](../reference/permissions.md). The date, time, time zone and the operating system's name are always available and need no permission.

`deep-research` uses `full` access, because it reads any page the research needs. A script that talks to one service uses `strict`, like this one for the GitHub API:

```json
{
  "network": {
    "info": "Reads public repositories from the GitHub API when gh is not installed",
    "allow": true,
    "access": "strict",
    "hosts": [
      "api.github.com"
    ]
  }
}
```

A script is a normal program, so the Engine cannot block what it reaches. While a permission is refused, the Engine asks the user before each call instead. Declare every type the script really uses, as `deep-research` declares `files` (it saves the report and reads it back).

## Env

`env` names every setting the tool reads, keys among them. It is not a permission: a key says who the user is to a service, a permission says what the tool may reach.

| Field | Meaning |
|---|---|
| The name | Upper case letters, digits and `_`, as for an environment variable |
| `info` | One sentence the user reads when asked for the value |
| `need` | `required`: the tool cannot work without it, and the user is asked at install. `optional` (the default): the tool works without it |

The Engine takes each value from the system environment variable of that name first. Otherwise the user sets it with `cluaiz skill config deep-research <NAME>` (asked for hidden when the value is left out), the App, or `tools.set_settings` with `user_settings`. Values are never written into the tool's folder.

The script gets each value as an environment variable.

## Dependencies

`dependencies` lists other tools by kind: `plugins`, `mcp` and `skills`. Keep all three keys, empty when unused.

| Field | Meaning |
|---|---|
| `version` | A version range, such as `^1.0.0` |
| `url` | The tool's raw `package.json`: `https://raw.githubusercontent.com/cluaiz/cluaiz-tools/main/<kind>/<id>/package.json` |
| `need` | `required`: cannot work without it. `recommended`: works without it, better with it. `optional`: useful sometimes, only suggested |
| `info` | One sentence the user reads at install. Required for `required` and `recommended` |

What is installed without asking depends on the user's mode (`full_access`, `sandboxed`, `strict`): see [Installing by the user's mode](../package-spec.md#installing-by-the-users-mode). The tool must still work, with a clear message, when a `recommended` or `optional` one is missing.
