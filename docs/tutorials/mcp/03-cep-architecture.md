---
title: "CEP and MCP"
description: "How MCP servers relate to CEP, and the planned ways for a server to use the Engine."
category: "Tutorials"
---

# 3. CEP and MCP

MCP and CEP are two different protocols with a similar shape. Both use JSON-RPC.

| | MCP | CEP |
|---|---|---|
| Purpose | The Engine uses outside tools | Programs use the Engine |
| Direction | Engine to server | App or script to Engine |
| Standard | Open standard | Specific to Cluaiz |
| Transport | Standard input and output | `POST /v1/cep` |

An MCP server speaks only MCP, so it works in other agents unchanged. The old CEL language is gone. See [From CEL to CEP](../../architecture/cel-to-cep.md).

## How a call flows today

1. The model makes a standard tool call to a function the server listed.
2. The Engine asks for approval if needed.
3. The Engine writes this to the server's standard input:

   ```json
   {
     "jsonrpc": "2.0",
     "id": 1,
     "method": "tools/call",
     "params": {
       "name": "list_pull_requests",
       "arguments": { "owner": "cluaiz", "repo": "cluaiz" }
     }
   }
   ```

4. The server answers on standard output, and the result goes to the model.

A program can start the same flow with CEP `tools.call`, using `"kind": "mcp"`.

## Cluaiz as a server

The other direction also exists: `POST /mcp` and `cluaiz mcp-server` make the Engine an MCP server for clients such as Claude Desktop and Cursor, for read-only methods. See [Engine integration](../../architecture/engine-integration.md#cluaiz-as-an-mcp-server).

## Planned: a server using the Engine

**This is a design. It is not built.** A server has no built-in way to use the Engine's models or other features today. The plan has two parts:

| Need | Planned route |
|---|---|
| The server wants the model to write text | MCP's standard [sampling](https://modelcontextprotocol.io/specification/2025-11-25/client/sampling), so it also works in other agents |
| The server wants another Engine feature | A CEP address and token in its environment, limited to the methods it declares |

Until then, a server should do its own work and return results to the model.
