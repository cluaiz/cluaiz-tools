---
name: learn-cluaiz
description: "Explains what Cluaiz is and how to use it: installing, running models, the coding agent, tools (skills, plugins, MCP servers), the APIs and CEP, and how to contribute. Use when the user asks about Cluaiz itself, its commands, its API, or how to do something with it."
---

# Learn Cluaiz

Answer plainly and correctly from this skill and its files. If something is not covered, say so instead of guessing, and point to `cluaiz --help`, `cluaiz <command> --help` or https://cluaiz.com/docs. Never invent a command, flag, API route or CEP method.

Links such as `references/...` are files of this skill: read them with `read_tool_file` (terminal agent: `read_item` with `learn-cluaiz` gives the folder, then `read_file` on folder plus link).

## What it is

Cluaiz is an AI engine written in Rust, in pre-release (v0.1.x). One process runs models on the user's own GPU, RAM and CPU and serves them to apps, scripts and agents:

- GGUF models through a fork of llama.cpp: chat, vision, audio input, embeddings.
- ONNX models through ONNX Runtime: text-to-speech (Kokoro and others), speech-to-text (Whisper), embeddings.
- A placement planner that measures free GPU memory and RAM on every load and chooses where the model runs, the context size and the KV cache type. `cluaiz show <model>` prints the plan.
- Models of OpenAI-compatible APIs can be added as providers and used as `<provider>/<model>`.
- APIs: OpenAI-compatible and Ollama-style routes, plus CEP (JSON-RPC 2.0 at `POST /v1/cep`) for everything Engine-specific.
- Tools for models: skills, plugins and MCP servers, with approvals and an audit log.

## Quick answers

Answer these directly; read a reference only for more.

| Question | Answer |
|---|---|
| Start the API server | `cluaiz serve`: OpenAI-compatible routes at `http://127.0.0.1:8000/v1`, CEP at `POST /v1/cep` |
| The token | `Authorization: Bearer <token>`, the token in `~/.cluaiz/session.token` (when the owner turned authentication on), or an API key from `tokens.create` |
| List installed tools (CEP) | `tools.list`; install with `tools.install`, remove with `tools.remove`, call a function with `tools.call` |
| All CEP methods | `rpc.discover` |
| Chat with a model | `cluaiz run <model>`; the coding agent: `cluaiz agent` |
| Where a model would run | `cluaiz show <model>` |

## Which file to read

| The user asks about | Read |
|---|---|
| Installing, first model, chat, serving | [references/getting-started.md](references/getting-started.md) |
| `cluaiz agent`, `.agents/`, rules, workflows, memory | [references/cli-and-agent.md](references/cli-and-agent.md) |
| How the Engine works inside, the repository folders | [references/engine-architecture.md](references/engine-architecture.md) |
| The HTTP API, CEP methods, tokens | [references/cep-api-reference.md](references/cep-api-reference.md) |
| Reporting bugs, contributing, building from source | [references/contributing-guide.md](references/contributing-guide.md) |
| Something none of these cover | [references/dynamic-retrieval.md](references/dynamic-retrieval.md) |

## Tools

| Type | What it is | Needs on the machine |
|---|---|---|
| Skill | Instructions the model reads when a task fits (`SKILL.md`); some also offer functions run by a script | Nothing, or Python for skills with functions |
| Plugin | A program the Engine runs, usually one WebAssembly file in a sandbox | Nothing, for WebAssembly |
| MCP server | An outside program using the Model Context Protocol | Usually Node or `uv` |
| Extension | A mini-application in the App | In development |

The model first sees only short entries for the installed tools. When a tool fits, it loads that tool's instructions and functions, then makes ordinary tool calls. The Engine runs each call, asks for approval when needed, and returns the result.

| Kind | In the catalog |
|---|---|
| Plugins | `math`, `text`, `time`, `rank`, `web-search` |
| Skills | `code-interpreter`, `code-reviewer`, `git-assistant`, `github`, `build-feature`, `frontend-design`, `deep-research`, `speech`, `tool-creator`, `workflow-management`, `learn-cluaiz` |
| MCP servers | `browser`, `documents`, `filesystem`, `git`, `fetch`, `context7`, `everything` |

```bash
cluaiz skill install code-reviewer
cluaiz plugin install web-search
cluaiz mcp install filesystem
cluaiz plugin config web-search TAVILY_API_KEY
cluaiz skill list
```

## How to answer

- Give the exact command or request, then one sentence on what it does.
- Say "pre-release" when the user may rely on something staying the same: settings, APIs and file layout can still change.
- Do not describe Cluaiz as only for local models: it also serves provider models through the same API.
