# Getting started

## Install

```powershell
# Windows
powershell -c "irm https://cluaiz.com/install.ps1 | iex"
```

```bash
# Linux and macOS
curl -fsSL https://cluaiz.com/install.sh | bash
```

The installer downloads the release for the system and the GPU pack for the GPU, and checks every file against the release checksums.

Supported: Windows 10 / 11 x64; Linux x64 or ARM64 with glibc 2.39 or newer (Ubuntu 24.04, Debian 13, Fedora 40 or newer); macOS 14 or newer on Apple Silicon. Not yet: Intel Macs, Windows on ARM, Ubuntu 22.04, Debian 12.

| GPU | Backend |
|---|---|
| NVIDIA | CUDA 12 or 13 pack |
| AMD Radeon, Intel Arc | Vulkan pack |
| Apple Silicon | Metal, in the core |
| None | CPU, the best build for the processor is picked at start |

Update with `cluaiz self update` (models and settings stay). Remove with `cluaiz self uninstall` (`--keep-models` keeps the models).

## Run a model

```bash
cluaiz run unsloth/Qwen3-4B-GGUF                          # chat in the terminal; downloads on first use
cluaiz run unsloth/Qwen3-4B-GGUF "Explain a KV cache."    # one answer
cluaiz pull unsloth/Qwen3-4B-GGUF                         # download only
cluaiz list                                               # installed model ids
cluaiz show unsloth/Qwen3-4B-GGUF                         # where it would run, with memory numbers
cluaiz rm <model-id>                                      # remove
```

A model can be an installed id (from `cluaiz list`), a registry name, a Hugging Face repository, or a path to a `.gguf` file. `cluaiz model set-chat <id>` sets the default chat model.

## Serve it to other programs

```bash
cluaiz serve
```

```bash
curl http://127.0.0.1:8000/v1/chat/completions \
  -H "Authorization: Bearer $(cat ~/.cluaiz/session.token)" \
  -H "Content-Type: application/json" \
  -d '{"model": "qwen3-4b-gguf-Q4_K_M", "messages": [{"role": "user", "content": "Hello"}]}'
```

The port and host come from `permission.json` (`cluaiz permission port <n>`). `cluaiz token show` shows the API token. `cluaiz ps` shows the loaded model.

## Provider models

```bash
cluaiz provider add      # an OpenAI-compatible API: its URL and where its key comes from
cluaiz provider models <provider>
cluaiz run <provider>/<model>
```

Keys are read from the environment or `config/.env` and never shown in full.

## Useful commands

| Command | Does |
|---|---|
| `cluaiz status` | The hardware the planner sees |
| `cluaiz calibrate` | Scan the hardware again after a change |
| `cluaiz bench <model>` | Prompt and generation speed |
| `cluaiz config` | Settings (`settings.json`) |
| `cluaiz ingest <file>` | Turn a document into text, chunks or vectors |
| `cluaiz --help`, `cluaiz <command> --help` | Every command and flag |
