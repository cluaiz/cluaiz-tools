---
name: tool-creator
description: "Creates and improves tools for Cluaiz: skills (instructions with optional scripts), WebAssembly plugins and MCP server packages, with correct manifests and registry entries. It also checks a tool package for problems and tests it. Use when the user wants a new tool, skill, plugin or MCP integration, wants an existing tool improved, or asks whether a tool package is correct."
---

# Tool Creator

Links such as `references/...` are files of this skill. Read them with `read_tool_file`. In the terminal agent, `read_item` with `tool-creator` gives the folder; then call `read_file` on the folder plus the link.

## Understand the need

- Find out what the tool should do: its input, its output, and who uses it (the model in a chat, the terminal agent, or an App). Ask one short question when the answer changes the design.
- Check that no tool does this already: `find_tools` in a chat, `cluaiz skill list`, `cluaiz plugin list` or `cluaiz mcp list`, or the `family.json` files in `cluaiz-tools`. Extend an existing tool rather than making a second one.
- When the tool goes into a repository, read that repository's `AGENTS.md` and `CONTRIBUTING.md`.

## Choose the type

| The tool must | Type |
|---|---|
| Teach a way of working | A skill: [references/skill-authoring.md](references/skill-authoring.md) |
| Do a fragile, mechanical step reliably (parse, check, call an API) | A skill with Python functions (same file) |
| Compute from its input or call public web addresses, with nothing to install | A WebAssembly plugin: [references/wasm-plugin-authoring.md](references/wasm-plugin-authoring.md) |
| Use files, a database or other programs, or wrap an existing MCP server | MCP: [references/mcp-server-authoring.md](references/mcp-server-authoring.md) |
| Run as a `/command`, or on a schedule or event | A workflow or a flow, not a tool: [references/workflow-and-flow-authoring.md](references/workflow-and-flow-authoring.md) |

A WebAssembly plugin cannot read files or start programs. Use MCP for that.

## Create

- **Call `create_tool`.** Pass the category, a `tool_id`, a name, a description and triggers. An MCP tool also needs `command` and `args`, with the package version pinned. The tool is written to the Engine's tools folder. When the open folder is `cluaiz-tools`, it is also added there and to `family.json`. An existing tool is replaced only with `overwrite: true` and the user's yes.
- **Description:**
  - Written in the third person.
  - Says what the tool does and when to use it, in the words a user would say.
  - Broad enough to cover neighbouring tasks.
  - The model chooses tools from the description, so it is the most important line.
- **Functions:**
  - A verb-first name.
  - A one-sentence description.
  - Each parameter has a type and a short description.
  - Keep them short: in a small context, the schemas of all installed tools share about 8,000 characters.
- **Next steps:** do only what `next` lists, and add no files the user did not ask for.
- **Plugins:**
  - Write the logic in `src/lib.rs` and build with `cargo build --release --target wasm32-unknown-unknown`.
  - Copy the file in as `<tool_id>.wasm`, and update `functions` when the inputs change.
  - The Engine passes only the arguments, not the function name, so a plugin with several functions tells them apart by their arguments.

## Check and test

1. **Validate.** Call `validate_tool` with `tool_id` or `path`. Fix every error, read the warnings, and call it again. What it cannot see: [references/validation-rubric.md](references/validation-rubric.md).
2. **Run the code directly.**
   - A script: with a small JSON file of arguments.
   - A plugin: with `cargo test`.
   - An MCP server: start it and check that it lists its tools.
3. **Run it through a model.** Use good input and bad input. Bad input must give a clear error, not a crash.
4. **Test a skill as an evaluation.**
   - Pick two or three real tasks, and run them with and without the skill.
   - Use a small model and a large model.
   - Fix what the small model gets wrong.
   - Cut whatever the large model did not need.
5. **Report.** Say what you tested and what you did not. Do not say it works before it has run.

## Publish

Tools are shared through the `cluaiz-tools` repository, which takes pull requests. When the user wants a tool published, and it validates and passes its tests, commit it and open a pull request with the `git-assistant` and `github` skills.

## Rules

- Never put keys or passwords in a tool's files. A plugin asks for them through `permissions.secrets`, an MCP server through `user_env`.
- `SKILL.md` is for the model: short judgement rules and limits, with no diagrams or long examples. `README.md` is for people: it explains what the tool does and includes a Mermaid diagram with quoted labels.
- Links are relative to the tool folder, and every file is listed in `versions.<version>.files`.
- The family key, the folder name and the `id` stay the same.
