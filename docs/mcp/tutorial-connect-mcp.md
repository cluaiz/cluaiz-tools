# Tutorial: connect an MCP server

Connect the official `everything` test server. Its tools exercise the protocol, which makes it good for a first check. It needs Node.js.

## 1. Check the server by itself

```bash
npx -y @modelcontextprotocol/server-everything
```

It waits for input, which is expected. Press Ctrl+C.

## 2. Install it by hand in a test home

Using a test home leaves your real installation untouched.

**PowerShell**

```powershell
$env:CLUAIZ_HOME = "$PWD\test-home"
$dir = "$env:CLUAIZ_HOME\tools\mcp\everything"
New-Item -ItemType Directory -Force $dir | Out-Null
```

**Linux and macOS**

```bash
export CLUAIZ_HOME="$PWD/test-home"
dir="$CLUAIZ_HOME/tools/mcp/everything"
mkdir -p "$dir"
```

Write this to `package.json` in that folder:

```json
{
  "id": "everything",
  "name": "MCP Everything",
  "category": "mcp",
  "hub_type": "mcp",
  "icon": "assets/icon.svg",
  "description": "The official MCP test server. Use it to check MCP tools. Needs Node.js.",
  "license": "MIT",
  "triggers": ["mcp test"],
  "execution": { "command": "npx", "args": ["-y", "@modelcontextprotocol/server-everything"] },
  "latest_version": "1.0.0",
  "versions": {
    "1.0.0": { "updated_at": "2026-11-06T12:00:00Z", "files": { "icon": "assets/icon.svg" } }
  }
}
```

## 3. Check that Cluaiz sees it

```bash
cluaiz mcp list
```

## 4. Call a tool

In the same shell, start `cluaiz serve`. Then send to `POST /v1/cep`:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools.call",
  "params": { "kind": "mcp", "name": "everything", "fn": "echo", "input": { "message": "hello" } }
}
```

The server starts on this first call, and the first call may be slow while `npx` downloads it.

## 5. Publish

1. Put the folder at `mcp/everything/` with `assets/icon.svg`.
2. Add it to `mcp/family.json`:

   ```json
   "everything": { "type": "local", "path": "everything/package.json" }
   ```

3. Push, then test with `cluaiz mcp install everything` in a clean `CLUAIZ_HOME`.
