---
name: cluaiz-db
version: 1.0.0
description: Core Database Engine for cluaiz
trigger: use plugin::database
---

# Skill: Cluaiz Database (cluaizd)

You are equipped with the cluaiz Core Database Engine. This allows you to persist data, manage users, and perform neural/vector searches.

## Grammar & Usage
To interact with the database, emit a standard tool call:

```xml
<tool_call>
{"name": "cluaizdb_query", "arguments": {"action": "find", "collection": "User", "query": "age >= 18"}}
</tool_call>
```

## Constraints
- You cannot write arbitrary SQL. You must use CDQL syntax.
- The database plugin executes in total isolation. You will receive a CXP pointer containing the results, which the engine will automatically decode into your context window.
