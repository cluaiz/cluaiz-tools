# Writing text to be spoken

## Choosing a voice

Call `speech` with `{"action": "voices"}` (add `"language": "hi"` to filter). Use an id it returns. Common Kokoro ids:

| Language | Voice | Kind |
|---|---|---|
| English (US) | `af_heart` | Female |
| English (US) | `am_adam` | Male |
| English (UK) | `bf_emma` | Female |
| Hindi | `hf_alpha` | Female |
| Hindi | `hm_omega` | Male |

The list depends on the installed model; trust `voices` over this table.

## Preparing the text

- Write numbers, dates and units the way they should be said when it matters: "3.5 GB" may be read as "three point five gigabytes" or not, depending on the rules for the language.
- Spell out abbreviations that should be read letter by letter only if the reading sounds wrong.
- Long text works; very long paragraphs read better split into sentences.
- Remove Markdown symbols (`#`, `*`, `|`, code blocks) before speaking a report; they are not words.

## Example

```json
{
  "action": "speak",
  "text": "The build finished in forty-two seconds.",
  "output": "build-done.wav",
  "language": "en",
  "voice": "af_heart"
}
```

Give the user the path of the file the answer names.
