# ✍️ Skill Authoring Guide (`SKILL.md`)

The `SKILL.md` file contains the cognitive instructions injected into the LLM's system context. Writing a high-assurance skill requires structured thinking, unambiguous rules, and clear trigger definitions.

---

## 📄 1. File Structure: YAML Frontmatter + Markdown Body

Every `SKILL.md` consists of two distinct parts separated by triple-dashes (`---`):

```markdown
---
name: code-reviewer
version: 1.0.0
description: Strict multi-language code quality, security vulnerability, and DRY enforcement protocol.
author: Aryan

triggers:
  semantic:
    - "review this code"
    - "audit security"
    - "check bugs"

allowed-tools:
  - read_file
  - grep_search
  - git_diff
---

# 🛡️ Code Reviewer Protocol

[Markdown system prompt body starts here...]
```

---

## 🎯 2. Authoring Best Practices

### A. Role Definition & Scope
Clearly define who the agent becomes and what strict boundaries it must enforce:
```markdown
When this skill is activated, you operate as a Principal Systems Software Auditor. Your mission is to identify security vulnerabilities, memory unsafety, duplicate logic, and unhandled edge cases.
```

### B. Unambiguous Checklists
Break complex processes into numbered, verifiable pillars (e.g. Memory Safety, DRY Rules, Error Handling).

### C. Output Formatting Templates
Always provide a strict Markdown report template so the model formats its final answers consistently:
```markdown
## 📋 Structured Output Format
Always format your response using this structure:
### 🔍 Summary
- **Files Inspected:** `[list]`
- **Status:** `[Pass | Fail]`

### 🚨 Detailed Findings
1. **[Issue Title]** (`path/to/file.rs:L123`)
   - **Vulnerability:** `...`
   - **Fix:** `...`
```

---

## 🚫 3. Anti-Patterns to Avoid

| Anti-Pattern | Why it Fails | What to Do Instead |
|---|---|---|
| **The God Prompt** | Bloats VRAM; degrades model attention | Write modular, single-responsibility skills |
| **Vague Guidance ("Write good code")** | Model ignores vague text | Write exact rules with before/after examples |
| **Missing Trigger List** | Router fails to activate skill | Provide 3–6 distinct semantic trigger phrases |
| **Missing Vector Icon** | UI displays broken icon placeholder | Always include `assets/icon.svg` |
