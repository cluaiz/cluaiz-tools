# 💻 Cluaiz CLI Command Reference

The `cluaiz` Command Line Interface (CLI) manages local tool installation, discovery, removal, and tools registry synchronization.

---

## 📦 1. Skill Commands

```bash
# Search for skills on Cluaiz Tools
cluaiz skill search <query>

# Install a skill from Cluaiz Tools
cluaiz skill install <name>

# Install a specific version of a skill
cluaiz skill install <name>@1.2.0

# List all locally installed skills
cluaiz skill list

# Update a skill to latest version
cluaiz skill update <name>

# Remove an installed skill
cluaiz skill remove <name>
```

---

## ⚡ 2. Plugin Commands

```bash
# Search for plugins on Cluaiz Tools
cluaiz plugin search <query>

# Install a plugin
cluaiz plugin install <name>

# List all locally installed plugins
cluaiz plugin list

# Remove an installed plugin
cluaiz plugin remove <name>
```

---

## 🔌 3. MCP Commands

```bash
# Search for MCP servers
cluaiz mcp search <query>

# Install an MCP server bridge
cluaiz mcp install <name>

# List all locally installed MCP bridges
cluaiz mcp list

# Remove an installed MCP bridge
cluaiz mcp remove <name>
```

---

## 🌐 4. Environment Variables

| Variable | Default Value | Description |
|---|---|---|
| `CLUAIZ_HOME` | `~/.cluaiz` | Base root directory for all local tools and configuration. |
| `CLUAIZ_REGISTRY_URL` | Official GitHub Tools Registry URL | Override URL for custom private or corporate registries. |
