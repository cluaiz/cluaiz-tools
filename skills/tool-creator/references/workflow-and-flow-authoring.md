# Workflows and automation flows

Two different things:

| | Workflow | Flow |
|---|---|---|
| What | Steps in Markdown that the model follows | A YAML or JSON program the Engine runs |
| Where | `.agents/workflows/<name>.md` (or `~/.agents/workflows/`) | `.agents/flows/<name>.flow.yaml` (project) or `~/.cluaiz/flows/` (user) |
| Started by | `/<name>` in `cluaiz agent` | Triggers (schedule, file, webhook, ...) or `cluaiz flow run <name>` |
| Runs | The model, with its tools, asking before changes | The Engine, step by step, within the flow's `permissions` |

## Workflow

```markdown
---
description: Check the changes before a commit and report what passed
---
1. Run `git status --short` and list the changed files.
2. Run the project's tests (find the command in AGENTS.md or package.json).
3. Report each check, its command and its result.
```

- The file name is the command: `check-before-commit.md` gives `/check-before-commit`.
- Text after the command is passed to the model: `/check-before-commit src/parser`.
- Only `description` (and `name`) are read from the front matter. There are no typed parameters.

## Flow

```yaml
flow: cluaiz.flow/1
name: new-csv-summary            # must equal the file name: new-csv-summary.flow.yaml
description: Summarise each new CSV file in the inbox folder into a note.
on:
  - file: { path: "~/Inbox", glob: "*.csv" }
permissions:
  folders: ["~/Inbox", "~/Notes"]
  methods: ["ingest", "fs.write"]
  models: ["*"]
  scopes: ["write"]
steps:
  - id: text
    call: ingest
    with: { source: "${{ trigger.path }}" }
  - id: summary
    ask: "Summarise this table in five lines."
    with: { table: "${{ steps.text.output.processed_sources[0].chunks }}" }
  - call: fs.write
    with: { path: "~/Notes/${{ trigger.name }}-summary.md", text: "${{ steps.summary.output }}" }
```

This flow passes `cluaiz flow validate`; it shows the format, and each part is explained below.

### Triggers (`on`)

`manual`, `schedule` (cron, `"0 9 * * 1-5"`), `every` (`15m`), `webhook`, `poll` (a tool's answer changes), `file` (`path`, `glob`, `events`, `recursive`), `engine` (Engine events), `event` (from another flow), `tool` (the flow offered as a tool), `startup`.

### Step kinds

One kind key per step: `tool`, `call`, `ask`, `agent`, `code`, `if`, `switch`, `each`, `parallel`, `repeat`, `wait`, `approve`, `set`, `run`, `emit`, `return`, `fail`.

| Kind | Does |
|---|---|
| `tool` | One function of an installed tool: `mcp:<server>/<tool>`, `plugin:<name>/<function>` or `skill:<name>` |
| `call` | A CEP method (`fs.write`, `memory.search`) or a route (`ingest`, `embeddings`, `speech`, `transcription`) |
| `ask` | One answer from a model, optionally with `output_schema` |
| `agent` | A model that uses tools for several steps |
| `code` | Your own WebAssembly |
| `run` | Another flow |
| `approve` | Wait for a person |

A flow cannot run programs or shell commands: `system.exec` is not callable from a flow. Use a plugin or an MCP tool through `tool`, or a workflow when the model should run commands with the user watching.

### Values

`${{ ... }}` reads `inputs.*`, `trigger.*`, `steps.<id>.output`. Conditions use `when:` with expressions such as `{ eq: [{ var: steps.judge.output.urgent }, true] }`.

### Check, save, run

```bash
cluaiz flow validate new-csv-summary.flow.yaml       # problems and the permissions it needs
cluaiz flow save new-csv-summary.flow.yaml --scope project
cluaiz flow run new-csv-summary
cluaiz flow draft "when a PDF lands in Downloads, summarise it into Notes"   # a model writes one
```

Always run `cluaiz flow validate` before giving a flow to the user. Automation is a draft for review in the Engine; field names may change before a release.
