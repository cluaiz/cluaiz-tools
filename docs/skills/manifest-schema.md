# Skill package fields

All fields are defined in the [package specification](../package-spec.md). This page lists what matters for a skill.

## Example

```json
{
  "id": "code-reviewer",
  "name": "Code Reviewer",
  "category": "engineering",
  "hub_type": "skill",
  "icon": "assets/icon.svg",
  "title": "Code review checklist",
  "description": "Reviews code for bugs, security problems and duplication. Use when the user asks for a review.",
  "license": "Apache-2.0",
  "tags": ["Code Review", "Security"],
  "triggers": { "semantic": ["review this code", "check for bugs"] },
  "dependencies": { "plugins": {}, "mcp": {}, "skills": {} },
  "latest_version": "1.0.0",
  "versions": {
    "1.0.0": {
      "updated_at": "2026-11-06T12:00:00Z",
      "files": {
        "skill": "SKILL.md",
        "icon": "assets/icon.svg",
        "assets": ["references/checklist.md"]
      }
    }
  }
}
```

## Fields a skill must get right

| Field | Why it matters |
|---|---|
| `id` | Must equal the folder, the family key and `name` in `SKILL.md` |
| `description` | What the matcher and the model read first. Say what it does and when to use it |
| `triggers.semantic`, `tags` | Extra words for matching. Both work for skills |
| `versions.<v>.files` | **List every file.** A skill without a release is installed from this list, and a file not listed is not installed |
| `build_type` | Leave it out, or `none`. Skills do not compile |

## With functions

A skill can also offer functions by running a script. Add `functions` and `execution`, and the skill behaves as a [script plugin](../plugins/build-types.md#script-plugins). `code-interpreter` and `tool-creator` work this way.
