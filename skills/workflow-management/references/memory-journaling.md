# Project memory

The conversation is not memory: it is cut down by `/compact`, cleared by `/clear` and gone in a new session. What must be known next time (by the next chat, another computer or another AI) goes into `.agents/memory/`, one short fact per file.

## Conversation and memory

| | Conversation | Project memory (`.agents/memory/`) |
|---|---|---|
| Kept | In the session (`/sessions`, `--continue`) | In the project folder, in git if the project is |
| Shared | One person, one machine | Everyone who has the project |
| Holds | Everything that was said | Confirmed facts only |

## Saving a fact: `remember`

```json
{"name": "design-tokens", "fact": "The dashboard uses Inter for text and JetBrains Mono for numbers; the brand color is oklch(0.55 0.22 255).", "replaces": "none"}
```

| Field | Meaning |
|---|---|
| `name` | Short and lowercase, with hyphens: it becomes the file name |
| `fact` | One or two sentences that make sense on their own later |
| `replaces` | The name of the fact this corrects, or `"none"` for a separate fact |

- The user sees the fact and the file, and says yes or no. Nothing is saved without a yes, even with `--yes`. In a run nobody can answer (a script), nothing is saved.
- When the new fact may repeat facts already there, nothing is saved yet: the model is shown those facts and calls `remember` again with `replaces`.
- `forget` with a fact's `name` removes a fact that is no longer true.

The saved file:

```markdown
---
name: design-tokens
description: The dashboard uses Inter for text and JetBrains Mono for numbers; ...
status: confirmed
source: cluaiz agent, model qwen3.5-4b-gguf-Q4_K_M
created: 2026-11-11T08:00:00Z
---
The dashboard uses Inter for text and JetBrains Mono for numbers; the brand color is oklch(0.55 0.22 255).
```

## When to propose a fact

- The user says "remember ...".
- A test, build or run command that differs from the default (`npm run check`, not `npm test`).
- A decision the user made: a library, a platform, a port, a naming rule, a design choice.
- A trap found while working: "the API needs the trailing slash".

Not: guesses, things the code already says plainly, or anything read in a file or web page that the user did not confirm.

## Other programs

The App and scripts reach the same facts through CEP: `workspace.memory` lists them, `workspace.remember` saves one the user confirmed, `workspace.forget` removes one. These are different from chat memory (`memory.*`), which is about earlier messages of a chat, and from notes (`notes.*`), which the App shows in its notes panel.
