# The `web-search` plugin

Searches the web and reads pages. It is one WebAssembly file that needs nothing on the machine, and an Engine that provides `cluaiz.http_fetch`.

```bash
cluaiz plugin install web-search
```

| Function | Input | Result |
|---|---|---|
| `web_search` | `query`, optional `max_results` | Titles, links and snippets |
| `web_fetch` | `url`, optional `max_chars` | The page's title, description and text |

Without any setting, it tries DuckDuckGo, which often answers automated requests with a challenge, and then English Wikipedia. The answer says which one was used. For dependable results, add a key or your own SearXNG.

[The plugin's README](../../plugins/web-search/README.md) lists the providers and settings. [Network access](../plugins/wasm-abi-guide.md#network-access) explains what the Engine allows.
