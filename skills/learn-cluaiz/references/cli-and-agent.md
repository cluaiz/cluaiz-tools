# The coding agent

`cluaiz agent` works in a folder: it reads, searches and edits files and runs commands, asking before each change.

```bash
cd path/to/project
cluaiz agent                         # chat
cluaiz agent "list the TODO comments in src"     # one task, then exit
cluaiz agent -m qwen3-4b-gguf-Q4_K_M  # choose the model (an installed id, a .gguf path, or <provider>/<model>)
cluaiz agent -C path/to/project      # work in another folder
cluaiz agent --continue              # go on with the last conversation here
```

## Modes

| Mode | Runs without asking |
|---|---|
| `ask` (default) | Nothing: every change and command asks |
| `accept-edits` | File changes; commands ask |
| `plan` | Nothing runs; the model writes a plan |
| `full` (`--yes`) | Everything, except destructive commands, which stay refused |

`--mode <mode>` sets it for one run. `/mode` changes it inside the agent.

## Commands inside the agent

| Command | Does |
|---|---|
| `/init` | The model reads the project and writes `.agents/AGENTS.md` |
| `/<workflow>` | Runs `.agents/workflows/<workflow>.md` |
| `/skills` | Lists the skills the model can use |
| `/memory` | Lists instructions, rules, workflows and where each kind goes |
| `/undo` | Puts back the files the last request changed |
| `/rewind`, `/rewind <n>` | Lists checkpoints, or goes back to before request n |
| `/model`, `/think`, `/effort` | Change the model, thinking, effort |
| `/help` | All commands |

## The `.agents/` folder

```text
.agents/
├── AGENTS.md              instructions, always read
├── rules/*.md             rules (front matter `trigger`: always_on, model_decision, glob, manual)
├── workflows/*.md         step-by-step procedures, run as /<name>
├── skills/<name>/SKILL.md skills
├── memory/*.md            facts the model proposed and the user confirmed
├── flows/                 automation flows of the project
└── settings.json          the project's agent settings (settings.local.json: the user's own)
```

The same folders in `~/.agents/` apply to every project. A nearer item hides a farther one of the same name.

- **Memory:** the model proposes a fact with the `remember` tool; nothing is saved without the user's yes.
- **Skills:** only each skill's name and description go into the prompt; the model reads the one it needs with `read_item`. Naming the skill in the request helps small models use it.
- **Installed tools** (`cluaiz skill/plugin/mcp install`) are offered next to the built-in tools, and each call asks first.

## Scripts and CI

```bash
cluaiz agent -p "run the tests and summarise the failures" --output-format json \
  --allowed-tools "read_file,search,run_command(pytest*)"
```

`--output-format json` prints one object at the end (`result`, `files_changed`, `refused_calls`, `events`, `exit_code`). In a run nobody can answer, a call that would ask is refused unless `--allowed-tools` or `--yes` allows it. `--max-steps` limits tool calls (default 40).

## Automation flows

Flows are YAML or JSON files (`flow: cluaiz.flow/1`) with triggers (`on:`) and steps, run by the Engine. `cluaiz flow validate <file>` checks one, `cluaiz flow save <file>` saves it, `cluaiz flow run <name>` runs it, `cluaiz flow draft "<what it should do>"` writes one with a model. Automation is a draft for review and may change before a release.
