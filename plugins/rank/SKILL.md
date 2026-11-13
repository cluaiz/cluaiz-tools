---
name: rank
description: "Ranks texts passed to it by how well they match a keyword query. It cannot read files or the web. Use when several texts are already at hand and the best matches are needed."
---

# Rank

Call `rank_documents` with `query`, `documents` (each with `id`, `title` and `content`) and `limit`. It scores the query's words found in each title and content, and returns the best matches with snippets.

It cannot read files or the web, so the texts must be in the call. It matches words, not meaning: a text that says the same thing in other words scores low.
