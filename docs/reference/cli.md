# CLI for tools

The commands are the same for `skill`, `plugin` and `mcp`.

```bash
cluaiz plugin install math          # The latest version, with its dependencies by your mode
cluaiz plugin install math@1.0.0    # A listed version
cluaiz plugin list                  # What is installed
cluaiz plugin remove math
cluaiz plugin cache                 # Cached downloads

cluaiz plugin permission time               # Its permissions: allowed or refused
cluaiz plugin permission time network off   # Refuse one; `on` allows it again

cluaiz plugin config web-search                     # The settings it reads (its `env`)
cluaiz plugin config web-search TAVILY_API_KEY      # Set one; asked for hidden
cluaiz plugin config web-search SEARXNG_URL http://localhost:8888
```

`install` shows a plan first: the tool, each dependency by its `need`, and each permission. `--yes` accepts all of it, and `--no-recommended` takes only what is required. See [Installing by the user's mode](../package-spec.md#installing-by-the-users-mode).

A setting is read from the system environment variable of the same name first; `config` sets the value Cluaiz keeps when there is none.

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
| A name in a tool's `env` | That tool's setting, such as `TAVILY_API_KEY` |

## Commands that do not exist

Older pages list these: `cluaiz skill search`, `cluaiz skill update`, `cluaiz registry sync`, `cluaiz list --all` and `cluaiz chat`. To update a tool, run `install` again.
