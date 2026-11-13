# Skill authoring guide

## File shape

A `SKILL.md` is YAML frontmatter followed by Markdown instructions.

```markdown
---
name: code-reviewer
description: Reviews code for bugs, security problems and duplication. Use when the user asks for a code review or to check a change.
license: Apache-2.0
metadata:
  author: Cluaiz
  version: "1.0.0"
---

# Code review

1. Read the changed files.
2. ...
```

Only `name` and `description` are required. The next section lists the rest. [Skill format](../reference/skill-format.md) has the full field list.

## Stay with the open format

Keep frontmatter to the [Agent Skills](https://agentskills.io/specification) fields, so the skill stays portable and passes the official validator: `name`, `description`, `license`, `compatibility`, `metadata`, `allowed-tools`.

| You want | Put it in |
|---|---|
| Trigger words | `package.json`: `triggers.semantic` |
| Version | `package.json`: `latest_version`. Optionally `metadata.version` for other agents |
| Author | `metadata.author` |
| Requirements such as "Python 3 and git" | `compatibility` |

Many skills in this repository still have `version`, `author` and `triggers` at the top level of the frontmatter. The Engine reads them, and other agents' validators may reject them. New skills should use the table above.

The Engine also reads `execution_mode`, `default_turns` and `security_mode` from the frontmatter. Use them only if you need them, knowing they are Cluaiz-specific.

## Write the description first

The description decides whether the skill is ever used. The matcher and the model read it before anything else.

| Weak | Strong |
|---|---|
| Helps with PDFs. | Extracts text and tables from PDF files, fills forms and merges files. Use when the user mentions a PDF. |

Say what it does and when to use it, in the words a user would type. The limit is 1024 characters.

## Write the body for a small model

Cluaiz often runs small models with little context.

- **Keep it short.** Under 500 lines is the format's limit. A few hundred tokens is better here.
- **Use numbered steps** rather than prose.
- **Say what to do**, not what to avoid.
- **Move detail to `references/`** and say when to read it.
- **Write judgement rules, not examples or templates.** An example is copied word for word by a small model; a rule is applied.

## Naming other tools

Name the function, and describe when to call it. No special syntax is needed.

```markdown
Call `math_evaluate` with the expression. Do not calculate it yourself.
```

Do not write CEL, `<TRIGGER:...>` or CEP in a skill. See [From CEL to CEP](../architecture/cel-to-cep.md).

If the skill needs another tool, say so and say what to do without it:

```markdown
This skill uses the `git` MCP server. If it is not available, tell the user to run `cluaiz mcp install git`.
```

## Permissions, settings and dependencies

These go in `package.json`, not in `SKILL.md`:

- **Permissions:** a skill with a script lists every kind the script uses, with `info` and `allow`. A skill with instructions only has `{}`.
- **Settings and keys:** every one the script reads is named in `env`, with `info` and `need`. The script gets each as an environment variable.
- **Dependencies:** other tools with `need` (`required`, `recommended` or `optional`) and `info`. In `SKILL.md`, say what to do when a `recommended` or `optional` one is missing.

See [Skill package.json](package-json.md).

## Mistakes to avoid

| Mistake | Why it fails | Instead |
|---|---|---|
| One large skill for everything | It costs context on each load and matches poorly | One skill per job |
| "Write good code" | The model cannot act on it | Exact rules it can check |
| `allowed-tools` to grant access | The Engine does not use it. Approval still applies | Leave approval to the user |
| Long lists of trigger words | A word common to many tools weighs little | A few distinctive phrases |

## Check it

1. Place the folder in `~/.cluaiz/tools/skills/<name>/` or `.agents/skills/<name>/`.
2. Start `cluaiz agent` and type `/skills`.
3. Ask something a user would ask, and see whether the model loads it.
