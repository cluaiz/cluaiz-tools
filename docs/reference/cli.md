# CLI for tools

The commands are the same for `skill`, `plugin` and `mcp`.

```bash
cluaiz plugin install math          # The latest version
cluaiz plugin install math@1.0.1    # A listed version
cluaiz plugin list                  # What is installed
cluaiz plugin remove math
cluaiz plugin config math           # View or change its settings
cluaiz plugin cache                 # Cached downloads
cluaiz mcp start filesystem         # Start an MCP server in the background
cluaiz plugin link                  # Link a plugin to a skill
```

Each exits with a non-zero code on failure.

`install` takes an id from the registry, not a URL or a path. A folder copied by hand into `~/.cluaiz/tools/<type>/<id>/` is also picked up.

## Using tools

```bash
cluaiz agent                        # The coding agent in this folder
cluaiz agent -p "What is 17 * 23?"  # One task, then exit
cluaiz run <model> "..."            # Chat with a model
```

In the agent, `/skills` lists the skills it found.

## Programs

Other programs use CEP at `POST /v1/cep` while `cluaiz serve` runs. See [Engine integration](../architecture/engine-integration.md#managing-tools-through-cep).

## Environment

| Variable | Effect |
|---|---|
| `CLUAIZ_HOME` | Use another folder instead of `~/.cluaiz` |

## Commands that do not exist

Older pages list these: `cluaiz skill search`, `cluaiz skill update`, `cluaiz registry sync`, `cluaiz list --all` and `cluaiz chat`. To update a tool, run `install` again.
