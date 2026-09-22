---
name: search
description: High performance local workspace documentation and keyword retrieval engine.
version: 1.0.0
triggers:
  - "search docs"
  - "find documentation"
  - "search workspace"
  - "lookup keyword"
execution_mode: auto
default_turns: 1
---

# 🔍 Local Documentation & Search Plugin

This plugin indexes and searches local workspace markdown, source code comments, and technical documentation offline with sub-millisecond retrieval.

## 📋 Input Payload Schema (JSON)
```json
{
  "query": "search query string",
  "limit": 5
}
```

## ⚡ Usage
Emit standard tool call when local project or workspace search is required:
```xml
<tool_call>
{"name": "workspace_search", "arguments": {"query": "<search_term>"}}
</tool_call>
```
