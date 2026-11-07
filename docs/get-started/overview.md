# Overview

Cluaiz Tools gives the Cluaiz Engine new abilities. The model uses them to do real work: run exact calculations, read a project, follow a procedure.

| Type | What it is | Example |
|---|---|---|
| **Skill** | Instructions for the model | Review code with a checklist |
| **Plugin** | A program the Engine runs for the model, usually one WebAssembly file | Evaluate math exactly |
| **MCP server** | An outside program using the open Model Context Protocol | Read a Git repository |
| **Extension** | An App mini-application. Experimental, and not run by the Engine | |

## Try one

```bash
cluaiz plugin install math
cluaiz agent -p "What is 1743 * 2291? Use the math tool."
```

## Next

| You want to | Read |
|---|---|
| Use tools | [Use a plugin](plugin-tutorial.md) |
| Write your first tool | [Quickstart](quickstart.md) |
| Understand the system | [System overview](../architecture/system-overview.md) |
| Choose a type | [Skills, Plugins, MCP and Extensions](../architecture/plugin_vs_mcp_vs_extension_vs_skill.md) |
| Everything | [Documentation map](../README.md) |

## Repository layout

```
cluaiz-tools/
├── registry.json     Index of the families
├── skills/           family.json and skill folders
├── plugins/          family.json and plugin folders
├── mcp/              family.json and server descriptions
├── extensions/       Experimental
└── docs/             This documentation
```
