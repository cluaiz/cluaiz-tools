# Reference Skill package.json

Every reasoning protocol or workflow skill published through Cluaiz Tools contains a `package.json` manifest at its root. This manifest defines capability metadata, prompt routing tags, and references to its core `SKILL.md` instruction file.

---

## Complete Example Manifest

Below is the complete reference `package.json` for the official **Code Reviewer** reasoning skill:

```json
{
  "id": "code-reviewer",
  "name": "Code Reviewer",
  "category": "engineering",
  "hub_type": "skill",
  "icon": "assets/icon.svg",
  "title": "Strict Code Quality Protocol",
  "description": "Multi-language code quality, memory safety, and DRY enforcement reasoning protocol.",
  "author": {
    "name": "Aryan",
    "url": "https://github.com/cluaiz"
  },
  "license": "Apache-2.0",
  "tags": [
    "Code Review",
    "Security",
    "Rust",
    "TypeScript"
  ],
  "dependencies": {
    "plugins": {
      "text": {
        "version": "^1.0.0",
        "url": "https://raw.githubusercontent.com/cluaiz/cluaiz-tools/main/plugins/text/package.json"
      }
    },
    "mcp": {
      "git": {
        "version": "^1.0.0",
        "url": "https://raw.githubusercontent.com/cluaiz/cluaiz-tools/main/mcp/git/package.json"
      }
    },
    "skills": {}
  },
  "latest_version": "1.0.0",
  "versions": {
    "1.0.0": {
      "updated_at": "2026-07-01T12:00:00Z",
      "files": {
        "skill": "/SKILL.md",
        "icon": "/assets/icon.svg",
        "file_directory": "https://github.com/cluaiz/cluaiz-tools/releases/download/v1.0.0/code-reviewer-files.zip"
      }
    }
  }
}
```

---

## Key Manifest Sections

### 1. Identity & Routing
* **`id`**: Unique kebab-case slug for the skill (e.g. `code-reviewer`, `testing-protocol`, `learn-cluaiz`).
* **`hub_type`**: Must be set to `"skill"`.
* **`tags`**: Semantic keywords and trigger phrases used by the Orchestrator to dynamically route incoming tasks to this skill.

### 2. Version & Skill Files
Inside `versions[version]`:
* **`files.skill`**: Relative path to the markdown protocol file (`/SKILL.md`) injected into the LLM context.
* **`files.file_directory`**: Distribution archive containing helper scripts, evaluation schemas, and resources.

### 3. Dependencies
Skills can declare companion plugins and MCP servers in the `dependencies` block. For example, `code-reviewer` references the `git` MCP server to inspect repository diffs directly.

