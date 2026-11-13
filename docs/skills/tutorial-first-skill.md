# Tutorial: first skill

Build `commit-helper`, a skill that writes commit messages.

## 1. Create the folder

```bash
mkdir -p commit-helper/assets
cd commit-helper
```

## 2. Write `SKILL.md`

```markdown
---
name: commit-helper
description: Writes clear Git commit messages from the current changes. Use when the user asks for a commit message or wants to commit.
license: Apache-2.0
---

# Commit messages

1. Run `git diff --staged`. If it is empty, run `git diff`.
2. Find the one main purpose of the change.
3. Write one line of at most 72 characters: `type(scope): what changed`.
4. Show the message and ask before committing.

Types: `feat`, `fix`, `docs`, `refactor`, `test`, `chore`. Use the repository's own style when its history shows one.
```

This one file is already a working skill.

## 3. Try it

Copy the folder into your project, then start the agent:

```bash
mkdir -p ../my-project/.agents/skills
cp -r ../commit-helper ../my-project/.agents/skills/
cd ../my-project
cluaiz agent
```

Type `/skills` and check that it is listed. Then ask: `write a commit message for my changes`.

## 4. Prepare to publish

Add `README.md` (human-facing documentation with a Mermaid architecture diagram for the website and catalog), `assets/icon.svg` (a pure SVG with a 24 by 24 `viewBox`), and `package.json`:

```json
{
  "id": "commit-helper",
  "name": "Commit Helper",
  "category": "engineering",
  "hub_type": "skill",
  "icon": "assets/icon.svg",
  "description": "Writes clear Git commit messages from the current changes. Use when the user asks for a commit message or wants to commit.",
  "license": "Apache-2.0",
  "triggers": { "semantic": ["commit message"] },
  "dependencies": {
    "plugins": {},
    "mcp": {
      "git": {
        "version": "^1.0.0",
        "url": "https://raw.githubusercontent.com/cluaiz/cluaiz-tools/main/mcp/git/package.json",
        "need": "optional",
        "info": "Git tools, when the agent cannot run git itself"
      }
    },
    "skills": {}
  },
  "permissions": {},
  "latest_version": "1.0.0",
  "versions": {
    "1.0.0": {
      "updated_at": "2026-11-06T12:00:00Z",
      "files": { "readme": "README.md", "skill": "SKILL.md", "icon": "assets/icon.svg" }
    }
  }
}
```

`permissions` is `{}` because this skill has no script: the model runs git with its own tools, which ask for approval as usual. `git` is `optional`, so it is only suggested at install. A skill with a script lists what the script uses; see [Skill package.json](package-json.md#permissions).

## 5. Publish

1. Put the folder at `skills/commit-helper/`.
2. Add it to `skills/family.json`:

   ```json
   "commit-helper": { "type": "local", "path": "commit-helper/package.json" }
   ```

3. Push. CI makes the zip and the release.
4. Test with `cluaiz skill install commit-helper` in a clean `CLUAIZ_HOME`.
