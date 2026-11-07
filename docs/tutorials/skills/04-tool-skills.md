---
title: "SKILL.md for plugins and MCP servers"
description: "How to give a plugin or MCP server instructions for the model."
category: "Tutorials"
---

# 4. `SKILL.md` for plugins and MCP servers

A plugin or MCP server can include a `SKILL.md`. The model gets its body when it loads the tool, along with the function schemas. It is optional and worth adding when the schema alone does not explain how to use the tool.

## Example for a plugin

```markdown
---
name: math
description: Evaluates math expressions exactly. Use for any calculation.
---

# Math

When the user asks for a calculation, call `math_evaluate`. Do not calculate it yourself.

Write the whole expression, such as `sqrt(144) * 25`. Show the `formatted` value from the result.
```

## Rules

| Rule | Why |
|---|---|
| Name the functions exactly | The model must call them by name |
| Put trigger words in `package.json` as a plain list | The Engine does not read them from `SKILL.md` when `package.json` exists. See [matching](../../architecture/engine-integration.md#matching-which-tools-are-offered) |
| Keep the description in both files the same | `package.json` is what the catalog and matcher use |
| Do not repeat the schema | The model already receives it |
| Do not rely on `permissions` here | Security comes from the Engine and `package.json` |

## When to add one

| Add one when | Skip it when |
|---|---|
| Arguments have a format or rules | The function explains itself |
| Order of calls matters | There is one simple function |
| There is a common mistake to prevent | The schema's descriptions are enough |
