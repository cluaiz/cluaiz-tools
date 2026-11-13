# Skills

A skill is a set of instructions the model reads when it needs them. It teaches a procedure: how to review code, how to write a release note, which tool to use for a job.

Skills use the open [Agent Skills](https://agentskills.io/specification) format, so a skill written for Cluaiz can also work in other agents, and the other way round.

## What a skill contains

```
my-skill/
├── SKILL.md            Required: frontmatter and instructions
├── package.json        Required to publish: triggers, permissions, settings, dependencies
├── README.md           Required to publish: for people, with a Mermaid diagram
├── assets/icon.svg     Required to publish
├── references/         Optional: read on demand
└── scripts/            Optional: run by the model
```

A bare folder with only `SKILL.md` works when placed by hand, as other agents' skills are.

## How the model gets it

1. The Engine matches the user's message against installed skills and shows short cards.
2. The model calls `load_tool` and receives the `SKILL.md` body.
3. If the body points to a reference, the model reads it with `read_tool_file`.

An unmatched skill costs nothing or only a title, however long it is. See [Skills at run time](../architecture/skill_architecture.md).

## What a skill can do

| Can | Cannot |
|---|---|
| Tell the model which tools to use | Call a tool by itself |
| Include reference files | Keep memory by itself |
| Include scripts for the model to run | Grant itself approval or a permission |
| Offer functions through `execution` | Reach what the user refused: while a permission is refused, each call is asked about |
| List other tools in `dependencies`; they are installed or offered with it by the user's mode | Read a key not named in `env` |

## Where to go next

- [Authoring guide](authoring-guide.md)
- [Skill package.json](package-json.md): a complete, real manifest, field by field
- [Package fields](manifest-schema.md)
- [Tutorial: first skill](tutorial-first-skill.md)
- [Skill format](../reference/skill-format.md)
