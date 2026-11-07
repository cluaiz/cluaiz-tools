# Quickstart: write a skill

A skill is the quickest tool to write. It needs one file and no build.

## 1. Create it in your project

```bash
mkdir -p .agents/skills/summarizer
```

Write `.agents/skills/summarizer/SKILL.md`:

```markdown
---
name: summarizer
description: Condenses long text into short bullet points. Use when the user asks to summarize or extract key points.
license: Apache-2.0
---

# Summarize

1. Read all of the text.
2. Find the 5 to 10 most important points.
3. Write them as short bullets, most important first.
4. Keep names, numbers and dates exactly as written. Add nothing that is not in the text.
```

The frontmatter is the metadata, and the body is the prompt.

## 2. Try it

```bash
cluaiz agent
```

Type `/skills` to see it listed. Then ask: `summarize README.md`.

## 3. Improve it

Change the file and try again. No install step is needed. If the model does not use the skill, make the description more specific.

## 4. Publish it

See [Tutorial: first skill](../skills/tutorial-first-skill.md#4-prepare-to-publish), which adds `package.json`, the icon and the family entry.

## Next

- [Authoring guide](../skills/authoring-guide.md)
- [Tutorial: first plugin](../plugins/tutorial-first-plugin.md)
- [Tutorial: connect an MCP server](../mcp/tutorial-connect-mcp.md)
