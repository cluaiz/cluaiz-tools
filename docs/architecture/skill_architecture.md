# Skills at run time

This page explains what the Engine does with a skill. To write one, see the [authoring guide](../skills/authoring-guide.md). For the file format, see [Skill format](../reference/skill-format.md).

## Stages

| Stage | What the model has | Size |
|---|---|---|
| Installed, not matched | Nothing, or only a title | 0 to about 15 tokens |
| Matched | A short card: kind, purpose, load cost | About 40 to 60 tokens |
| Loaded with `load_tool` | The whole `SKILL.md` body and the names of its other files | The size of the body |
| More needed | One file, through `read_tool_file` | The size of the file |

This is the progressive loading the Agent Skills format expects, and it matters most for small models with little context.

## Where skills come from

| Place | Used by | Added by |
|---|---|---|
| `~/.cluaiz/tools/skills/<id>/` | Engine and agent | `cluaiz skill install`, or by hand |
| `.agents/skills/<id>/` from the working folder up to the repository root | Terminal agent | The project |
| `~/.agents/skills/<id>/` | Terminal agent | The user |

`/skills` in the terminal agent lists what it found.

## What a skill may contain

```
my-skill/
├── SKILL.md          Required
├── package.json      Required to publish
├── assets/icon.svg   Required to publish
├── references/       Read on demand
└── scripts/          Run by the model or through execution
```

Text files in the folder, up to 40, can be read with `read_tool_file`.

## Skills that run something

A skill is normally text. Two things let it run code:

- **Telling the model to run a script**, such as `python scripts/check.py`. The model uses the agent's command tool, which asks for approval.
- **`execution` and `functions` in `package.json`.** The skill then offers functions as a script plugin does. `code-interpreter` and `tool-creator` work this way.

## Parts of older documents that are not real

These were design ideas, and the Engine does not do them: pre-computed `state.kvcache.bin` files, `.cell` and `.rhai` scripts, automatic `.cache/` folders in a skill, and `links` in frontmatter.

The Engine does reuse its KV cache across rounds, for all prompts. A skill does nothing to enable that.
