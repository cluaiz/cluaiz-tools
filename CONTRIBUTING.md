# Contributing to Cluaiz Tools

Thank you for helping. Cluaiz is in **pre-release**, and what helps most right now is people using the tools and
telling us what breaks, what is missing and what could be better.

## What we accept during the pre-release

| Contribution | Cluaiz Tools (this repository) | [Cluaiz Engine](https://github.com/cluaiz/cluaiz) |
| :--- | :--- | :--- |
| Bug reports | **Yes** | **Yes** |
| Hardware reports ("it works / it fails on my machine") | **Yes** | **Yes** |
| Feedback, ideas, "this is missing" | **Yes** | **Yes** |
| Fixes to an existing tool | **Yes**, after an issue | Not yet |
| New tools | **Yes**, after a tool proposal | Not yet |
| Documentation fixes | **Yes** | Not yet |

**Why the Engine does not take pull requests yet:** its base architecture is built and stable, but it can still
change while it is tested on many machines in this pre-release. Merging outside code now would mean asking people to
rebase again and again. Pull requests to the Engine open once the pre-release ends with the alpha. Until then,
issues, reports and discussion there are very welcome.

**Tools are different:** each tool is small and stands on its own, and the tool formats (skills, MCP, WebAssembly)
are open standards. So new tools and fixes are welcome here now.

## Before you start

- **Join the [Discord](https://discord.gg/mab8kBURz)** if you want to contribute. It has channels for contributors,
  and it is the quickest way to agree on an idea before you build it.
- **Open an issue first.** For a fix, a [bug report](https://github.com/cluaiz/cluaiz-tools/issues/new/choose).
  For a new tool, a **tool proposal**: what it does, which type, what it needs. A pull request without an issue may
  be closed, so that nobody spends days on something that cannot be merged.
- **Security problems** are never reported in public. See [SECURITY.md](SECURITY.md).

## Build a tool for yourself

You do not need to contribute to use your own tool. Write it, and use it:

- A skill in `.agents/skills/<name>/SKILL.md` inside your project works there at once.
- Any tool folder in `~/.cluaiz/tools/<type>/<id>/` is picked up like an installed one.
- The `tool-creator` skill can write the first version for you.

Start with the [quickstart](docs/get-started/quickstart.md). Everything below also helps for a personal tool.

## Add a tool to the registry

### 1. Choose the type

| The tool must | Type |
| :--- | :--- |
| Teach the model a way of working, with no program | **Skill** |
| Compute from its input, or call public `https` services, with nothing for the user to install | **Plugin** (WebAssembly) |
| Use files, a database, a browser or other programs, or an MCP server for it already exists | **MCP server** |

Prefer the lightest type that works. A WebAssembly plugin cannot read files or start programs; if the tool must,
use MCP. [Which type?](docs/architecture/plugin_vs_mcp_vs_extension_vs_skill.md)

### 2. Create the folder

```text
<type>/<id>/
├── package.json     # id, name, description, triggers, functions, permissions, version, files
├── SKILL.md         # short instructions for the model
├── README.md        # for people: what it does, how it works, how to use it
├── assets/icon.svg  # 24 by 24 viewBox
└── src/ or scripts/ # plugin source (Rust) or skill scripts, if any
```

Add the tool to `<type>/family.json`. The family key, the folder name and `id` must be the same, in lowercase
kebab-case. A new tool starts at version `1.0.0`.

Guides: [first skill](docs/skills/tutorial-first-skill.md), [first plugin](docs/plugins/tutorial-first-plugin.md),
[connect an MCP server](docs/mcp/tutorial-connect-mcp.md), [package specification](docs/package-spec.md).

### 3. Test it with a small model

A tool must work for the models people really run, not only for large ones:

1. Install it into a clean home: `CLUAIZ_HOME=/tmp/cluaiz-test cluaiz <type> install <id>` (or copy the folder
   into that home's `tools/` folder).
2. Ask for it the way a user would, with a small model (around 4B parameters) and an 8K context.
3. Check that the model finds the tool, calls it with good arguments, and gives a correct answer.
4. Call it once with bad input. It must answer with a clear error, not a crash or a hang.

Write in the pull request which model you used and what you asked.

### 4. Open the pull request

- **One pull request, one purpose:** one new tool, one fix, or one documentation change.
- Title in [Conventional Commits](https://www.conventionalcommits.org/) form: `feat(word-count): add a plugin that
  counts words`, `fix(math): handle division by zero`.
- For a change to an existing tool, raise its version in `package.json` (`latest_version` and the key under
  `versions`). CI releases only tools whose `package.json` changed.
- Fill in the checklist in [docs/get-started/contributing.md](docs/get-started/contributing.md).

## Best practices

**Descriptions and triggers decide whether a tool is used.** The model sees one line per tool until it loads one.

- The `description` says what the tool does **and when to use it**: "Counts words in text. Use when the user asks
  how long a text is."
- Triggers are words a user really types: "word count", "how many words", not "lexical metrics".
- Keep `SKILL.md` short and direct: exact steps, exact function names, limits. It is read by a model with little
  context.

**Results must fit a small context.**

- Return the useful part, not everything: a short summary with the details on disk or behind a second call.
- Say what to do next in the result ("No useful sources for Q2; continue with Q3"), so a small model does not loop.
- Fail clearly: "The Engine is not running. Tell the user to start it with `cluaiz serve`" beats a stack trace.

**Safety rules** are in [SECURITY.md](SECURITY.md#rules-for-tool-authors): no secrets in files, the least access,
pinned versions, no installs at run time without asking, and outside text treated as data.

## What not to do

- Do not add a tool that duplicates one already in the registry; improve the existing one.
- Do not commit compiled files (`.wasm`, `.dll`, `node_modules`, `target/`).
- Do not use `@latest` or unpinned packages in an MCP command.
- Do not claim a tool works, or how fast it is, without having run it.
- Do not change other tools in the same pull request.

## Licenses

Your contribution is released under the [Apache License 2.0](LICENSE), like the rest of this repository. A tool that
starts or bundles someone else's software names that software's license in `package.json` and credits its authors.

## Code of conduct

Be respectful and helpful. The [Code of Conduct](https://github.com/cluaiz/cluaiz/blob/main/CODE_OF_CONDUCT.md) of
the Cluaiz project applies here too.
