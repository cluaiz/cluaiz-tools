# Web Fetch

Fetches a web page from an HTTP/HTTPS URL and converts its HTML content into readable Markdown using the official `mcp-server-fetch` package.

## Technical Specification

- **Purpose:** Retrieves single web pages as clean Markdown with pagination (`start_index`, `max_length`) so long articles or documentation pages can be read incrementally without overflowing model context.
- **Package Type:** MCP Server (`uvx mcp-server-fetch@2026.11.4`)
- **Platform Support:** Cross-platform (Windows, macOS, Linux)
- **Runtime Requirements:** `uv` (`uvx`) and internet access.

## System Architecture & Execution Flow

```mermaid
flowchart LR
    User["User URL Request"] --> Engine["Cluaiz Engine"]
    Engine -->|"fetch(url, max_length?, start_index?, raw?)"| MCP["mcp-server-fetch (uvx stdio)"]
    MCP --> Robots["1. Check robots.txt Compliance"]
    Robots --> HTTP["2. HTTP GET Request"]
    HTTP --> Convert["3. HTML-to-Markdown Conversion & Chunking"]
    Convert --> Output["Markdown Slice + Continuation Offset"]
```

## Exposed MCP Tool

### Function: `fetch`

| Parameter | Type | Required | Description |
|---|---|---|---|
| `url` | `string` | Yes | Full `http://` or `https://` address of the web page to read. |
| `max_length` | `integer` | No | Maximum number of characters to return in one call (default `5000`). |
| `start_index` | `integer` | No | Character offset to continue reading a truncated page from a previous call. |
| `raw` | `boolean` | No | Set to `true` to return raw HTML instead of converted Markdown (default `false`). |

## Failure & Recovery Logic

- **`robots.txt` Enforcement:** `mcp-server-fetch` respects the target site's `robots.txt` rules for autonomous agents. If a site disallows fetching, the model reports the block directly and does not attempt to bypass it.
- **JavaScript-Only Sites:** `fetch` does not execute client-side JavaScript. If a page renders an empty shell, use the `browser` MCP server instead.

## Installation & Usage

```bash
cluaiz mcp install fetch
```

Example prompt:

```text
Fetch https://blog.rust-lang.org and summarize the latest release announcement.
```

