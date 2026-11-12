# The `.agents/` folder

A project keeps its instructions, rules, workflows, flows, skills and memory for agents in `.agents/` at the repository root (`.agent/` is read too).

## 1. Directory Layout

```text
<project-root>/
├── .agents/
│   ├── AGENTS.md                  # Master instructions (project description, build/test, code style)
│   ├── rules/                     # Modular rules loaded on specific conditions
│   │   ├── code-style.md          # Always-on or glob-matched conventions
│   │   └── security.md            # Critical security and boundary checks
│   ├── workflows/                 # Step-by-step procedures accessible as /slash commands
│   │   ├── release.md             # Gives /release in interactive sessions
│   │   └── audit.md               # Gives /audit in interactive sessions
│   ├── flows/                     # Cluaiz automation flows (cluaiz.flow/1)
│   │   └── weekly-review.flow.yaml # Scheduled and event-driven automation
│   ├── memory/                    # Persistent confirmed facts saved via the remember tool
│   │   ├── architecture-choice.md # Domain decisions outliving conversation context
│   │   └── test-commands.md       # Project-specific verified commands
│   ├── skills/                    # Project-local specialized skills
│   │   └── <skill-name>/SKILL.md  # Standalone skill definitions
│   ├── settings.json              # Project-wide agent permissions (committed to git)
│   └── settings.local.json        # Developer-specific overrides (in .git/info/exclude)
```

## 2. What each part does

### `AGENTS.md`
- **Role:** Read when the agent starts. Contains project scope, build/test commands, code conventions, and file boundaries.
- **Order read:**
  1. `~/.cluaiz/AGENTS.md` (or `$CLUAIZ_HOME/AGENTS.md`): the user's rules for every project.
  2. For each folder from the repository root down to the working folder: `.agents/AGENTS.md`, then the folder's own `AGENTS.md` (or `CLAUDE.md` / `GEMINI.md` when it has no `AGENTS.md`).
  3. A nearer file is more specific and wins where two disagree.

### Rules (`.agents/rules/*.md`)

A rule's front matter says when it applies:

| Trigger | Front matter | When it is used |
|---|---|---|
| `always_on` | `trigger: always_on` | Injected into the system prompt for every turn. |
| `model_decision` | `trigger: model_decision` | Listed in the index; read via `read_item` when the prompt matches `description`. |
| `glob` | `globs: "*.rs, *.toml"` | Injected on the first read/write/edit of a matching file. |
| `manual` | `trigger: manual` | Loaded only when explicitly invoked by user prompt. |

### Workflows (`.agents/workflows/*.md`)
- Workflows map directly to terminal slash commands. For example, `.agents/workflows/deploy.md` exposes `/deploy`.
- When invoked, the model follows the steps in order with its tools; each change or command asks first as usual. Text after the command is passed on (`/release 0.1.2`).

### Flows (`.agents/flows/*.flow.yaml` or `.json`)
- Defined using `flow: cluaiz.flow/1`. The file name is `<name>.flow.yaml` (or `.flow.json`) and must match `name`.
- Supported triggers: `schedule` (cron), `every` (intervals), `webhook`, `poll`, `file` (filesystem watch), `engine` events, `tool` calls, or `startup`.
- Steps: `tool`, `call` (CEP), `ask` (model), `agent`, `code` (WASM), `if`, `switch`, `each`, `parallel`, `repeat`, `wait`, `approve`, `set`, `run`, `emit`, `return`, `fail`. A flow cannot run programs; check a flow with `cluaiz flow validate <file>`.

### Memory (`.agents/memory/*.md`)
- Holds durable facts across sessions. Conversation transcripts are ephemeral; `.agents/memory/` survives context compaction and restarts.
- Every fact requires explicit user confirmation before writing to disk.

## 3. Starting a project's folder

Inside `cluaiz agent`, the user types `/init`. The model reads the project (its manifests, README and layout) and writes `.agents/AGENTS.md`, and the empty folders `rules/`, `workflows/`, `skills/` and `memory/` are created. When the project already has instructions (`.agents/AGENTS.md`, `AGENTS.md`, `CLAUDE.md` or `GEMINI.md`), `/init` names them and changes nothing.

`/init` is a command for the user; the model cannot type it. A model can suggest it, or write `.agents/AGENTS.md` itself when the user asks: what the project is and its scope, the build, test and lint commands, code conventions, and boundaries (what the agent must not touch or do), all taken from the project's real files.
