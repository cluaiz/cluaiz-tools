---
name: rank
description: "Ranks documents passed to it by a keyword query. It cannot read files or the web."
---

# Rank

Call `rank_documents` when you already hold several texts and want the ones that best match a query. It scores words found in the title and content, and returns snippets.

It cannot read files or the web, so the text must be in the call.

```json
{"query": "memory limit", "documents": [{"id": "1", "title": "Plugins", "content": "Each call has a memory limit."}], "limit": 3}
```
