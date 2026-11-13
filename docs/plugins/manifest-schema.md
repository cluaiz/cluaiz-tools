# Plugin package fields

All fields are defined in the [package specification](../package-spec.md). This page lists what matters for a plugin. A complete, real manifest with permissions, settings and dependencies is explained in [Plugin package.json](package-json.md).

## Example

```json
{
  "id": "math",
  "name": "Math",
  "category": "utility",
  "hub_type": "plugin",
  "build_type": "wasm",
  "icon": "assets/icon.svg",
  "description": "Evaluates math expressions exactly. Use for any calculation.",
  "license": "Apache-2.0",
  "triggers": ["calculate", "solve equation", "trigonometry"],
  "functions": [
    {
      "name": "math_evaluate",
      "description": "Evaluate a math expression exactly.",
      "parameters": {
        "type": "object",
        "properties": {
          "expression": { "type": "string", "description": "Such as 'sqrt(144) * 25'" }
        },
        "required": ["expression"]
      }
    }
  ],
  "dependencies": { "plugins": {}, "mcp": {}, "skills": {} },
  "permissions": {},
  "latest_version": "1.0.0",
  "versions": {
    "1.0.0": {
      "updated_at": "2026-11-06T12:00:00Z",
      "builds_os": ["wasm"],
      "files": { "readme": "README.md", "skill": "SKILL.md", "icon": "assets/icon.svg" }
    }
  }
}
```

## Fields a plugin must get right

| Field | Why it matters |
|---|---|
| `id` | Must equal the folder and the family key. Release addresses and the installed file are built from it |
| `build_type` | Decides what CI builds and what the Engine downloads. See [Build types](build-types.md) |
| `functions` | Without it the model has nothing to call |
| `description` | What the matcher and the model read first |
| `triggers` | A plain list. This is the only way to add trigger words to a plugin |
| `permissions` | Every type the plugin uses, from the [fixed list](../reference/permissions.md), with `info`, `allow` and the type's options; `{}` when it uses none. Without `network` every `http_fetch` is refused |
| `env` | Every key or setting the plugin reads. A name not listed is never filled in |
| `dependencies` | Other tools with `need` and `info`. Keep `plugins`, `mcp` and `skills`, empty when unused |
| `latest_version` | Bump it in the commit that changes the code, or CI will not build |

## Common mistakes

| Mistake | What happens |
|---|---|
| Trigger words only in `SKILL.md` | The Engine does not read them for a plugin |
| `triggers` as `{ "semantic": [...] }`, or `tags` | No effect for a plugin |
| `id` differs from the folder name | The install looks for a release that does not exist |
| Changing code without changing `package.json` | CI builds nothing |
| Listing `execution` with `args` | `args` is ignored for a plugin. Put the whole line in `command` |
| Expecting the function name in the input | It is not passed. Tell functions apart by their arguments |
| `network` without `access` | `validate_tool` reports it. Use `full`, or `strict` with `hosts` |
| A key in `permissions.secrets`, or `why`/`default` in a permission | No longer read. Use `env`, `info` and `allow` |
