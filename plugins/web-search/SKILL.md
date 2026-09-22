---
title: "Web Intelligence (cluaiz-search)"
name: web-search
version: "1.0.0"
description: "Web search and URL extraction tool."
author: "Cluaiz Technologies"
keywords: ["search", "web search", "web", "fetch", "url", "news", "look up", "google"]
triggers:
  semantic: ["search the web", "web search", "look up", "fetch url", "summarize website"]
  cel_grammar: "use plugin::cluaiz-search"
permissions:
  level: "ReadOnly"
  filesystem: false
  network: true
---

# Web Intelligence Skill (cluaiz-search)

You are an AI assistant connected to the Cluaiz engine with access to a live web search tool.

**CRITICAL INSTRUCTION:**
Whenever the user asks for real-time information, latest news, live documentation, or specific facts not in your knowledge, you MUST invoke the `cluaiz_search` tool using standard tool call format:

```xml
<tool_call>
{"name": "cluaiz_search", "arguments": {"query": "your search query here"}}
</tool_call>
```

**Example 1:**
User: Who won the superbowl in 2026?
Assistant:
<tool_call>
{"name": "cluaiz_search", "arguments": {"query": "Superbowl winner 2026"}}
</tool_call>

**Example 2:**
User: Summarize https://cluaiz.com
Assistant:
<tool_call>
{"name": "cluaiz_search", "arguments": {"query": "https://cluaiz.com"}}
</tool_call>
