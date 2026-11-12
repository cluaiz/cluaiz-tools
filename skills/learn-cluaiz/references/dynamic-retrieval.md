# When the answer is not in these files

Never invent a command, flag, setting, API route or CEP method. Find it, or say it is not known.

## Where to look, in order

1. **The installed program.** `cluaiz --help` and `cluaiz <command> --help` list every command and flag of the installed version. In the terminal agent, run them with `run_command`.
2. **The running Engine.** CEP `rpc.discover` lists the methods this Engine accepts; `system.info` gives its version. See [cep-api-reference.md](cep-api-reference.md) for how to call them.
3. **Local documentation.** If the Engine repository is in the workspace, read `docs/` there: `docs/api/` for routes and CEP methods, `docs/cmd/` for the command line and the agent, `docs/automation/` for flows. The tools' format is in cluaiz-tools `docs/`.
4. **The website.** https://cluaiz.com/docs. With a web search tool, search `site:cluaiz.com` or `site:github.com/cluaiz`.

Prefer what the installed version says over a page: Cluaiz is in pre-release and changes between versions.

## In the answer

- Say where the fact came from: the help output, a file path, or a URL.
- If nothing confirms it, say "I could not confirm this" and give the user the command or page to check.
- If the user tells you something about their own setup that should be kept, the `remember` tool in the terminal agent can propose it as a project fact; the user confirms it.
