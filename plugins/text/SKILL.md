---
name: text
description: "Exact text work: hashes, Base64, URL and hex encoding, JSON formatting and checking, regular expressions, case changes, diffs and counts. Use when a text result must be exact, because working these out yourself gives wrong results."
---

# Text

Call `text_transform` with `action` and `input`:

| `action` | Also needs | Answer |
|---|---|---|
| `md5`, `sha1`, `sha256`, `sha512` | | `result`: the hash in hex |
| `base64_encode`, `base64_decode`, `url_encode`, `url_decode`, `hex_encode`, `hex_decode` | | `result` |
| `json_format`, `json_minify` | | `result`, with key order and numbers kept |
| `json_validate` | | `valid`, and `error` with the line and column when it is not valid |
| `case` | `case`: upper, lower, title, sentence, snake, kebab, camel, pascal or constant | `result` |
| `stats` | | `stats`: characters, words, lines, bytes |
| `diff` | `target` | `diff_summary`: counts and a patch |
| `extract` | `pattern`, a regular expression | `matches`: up to 1000 |
| `replace` | `pattern`, and `replacement`, where `$1` or `${name}` is a group | `result`, and `replaced`: how many matches |

- Hashes are of the UTF-8 bytes of `input`. MD5 and SHA-1 are not safe for passwords or signatures; say so if the user wants them for that.
- `url_decode` reads `+` as a space. A decode that is not UTF-8 text is an error that shows the bytes in hex.
- Regular expressions use Rust syntax: no look-ahead or back-references.
