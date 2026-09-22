---
title: "Creating SKILL.md for Tools"
description: "How to properly build a SKILL.md file to give AI the power to understand and use Plugins and MCPs."
category: "Tutorials"
---

# 4. Creating `SKILL.md` for Your Tools

When you build a new tool for cluaiz (whether it's a WASM/Native Plugin or an MCP Server), you are building **Muscle**. 

However, the AI does not automatically know how to use your tool. To give the AI power, context, and exact instructions, you can create a `SKILL.md` file.

### Requirement Levels
- **Native Plugins:** Recommended. You can write a `SKILL.md` to safely guide query construction and parameters.
- **WASM Plugins & MCPs:** Optional, but useful for complex schemas. Without it, the AI calls the tool based on manifest schemas.

---

## The `SKILL.md` Structure

A `SKILL.md` file consists of two parts:
1. **YAML Frontmatter:** For Engine routing and limits.
2. **Markdown Body:** The system prompt injected into the AI.

### Example `SKILL.md` for a Plugin

```markdown
---
name: math-accelerator
title: "Fast Math Accelerator"
version: "1.0.0"
description: "Equips the AI with an exact, hardware-accelerated math calculation tool."
author: "Cluaiz"
keywords: ["math", "calculator", "add", "multiply", "divide"]
triggers:
  semantic: ["calculate", "math problem"]
permissions:
  level: "ReadOnly"
  filesystem: false
  network: false
---

# Math Execution Protocol

You are equipped with the `cluaiz-math-accelerator` plugin. When the user asks you to perform a calculation, you MUST NOT calculate it yourself (to avoid hallucination).

Instead, you must use the plugin by emitting the following CEL command:

`let $result = use plugin::cluaiz-math-accelerator -> invoke(calculate, expr: "<THE_EXPRESSION>");`

Once the engine returns `$result`, you must present the final number clearly to the user.
```

---

## Understanding the Frontmatter

The Engine uses the YAML frontmatter to index the skill and route requests efficiently.

| Field | Purpose |
|-------|---------|
| `name` | Unique kebab-case name of the skill. |
| `description` | Concise summary of the skill's capabilities. |
| `triggers.semantic` | Exact keywords and semantic phrases that activate this skill when matched. |
| `permissions` | Access controls defining whether the tool needs filesystem, network, or read-only access. |

---

## Why Provide a `SKILL.md`?

1. **Eliminate Hallucination:** If you just expose a tool or MCP, the AI might hallucinate commands. By providing a `SKILL.md`, you teach it the *exact* syntax and constraints.
2. **Context Efficiency:** Instead of dumping instructions in a global prompt, the `SKILL.md` is lazily loaded only when a relevant trigger is matched.
3. **Safety Boundaries:** The frontmatter defines explicit `permissions.level` (e.g., `ReadOnly`), ensuring the AI cannot accidentally trick your tool into deleting files.
