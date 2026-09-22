# 🧠 Skills: Cognitive Architecture & Context Injection

In Cluaiz, **Skills** represent the **Cognitive Brain** of the agentic system. While Plugins and MCP servers provide computational muscle (binaries and APIs), a Skill provides the neural reasoning frameworks, domain rules, and prompt guardrails that guide the LLM.

---

## 🎯 1. What is a Skill?

A Skill is a declarative package containing:
- **`package.json`**: Single-source package distribution metadata and dependency resolution rules for Cluaiz Tools.
- **`SKILL.md`**: The structured prompt instructions dynamically injected into the LLM system context.
- **`assets/icon.svg`**: A pure SVG vector icon for native UI rendering.

```
skills/my-skill/
├── package.json         ← Tools registry packaging & dependencies
├── SKILL.md             ← Neural instructions & domain protocol
└── assets/
    └── icon.svg         ← Vector UI icon
```

---

## ⚡ 2. Dynamic KV-Cache Context Injection

Traditional AI applications suffer from the **"God Prompt" Anti-Pattern**—dumping thousands of lines of prompt instructions into the LLM context at boot time. This consumes massive amounts of memory and degrades attention accuracy.

### The Cluaiz Solution: Prefix Caching & Dynamic Context Injection
1. **Dormant by Default:** Skills occupy zero context memory when inactive.
2. **Semantic Activation:** When a user's prompt matches a skill's triggers, the `SkillRouter` activates the skill.
3. **Dynamic Prompt Injection:** The contents of `SKILL.md` are injected into the active turn context.
4. **Prefix Caching & Delta Decoding:** The engine matches prompt prefixes against active KV-cache memory and decodes only new delta tokens, eliminating redundant prefill computation.
5. **Auto-Purge:** Upon turn completion, the prompt is purged from the context window, keeping inference fast and memory-efficient.

---

## 🛡️ 3. Execution Constraints & Guardrails

- **Zero Binary Overhead:** Skills never execute foreign machine code directly.
- **Dependency Binding:** A Skill can declare dependencies on underlying Plugins (`package.json -> dependencies.plugins: { "math": "^1.0.0" }`) to instruct the LLM on which execution muscle to trigger.
- **Determinism:** Skills must be written with unambiguous, structured protocol rules to prevent LLM hallucinations.
