---
name: text
description: Deterministic string transformation, regex extraction, hashing, and diffing engine.
version: 1.0.0
triggers:
  - "regex extract"
  - "sha256 hash"
  - "base64 encode"
  - "base64 decode"
  - "diff text"
execution_mode: auto
default_turns: 1
---

# 📝 Text & Cryptographic Transformation Plugin

This plugin provides exact string manipulation, pattern matching, cryptographic hashing (SHA256, MD5), base64 encoding/decoding, and structural diffs without LLM transcription errors.

## 📋 Input Payload Schema (JSON)
```json
{
  "action": "hash" | "regex" | "base64_encode" | "base64_decode" | "diff",
  "input": "string to process",
  "pattern": "optional regex pattern",
  "target": "optional diff comparison string"
}
```

## ⚡ Usage
Emit standard tool call when deterministic text transformation, regex extraction, or hashing is required:
```xml
<tool_call>
{"name": "text_transform", "arguments": {"action": "hash_sha256", "input": "text to hash"}}
</tool_call>
```
