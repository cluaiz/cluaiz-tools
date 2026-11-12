# The HTTP API and CEP

`cluaiz serve` starts the server, by default on `http://127.0.0.1:8000` (the port is in `permission.json`; `cluaiz permission port <n>` changes it).

## Authentication

Send `Authorization: Bearer <token>`. The local session token is in `~/.cluaiz/session.token`; `cluaiz token show` shows it. API keys for other programs are made with `cluaiz token create` (or CEP `tokens.create`).

## Standard routes

| Route | Purpose |
|---|---|
| `POST /v1/chat/completions`, `POST /api/chat` | Chat, streaming or not, with tools, images, audio and documents |
| `POST /v1/embeddings` | Text and image embeddings |
| `POST /v1/audio/speech` | Text to speech; the answer holds a WAV |
| `POST /v1/audio/transcriptions` | Speech to text |
| `POST /v1/ingest` | A document to text, chunks or vectors |
| `GET /v1/models`, `GET /api/tags`, `POST /api/pull` | Installed models and provider models, downloads |

## CEP

CEP is JSON-RPC 2.0 for everything Engine-specific: `POST /v1/cep`, also `POST /mcp` and `cluaiz mcp-server` for MCP clients (read-only methods), and `cep_call` in the C library.

```bash
curl http://127.0.0.1:8000/v1/cep \
  -H "Authorization: Bearer $(cat ~/.cluaiz/session.token)" \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc": "2.0", "id": 1, "method": "system.info", "params": {}}'
```

`rpc.discover` returns the full, current list of methods the caller may use (OpenRPC). Use it instead of guessing a name.

| Group | Methods |
|---|---|
| `system` | `info`, `live`, `ps`, `plan`, `audit`, `exec` (owner approves), `update_check` |
| `models` | `list`, `download`, `load`, `remove`, `inspect`, `embedding_room` |
| `providers` | `list`, `set`, `remove`, `models`, `test` |
| `tools` | `list`, `discover`, `call`, `install`, `remove`, `files`, `read_file`, `write_file`, `get_settings`, `set_settings`, `cache`, `clear_cache`, `open_file` |
| `approvals` | `list`, `approve`, `reject` |
| `chat` | `cancel`, `set_tools`, `tools`, `telemetry`, `skip_reasoning`, `import_app` |
| `memory` | `search`, `events`, `index`, `item`, `export`, `import`, `delete`, `grants`, `revoke_grants` |
| `notes` | `list`, `create`, `confirm`, `invalidate`, `pin`, `move` |
| `context` | `preview`, `report`, `reports`, `jobs` |
| `prompts` | `list`, `get`, `set`, `reset`, `history` |
| `audio` | `voices`, `languages`, `run` |
| `kv` | `status`, `delete`, `clear` |
| `fs` | `list`, `read`, `write`, `mkdir`, `copy`, `rename`, `delete`, `pick_folder` |
| `workspace` | `get`, `set`, `index`, `item`, `memory`, `remember`, `forget` |
| `config` | `get`, `set` |
| `tokens` | `list`, `create`, `revoke` |
| `storage` | `status`, `clean` |
| `tasks` | `list`, `get`, `result`, `cancel` |
| `rpc` | `discover` |

Automation adds `flows.*` methods (`list`, `get`, `validate`, `save`, `run`, `runs`, `run_get`, `rerun_from`, `enable`, `disable`, `remove`, `draft`, `explain`, `fix`, `schema`, `nodes`); automation is a draft for review.

## Long work: tasks

A method marked `task` (such as `models.download`) answers with a task id at once. Poll `tasks.get` for progress, or `tasks.result` with `wait_ms` for the end.

## Errors

Errors follow JSON-RPC: `{"error": {"code": ..., "message": "..."}}`. A call that needs the owner's approval waits in `approvals.list` until `approvals.approve` or `approvals.reject`.

Each method has a page in the Engine repository under `docs/api/cep/`.
