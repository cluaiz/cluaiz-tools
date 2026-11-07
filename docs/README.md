# Cluaiz Tools documentation

This folder explains how tools are written, published, installed and run by the Cluaiz Engine.

## Status words

Every page uses the same three words, so you can tell the design from what runs today.

| Word | Meaning |
|---|---|
| **Works** | The Engine does this in the current release. |
| **Partial** | Part of it works. The page says which part. |
| **Planned** | This is the design. The Engine does not do it yet. Do not build on it. |

**Experimental** applies to Extensions as a whole: they may or may not ship, and the Engine does not run them.

## Read in this order

1. [System overview](architecture/system-overview.md): the repositories and how they connect.
2. [Skills, Plugins, MCP and Extensions](architecture/plugin_vs_mcp_vs_extension_vs_skill.md): what each type is and which one to build.
3. [Registry and discovery](architecture/registry-and-discovery.md): `registry.json` to `family.json` to `package.json`.
4. [Build pipeline](architecture/build-pipeline.md): what gets compiled, and what is used directly.
5. [Engine integration](architecture/engine-integration.md): install, matching, loading, calling and approval.
6. [Package specification](package-spec.md): every `package.json` field and who reads it.

## Where each topic lives

Each topic has one owner page. Other pages link to it instead of repeating it.

| Topic | Page |
|---|---|
| The tool types and how to choose | [plugin_vs_mcp_vs_extension_vs_skill.md](architecture/plugin_vs_mcp_vs_extension_vs_skill.md) |
| `registry.json` and `family.json` | [registry-and-discovery.md](architecture/registry-and-discovery.md), [tools-registry.md](reference/tools-registry.md) |
| Tools that live in another GitHub repository | [decoupled-ecosystem.md](architecture/decoupled-ecosystem.md) |
| `package.json` fields | [package-spec.md](package-spec.md) |
| `package.json` and `tools_registry.json` | [manifest_architecture.md](architecture/manifest_architecture.md) |
| WebAssembly, binary, or no build | [plugins/build-types.md](plugins/build-types.md), [build-pipeline.md](architecture/build-pipeline.md) |
| What a plugin may contain, and dependencies | [plugins/overview.md](plugins/overview.md) |
| The WebAssembly interface | [plugins/wasm-abi-guide.md](plugins/wasm-abi-guide.md) |
| How the Engine uses installed tools | [engine-integration.md](architecture/engine-integration.md) |
| One turn with tools | [turn-lifecycle.md](architecture/turn-lifecycle.md) |
| CEL, the old language, and CEP, the current protocol | [cel-to-cep.md](architecture/cel-to-cep.md) |

## Sections

### Architecture

- [System overview](architecture/system-overview.md)
- [Skills, Plugins, MCP and Extensions](architecture/plugin_vs_mcp_vs_extension_vs_skill.md)
- [Registry and discovery](architecture/registry-and-discovery.md)
- [Tools from other repositories](architecture/decoupled-ecosystem.md)
- [Package and local registry](architecture/manifest_architecture.md)
- [Build pipeline](architecture/build-pipeline.md)
- [Engine integration](architecture/engine-integration.md)
- [Turn lifecycle](architecture/turn-lifecycle.md)
- [Skills at run time](architecture/skill_architecture.md)
- [From CEL to CEP](architecture/cel-to-cep.md)

### Skills

- [Overview](skills/overview.md)
- [Package fields](skills/manifest-schema.md)
- [Authoring guide](skills/authoring-guide.md)
- [Tutorial: first skill](skills/tutorial-first-skill.md)
- Guides: [creating skills](tutorials/skills/01-creating-ai-skills.md), [teaching tools](tutorials/skills/02-prompt-engineering-tools.md), [context](tutorials/skills/03-context-injection.md), [`SKILL.md` for tools](tutorials/skills/04-tool-skills.md)

### Plugins

- [Overview](plugins/overview.md)
- [Build types](plugins/build-types.md)
- [Package fields](plugins/manifest-schema.md)
- [WebAssembly interface](plugins/wasm-abi-guide.md)
- [Tutorial: first plugin](plugins/tutorial-first-plugin.md)
- Guides: [building](tutorials/plugins/01-building-wasm-plugins.md), [settings](tutorials/plugins/02-fetching-settings.md), [CEP and plugins](tutorials/plugins/03-cep-architecture.md), [local testing](tutorials/plugins/04-local-testing.md)

### MCP

- [Overview](mcp/overview.md)
- [Package fields](mcp/manifest-schema.md)
- [Tutorial: connect a server](mcp/tutorial-connect-mcp.md)
- Guides: [connecting](tutorials/mcp/01-connecting-mcp-servers.md), [settings](tutorials/mcp/02-fetching-settings.md), [CEP and MCP](tutorials/mcp/03-cep-architecture.md), [local testing](tutorials/mcp/04-local-testing.md)

### Extensions (experimental)

- [Overview](extensions/overview.md)
- [Package fields](extensions/manifest-schema.md)
- [Developer guide](extensions/developer-guide.md)

### Reference

- [Package specification](package-spec.md)
- [Registry files](reference/tools-registry.md)
- [Skill format](reference/skill-format.md)
- [Plugin format](reference/plugin-format.md)
- [CLI](reference/cli.md)
- [Soul format](reference/soul-format.md) (not supported)

### Get started

- [Overview](get-started/overview.md)
- [Quickstart: write a skill](get-started/quickstart.md)
- [Use a plugin](get-started/plugin-tutorial.md)
- [Contributing](get-started/contributing.md)
