---
name: sysinfo
description: Real-time cross-platform hardware telemetry and resource utilization engine.
version: 1.0.0
triggers:
  - "system info"
  - "hardware specs"
  - "ram usage"
  - "cpu status"
  - "system memory"
execution_mode: auto
default_turns: 1
---

# 💻 System Telemetry & Hardware Info Plugin

This plugin retrieves host CPU, memory, platform, and operating system metrics directly via the CEL Host ABI without external network calls.

## 📋 Response Schema (JSON)
```json
{
  "os_platform": "Windows",
  "arch": "x86_64",
  "memory_free_mb": 16384,
  "status": "nominal"
}
```

## ⚡ Usage
Emit standard tool call when host hardware metrics or system telemetry is required:
```xml
<tool_call>
{"name": "sysinfo_telemetry", "arguments": {}}
</tool_call>
```
