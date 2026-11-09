---
name: learn-cluaiz
description: "Explains what Cluaiz is and how its Engine and tools work. Use when the user asks about Cluaiz itself."
---

# Learn Cluaiz

Use this to explain Cluaiz plainly and correctly. If something is not covered here, say so instead of guessing, and point to https://cluaiz.com.

## What it is

Cluaiz is a local AI engine written in Rust. It runs GGUF and ONNX models for chat, vision, speech and embeddings on the user's own GPU, RAM and CPU. It places models automatically and offers OpenAI-compatible APIs.

## Tools

| Type | What it is | Needs on the machine |
|---|---|---|
| Skill | Instructions the model loads when needed (`SKILL.md`) | Nothing |
| Plugin | A program the Engine runs, usually one WebAssembly file in a sandbox | Nothing, for WebAssembly |
| MCP server | An outside program using the Model Context Protocol | Usually Node or `uv` |
| Extension | A mini-application in the App | Experimental |

The model first sees only short cards for the installed tools. When a tool fits, it loads that tool's instructions and functions, and then makes standard tool calls. The Engine runs each call, asks for approval when needed, and returns the result.

A WebAssembly plugin has no files or other programs. It can reach the web only if its `package.json` asks for network access, and then only public https addresses.

Programs talk to the Engine through CEP: JSON-RPC at `POST /v1/cep`, for example the method `tools.call`.

## The tool catalog

| Kind | Tools |
|---|---|
| Plugins | `math`, `text`, `time` (with date arithmetic), `rank`, `web-search` |
| Skills | `code-interpreter`, `code-reviewer`, `git-assistant`, `github`, `build-feature`, `frontend-design`, `speech`, `tool-creator`, `learn-cluaiz` |
| MCP servers | `browser` (also tests websites), `documents` (PDF and Office files), `filesystem`, `git`, `fetch`, `context7`, `everything` |

`speech` uses the Engine's own speech models, so audio never leaves the computer.

## Commands

```bash
cluaiz skill install code-reviewer
cluaiz plugin install web-search
cluaiz mcp install filesystem
cluaiz plugin list
cluaiz plugin config web-search TAVILY_API_KEY
cluaiz agent
```
