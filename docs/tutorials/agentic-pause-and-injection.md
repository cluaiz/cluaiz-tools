---
title: "Loading a tool during an answer"
description: "How the model gets a tool's instructions only when it needs them."
category: "Plugins"
---

# Loading a tool during an answer

The model does not start with every tool in its prompt. It sees short cards, and loads a tool when it needs it. This keeps the context small, which matters most on small models.

An older design did this with a `<TRIGGER:plugin:x>` token and a pause in generation. That is gone. The current way uses standard tool calls.

## The steps

1. The Engine matches the user's message against installed tools, and adds short cards and the function `load_tool`.
2. The model calls `load_tool` with the tool's id, like any other tool call.
3. The Engine answers with the tool's `SKILL.md` body and its function schemas, and generation continues in a new round.
4. The model calls one of the functions.
5. The Engine runs it and returns the result, and the model answers.

The Engine reuses its KV cache for the part of the prompt that did not change, so loading a tool does not process the whole conversation again.

[Engine integration](../architecture/engine-integration.md) and [Turn lifecycle](../architecture/turn-lifecycle.md) have the details.
