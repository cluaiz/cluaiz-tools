---
title: "Teaching the model to use tools"
description: "How a skill names tools, so the model calls them correctly."
category: "Tutorials"
---

# 2. Teaching the model to use tools

The model is probabilistic. A vague instruction gets a guess. A skill should name the exact function and say when to call it.

The model calls tools with standard tool calls, using the schema from `functions` or from the MCP server. A skill does not teach any special syntax. Older versions of this page taught CEL, which no longer exists. See [From CEL to CEP](../../architecture/cel-to-cep.md).

## Name the function

**Vague**

```markdown
If the user asks about their balance, use the database.
```

The model may invent a query or a tool that does not exist.

**Clear**

```markdown
If the user asks about their balance, call `get_balance` with their user id.
Wait for the result before answering. Do not guess the number.
```

## Chain tools with steps

The model passes results from one tool to the next. Write it as steps:

```markdown
To analyse an issue:

1. Call `read_issue` with the issue number.
2. Call `text_transform` on the issue text with `action` set to `word_count`.
3. Summarise both results for the user.
```

## Load what you name

The tools a skill names may not be loaded yet. If a function is not available, the model can use `find_tools` and `load_tool`. Say so when it matters:

```markdown
This skill needs the `git` MCP server. Load it with `load_tool` if its functions are not available.
```

## Handle failure

Say what to do when a tool fails or is missing:

```markdown
If `git_diff` is not available, ask the user to run `cluaiz mcp install git`.
If it returns an error, show the error and stop.
```

## Small models

Short and direct works best. Give one example of the expected result, and avoid long explanations of why.
