# Architecture and duplication

Use this when a change adds modules, helpers or dependencies, or moves code between layers.

## Duplicated logic

- Search before accepting a new helper: the same parsing, validation, retry or formatting logic may already exist. A quick `grep` for the function's key words or literals usually finds it.
- Report duplication when the copies must change together. Two copies of a URL parser, a price rule or a permission check will drift apart and one of them will be wrong.
- Do not report small look-alike code that changes for different reasons. Forcing it into one function couples unrelated parts.
- The fix is to call the existing function, or move the shared part into one place both callers import.

## Boundaries

- Each layer should do its own job. Typical rules: core logic does no terminal output, prompts or UI work; UI code does not open database connections; request handlers do not contain business rules that other entry points also need.
- A dependency should point inward: UI to service to core, not core back to UI.
- A new dependency (crate, package, service) is worth a question: is it needed, is it maintained, does the project already have one that does this?

## Hardcoded values

- Machine-specific paths, user names, ports, hostnames and keys do not belong in code. They come from configuration, the environment or arguments.
- Lists of special cases written into logic (model names, customer ids, file names) usually mean a rule is missing. Ask what decides the case, and read it from data instead.
- Constants that are part of a protocol or format (an HTTP status, a file signature) are fine as named constants.

## Project rules

Many projects state their own rules in `AGENTS.md`, `CONTRIBUTING.md`, `.agents/rules/` or a style guide. Read them, and check the change against them. A broken project rule is a finding even when the code works.
