# Security Policy

Tools let a model act on your computer and on the internet, so how they are allowed to act matters. This page
says what protects you, where that protection ends, and how to report a problem.

## Supported versions

Cluaiz Tools is in **pre-release**. Security fixes go to `main` and to a new version of the affected tool.

| Version | Supported |
| :--- | :--- |
| The latest release of each tool (1.0.0 and later) | Yes |
| Anything older | No: reinstall the tool with `cluaiz <type> install <id>` |

## Reporting a vulnerability

**Please do not open a public issue for a security problem.** Report it privately:

1. Use GitHub's **Report a vulnerability** button on the
   [Security tab](https://github.com/cluaiz/cluaiz-tools/security), or email `security@cluaiz.com`.
2. Say which tool and version, and give the steps to reproduce it.
3. Describe the impact, for example a plugin reaching a private address, a key leaking to a tool, or a tool
   running without approval.

We will confirm that we received it, work on a fix with you, and agree on when it is made public. Problems in the
Engine itself belong to the [Engine's security policy](https://github.com/cluaiz/cluaiz/blob/main/SECURITY.md).

## What protects you

How much a tool can do depends on its type.

### Plugins (WebAssembly)

Plugins have the strongest isolation. The Engine runs each call in a WebAssembly sandbox:

- **No file access** and no access to programs or environment variables.
- **Memory limit:** 256 MB per call, or less if the manifest asks for less.
- **Instruction limit:** a call that runs forever is stopped with an error.
- **Network only through the Engine,** only if the manifest asks for `network_access`, only over `https`, only to
  public addresses. Local and private addresses are refused, and every redirect is checked again. The one
  exception is an address you set yourself, such as your own SearXNG server.
- **Keys stay hidden:** a plugin writes `{{secret:NAME}}` in a request header, and the Engine fills in the value,
  only for the host the manifest binds it to. The plugin never sees it.

### Skills

A skill is instructions for the model. A skill **with scripts** (such as `deep-research` or `speech`) runs them as
normal programs on your computer, in your working folder, with your user's rights. They are not sandboxed. That is
why:

- Official skill scripts are reviewed and kept small, and use only the language's standard library where possible.
- Install skills from sources you trust, and read the scripts of a skill from anywhere else.

### MCP servers

An MCP server is an outside program, such as `npx @modelcontextprotocol/server-filesystem`. It runs as a separate
process with your user's rights:

- Every MCP package in this registry is **pinned to an exact version**, so a new upstream release cannot change
  what you run without a new release here.
- In the default `sandboxed` mode, **every MCP call asks for approval**, unless you chose to trust the server's
  own read-only marks, or set the server to always allow.
- The Engine stops the server's process when it stops.

### Approvals

The Engine's security mode decides what asks first:

| Mode | What asks for approval |
| :--- | :--- |
| `sandboxed` (default) | MCP calls, tools that declare no capabilities, and tools that can write files, run commands or use the network |
| `strict` | Every tool call, unless you set that tool to always allow |
| `full_access` | Only tools you marked as needing approval |

An approval shows the exact call and its arguments, and a rejected call goes back to the model as an answer, so it
can try another way.

### Text from the outside

Web pages, documents, issues and tool results can contain text written to trick a model ("ignore your
instructions and..."). Tools return such text as **information**, and the official skills tell the model never to
follow it as instructions. This lowers the risk; it cannot remove it for every model. Keep approvals on for actions
that matter.

## Rules for tool authors

Every tool in this registry must follow these rules. A pull request that breaks one is not merged.

- **No secrets in any file.** A plugin asks for a key through `permissions.secrets`; the user sets it with
  `cluaiz plugin config`.
- **Ask for the least access.** No `network_access` if the plugin does not use the network, and only the hosts it
  needs for each secret.
- **Pin every outside package** to an exact version (`agent-browser@0.41.2`, not `agent-browser@latest`).
- **No downloads or installs at run time** without telling the user and asking. A tool that needs a program says
  which one and how to install it.
- **No compiled files** in the repository. CI builds plugins from source.
- **Treat everything a tool reads as data.** Say so in `SKILL.md` when the tool reads web pages, files or messages.
- **Refuse local and private addresses** in any script that fetches URLs, unless the user gave that address.

## What is not covered

- Tools installed from places other than this registry.
- Settings you changed yourself, such as `full_access` mode or always-allow on a tool.
- The models themselves: a model can still give a wrong or harmful answer. Approvals are there for that reason.
