---
name: text
description: "Hashes, Base64, regular expressions, line diffs and counts, computed exactly."
---

# Text

Call `text_transform` for these, because working them out yourself gives wrong results:

| `action` | Also needs | Answer |
|---|---|---|
| `sha256` | | `result`: the hash in hex |
| `base64_encode`, `base64_decode` | | `result` |
| `stats` | | `stats`: characters, words, lines, bytes |
| `diff` | `target` | `diff_summary`: counts and a patch |
| `extract` | `pattern`, a regular expression | `matches`: up to 1000 |

```json
{"action": "extract", "input": "mail a@b.com or c@d.org", "pattern": "[a-z]+@[a-z.]+"}
```
