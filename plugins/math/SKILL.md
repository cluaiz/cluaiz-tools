---
name: math
description: Deterministic high-precision mathematical evaluation engine. Eliminates LLM calculation and arithmetic hallucination.
version: 1.0.0
triggers:
  - "calculate"
  - "solve equation"
  - "math expression"
  - "trigonometry"
  - "evaluate math"
execution_mode: auto
default_turns: 1
---

# 📐 Math Evaluation Plugin

This plugin provides exact, deterministic arithmetic, trigonometry, calculus, and large-number computations inside an isolated WASM sandbox with zero hallucination.

## 📋 Input Payload Schema (JSON)
```json
{
  "expression": "2^64 - 1",
  "precision": 10
}
```

## ⚡ Usage
Emit standard tool call when exact numeric precision is required:
```xml
<tool_call>
{"name": "math_evaluate", "arguments": {"expression": "<math_expr>"}}
</tool_call>
```
