# 🚀 Tutorial: Build Your First Cluaiz Skill

In this hands-on tutorial, we will build, package, and test a complete Skill from scratch: **`git-assistant`**.

---

## 🛠️ Step 1: Create Directory Structure

Create a new directory in your local workspace or `~/.cluaiz/tools/skills/`:

```bash
mkdir -p git-assistant/assets
cd git-assistant
```

---

## 📦 Step 2: Create `package.json`

Create `package.json` with package metadata and dependencies:

```json
{
  "id": "git-assistant",
  "name": "Git Assistant",
  "category": "engineering",
  "hub_type": "skill",
  "logo": "/assets/icon.svg",
  "title": "Git Workflow Protocol",
  "description": "Assists developers with Git commit conventions, branch workflows, and merge conflict resolution.",
  "author": {
    "name": "Aryan",
    "url": "https://github.com/cluaiz"
  },
  "license": "Apache-2.0",
  "tags": [
    "Git",
    "DevOps",
    "Workflow"
  ],
  "dependencies": {
    "plugins": {},
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
        "file_directory": "https://github.com/cluaiz/cluaiz-tools/releases/download/v1.0.0/git-assistant-files.zip"
      }
    }
  }
}
```

---

## ✍️ Step 3: Create `SKILL.md`

Create `SKILL.md` with structured instructional rules:

```markdown
---
id: cluaiz.skill.devops.git-assistant
name: git-assistant
triggers:
  semantic:
    - "git help"
    - "write commit message"
    - "git workflow"
    - "resolve merge conflict"
---

# 🌿 Git Workflow Protocol

When this skill is active, you enforce strict Git best practices:

## 1. Conventional Commits Standard
Format all commit messages strictly as: `<type>(<scope>): <subject>`
- Types: `feat`, `fix`, `docs`, `style`, `refactor`, `perf`, `test`, `chore`.
- Imperative tone: "add feature" NOT "added feature".

## 2. Merge Conflict Resolution
When explaining merge conflicts, show exact 3-way conflict markers (`<<<<<<<`, `=======`, `>>>>>>>`).
```

---

## 🎨 Step 4: Add `assets/icon.svg`

Create `assets/icon.svg` for UI rendering:

```xml
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="24" height="24" fill="none" stroke="#10B981" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
  <circle cx="18" cy="18" r="3"/>
  <circle cx="6" cy="6" r="3"/>
  <circle cx="6" cy="18" r="3"/>
  <path d="M18 9a9 9 0 0 1-9 9"/>
  <line x1="6" y1="9" x2="6" y2="15"/>
</svg>
```

---

## 🧪 Step 5: Test Locally

Copy the folder to `~/.cluaiz/tools/skills/git-assistant/`. The Cluaiz engine will automatically probe and register the new skill on the next inference query!
