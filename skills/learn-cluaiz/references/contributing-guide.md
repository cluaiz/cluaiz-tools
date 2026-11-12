# Contributing

Cluaiz is in pre-release. What helps most is people trying it on their own machines and reporting what works and what breaks.

## What is accepted now

| Contribution | Where | Accepted now |
|---|---|---|
| Bug reports | https://github.com/cluaiz/cluaiz/issues | Yes |
| Hardware reports (works or fails on a CPU, GPU, OS) | Engine issues | Yes, especially macOS, AMD, Intel and low-RAM machines |
| Feedback, ideas, questions | Issues, or Discord https://discord.gg/mab8kBURz | Yes |
| Pull requests to the Engine | | Not yet: they open with the alpha release. An issue describing the change is just as useful now |
| New tools and fixes to tools (skills, plugins, MCP servers) | https://github.com/cluaiz/cluaiz-tools | Yes, start with an issue or a tool proposal |

Security problems are never reported in public; see `SECURITY.md` in the repository.

## A good report

- A bug: the output of `cluaiz --version` and `cluaiz status`, the model, the command or request, what was expected and what happened. Logs help; remove tokens, keys and personal paths first.
- A hardware report: the same, plus `cluaiz bench <model>` and whether it worked.
- Feedback: what the user was trying to do and why it matters.

## A new tool

Each tool in cluaiz-tools stands on its own: a folder with `package.json`, `README.md`, `SKILL.md` and `assets/icon.svg`, listed in its category's `family.json`. The `tool-creator` skill scaffolds one and checks it (`create_tool`, `validate_tool`). The format is in the cluaiz-tools docs (`docs/package-spec.md`, `docs/reference/skill-format.md`).

## Building the Engine from source

For people testing changes locally:

```bash
git clone https://github.com/cluaiz/cluaiz.git
cd cluaiz
cargo build --release -p cmd      # the cluaiz binary, at target/release/cluaiz
```

It needs a Rust toolchain, a C/C++ compiler and CMake (llama.cpp is built as part of it), and the CUDA toolkit for an NVIDIA build. A full build takes a long time; building only `-p cmd` is the usual way.
