# Contributing a tool: checklist

The process, the pre-release policy and best practices are in [CONTRIBUTING.md](../../CONTRIBUTING.md). This page is the checklist for the pull request.

## Pull requests

**One pull request, one purpose:** a new tool, a fix to a tool, or documentation.

Use [Conventional Commits](https://www.conventionalcommits.org/) in the title:

```
feat(commit-helper): add a skill for commit messages
fix(math): handle division by zero
docs: explain script plugins
```

Say what changed and why.

## Before you open it

For every tool:

- [ ] The family key, the folder name and `id` in `package.json` are the same, in lowercase kebab-case.
- [ ] `README.md` is present for human users and website display (`cluaiz.com/tools`), explaining the tool's purpose, system architecture, a Mermaid flow diagram, inputs/outputs and usage (distinct from `SKILL.md`, which is only for the AI model).
- [ ] `"readme": "README.md"` is listed under `versions.<v>.files` in `package.json`.
- [ ] `assets/icon.svg` is a pure SVG with a 24 by 24 `viewBox`.
- [ ] `description` says what it does and when to use it.
- [ ] Trigger words are in `package.json`, in the form for its type. See the [package specification](../package-spec.md#what-the-model-sees).
- [ ] `permissions` lists every kind the tool uses, each with `info` and `allow`; `network` has `access` (`full`, or `strict` with `hosts`). `{}` when it uses none. See [Permissions](../package-spec.md#permissions).
- [ ] Every key or setting the tool reads is named in `env`, with `info` and `need`. See [Env](../package-spec.md#env).
- [ ] `dependencies` has `plugins`, `mcp` and `skills`, and each entry has `need` and `info`. See [Dependencies](../package-spec.md#dependencies).
- [ ] `validate_tool` from the `tool-creator` skill reports no errors.
- [ ] A new tool starts at `1.0.0`. A change to an existing tool raises `latest_version` and the key under `versions`, or CI will not release it.
- [ ] It was tested with a small model (around 4B, 8K context), and the pull request names the model and the prompt.
- [ ] It follows the [rules for tool authors](../../SECURITY.md#rules-for-tool-authors).
- [ ] It works when installed in a clean `CLUAIZ_HOME`, and you called it.
- [ ] It contains no secrets and no compiled files.

For a skill:

- [ ] `name` in `SKILL.md` equals the folder name.
- [ ] Frontmatter uses the [standard fields](../reference/skill-format.md#standard-fields).
- [ ] Every file is listed under `versions.<v>.files`.

For a plugin:

- [ ] `functions` has a clear description and JSON schema for each function.
- [ ] A WebAssembly plugin builds with `wasm32-unknown-unknown` and has tests.
- [ ] `build_type` is not `binary`: the Engine cannot run those yet.

For an MCP server:

- [ ] The command is in `execution`.
- [ ] The description names what the user needs, such as Node.js.
- [ ] Required environment variables are documented.

## Licences

Include a licence that allows distribution, and name it in `license`. For third-party code or an outside server, keep the author's licence and credit.

## Documentation rules

- English only.
- Each topic has one owner page. See the [documentation map](../README.md#where-each-topic-lives). Link to it instead of repeating it.
- Use **Works**, **Partial** or **Planned** whenever a feature is not obvious.
- Do not claim speed or memory numbers without a benchmark you can reproduce.
