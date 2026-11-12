# Inside the Engine

## Runtimes

| Format | Runtime | Used for |
|---|---|---|
| GGUF | A fork of llama.cpp linked into the Engine | Chat, tool calls, thinking models, vision and audio input (with a projector), embeddings |
| ONNX | ONNX Runtime | Text-to-speech (Kokoro, Piper and other families), speech-to-text (Whisper, Moonshine), embeddings, CLIP-style image embeddings |
| Provider | An OpenAI-compatible API | Chat, embeddings, speech; runs through the Engine's own chat, so memory and tools work with it |

## Placement

On every load the planner measures free GPU memory and RAM and compares candidates: the whole model on the GPU, split across GPU and RAM, MoE experts in RAM (or read from the SSD when they do not fit RAM), or CPU only. It picks the context size and the KV cache type (F16, then Q8_0) and keeps a VRAM buffer free. A dense model that fits nowhere is refused with the amount missing.

- `cluaiz show <model>` prints every candidate with its memory numbers.
- Every value can be set by hand with llama.cpp's flag names (`-ngl`, `-c`, `-ctk`, `-fa`, `-ncmoe`) or in a request's `llama` object.
- The VRAM a load really takes is remembered per model, so the next plan is closer.

## Speed features

- The shared beginning of a chat stays in the KV cache between turns; shared chunks after it are moved, not recomputed.
- Prompt chunks are shaped for models split across GPU and RAM.
- The CPU backend picks the best build for the processor at start (SSE4.2 to AVX-512, NEON on ARM).

Measured against llama.cpp's own `llama-bench` on the same file and settings: generation speed is equal; prompt reading is equal on GPU-only and CPU-only, and faster when a model is split across GPU and RAM. The numbers are in the Engine repository's README and `test/engine_regression/RESULTS.md`.

## Chat memory and tools

- Earlier messages that no longer fit the prompt are recalled when a new message needs them.
- Installed tools are offered to the model as short entries; the model loads a tool's instructions and functions when it needs them (`load_tool`, `read_tool_file` in the API; `read_item` in the terminal agent).
- Execution policy `sandboxed` (default) or `full_access`, per Engine and per tool. Actions outside the workspace ask for approval; destructive commands and credential paths are blocked. Every write, exec and admin call goes to an audit log.

## Repository layout

The Engine repository (https://github.com/cluaiz/cluaiz) is a Rust workspace:

| Folder | Holds |
|---|---|
| `engine/` | `core`, `media`, `hardware`, `models`, `plan`, `llama`, `onnx`, `runtime`: model loading, the planner, the runtimes |
| `api/` | The HTTP server, the OpenAI-style routes and the CEP method handlers |
| `cep/` | The CEP protocol: the JSON-RPC envelope, method registry, approvals and tasks |
| `agents/` | Tools: registry, installer, skills, plugins, MCP client, execution |
| `context/` | Context planning: what goes into each prompt |
| `security/` | Execution policy and permissions |
| `cmd/` | The `cluaiz` command line and the coding agent |
| `terminal/` | Terminal display parts used by the command line |
| `automation/` | Automation flows |
| `ffi/` | The C library (`cep_call`, `cep_free`) |
| `docs/` | The documentation: `docs/api/` for routes and CEP methods, `docs/cmd/` for commands |

The tools live in a separate repository, https://github.com/cluaiz/cluaiz-tools.
