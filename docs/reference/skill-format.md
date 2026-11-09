# Skill format

The open [Agent Skills](https://agentskills.io/specification) format, with some optional Cluaiz fields. For how to write one, see the [authoring guide](../skills/authoring-guide.md).

## Folder

```
<name>/
├── SKILL.md            Required (loaded by the Engine for the AI model)
├── README.md           Required to publish here (human & website documentation with Mermaid diagram)
├── package.json        Required to publish here
├── assets/icon.svg     Required to publish here
├── references/         Optional, read on demand
└── scripts/            Optional, run by the model
```

## `SKILL.md`

YAML frontmatter between `---` lines, then Markdown.

### Standard fields

| Field | Required | Rules | Used by the Engine |
|---|---|---|---|
| `name` | Yes | 1 to 64 characters: lowercase letters, digits and single hyphens. Must equal the folder name | Yes |
| `description` | Yes | 1 to 1024 characters. What it does and when to use it | Yes |
| `license` | No | A license name | No |
| `compatibility` | No | Up to 500 characters. What the skill needs | No |
| `metadata` | No | A map of text keys and values | No |
| `allowed-tools` | No | Experimental in the standard | Read as a list only, and not acted on |

### Cluaiz fields

The Engine reads these, but they are outside the standard, and strict validators in other agents may reject them. For new skills, prefer `package.json` where the table says so.

| Field | Meaning | Prefer |
|---|---|---|
| `version` | Version | `latest_version` in `package.json` |
| `triggers` | A list, or `semantic` under an object | `triggers.semantic` in `package.json` |
| `execution_mode` | `auto`, `manual`, `confirm_destructive` | Keep here if needed |
| `default_turns` | Answers it stays loaded when unused | Keep here if needed |
| `security_mode` | `inherit` or `require_approval` | Keep here if needed |

### Fields that do nothing

`id`, `title`, `author`, `keywords`, `permissions`, `links` and `hard_trigger_tokens` appear in older skills and pages. The Engine ignores them for skills. `metadata.author` is the standard place for an author.

## Body

The instructions the model gets from `load_tool`. Under 500 lines is the standard's limit, and shorter is better for small models. Use relative paths for other files:

```markdown
See [the guide](references/guide.md).
```

The model reads those with `read_tool_file`. Text files only, and the first 40 are listed.

## Where the Engine and agent look

See [Skills at run time](../architecture/skill_architecture.md#where-skills-come-from).
