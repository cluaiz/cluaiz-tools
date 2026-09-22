# 📋 Skill Package Specification (`package.json`)

This document defines the single-source package specification for Skills in the Cluaiz Tools ecosystem.

---

## 📄 Complete Skill `package.json`

```json
{
  "id": "code-reviewer",
  "name": "Code Reviewer",
  "category": "engineering",
  "hub_type": "skill",
  "logo": "/assets/reviewer.svg",
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

## 🔍 Field Reference Table

| Field | Type | Required? | Description |
| :--- | :---: | :---: | :--- |
| `id` | `string` | **Yes** | Unique skill ID. |
| `name` | `string` | **Yes** | Human-readable title. |
| `hub_type` | `string` | **Yes** | Must be `"skill"`. |
| `dependencies` | `object` | **Yes** | Required muscle plugins or MCP bridges needed by this reasoning skill. |
| `versions.*.files.skill` | `string` | **Yes** | Relative path to `SKILL.md` containing prompt instructions. |
| `versions.*.files.file_directory` | `string` | **Yes** | Release zip bundle download URL. |
