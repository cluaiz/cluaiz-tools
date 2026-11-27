---
name: ocr
description: "Reads the text in an image file, such as a photo or scan of a page, with Tesseract on this computer. Use when the user gives an image and wants its text, instead of guessing what the image says."
---

# OCR

Call `ocr_read` with `path` and `languages` (codes joined with `+`, default `eng`).

- A language used for the first time is downloaded by `ocr_read` itself: only that language, a pinned file of
  0.5 to 5.3 MB. The Engine stops a call after 30 s, so pass `timeout: 120` when `ocr_languages list` does not
  show the language as downloaded, or when you are not sure. The answer lists `downloaded` when it fetched one.
- Ask only for the languages that are in the image: each one is a download.

| Function | Answer |
|---|---|
| `ocr_read` | `text`, `characters`, `languages`, `tesseract` (version); `downloaded` when a language was fetched; `note` when no text was found |
| `ocr_languages` `list` | every supported code, its size and whether it is downloaded |
| `ocr_languages` `download` / `remove` | download ahead of time, or delete, the codes named |

- Codes: `eng` English, `hin` Hindi, `ben` Bengali, `mar` Marathi, `nep` Nepali, `guj` Gujarati, `pan` Punjabi,
  `tam` Tamil, `tel` Telugu, `kan` Kannada, `mal` Malayalam, `urd` Urdu, `ara` Arabic, `chi_sim` / `chi_tra`
  Chinese, `jpn` Japanese, `kor` Korean, `rus` Russian, `deu` German, `fra` French, `spa` Spanish, `por`
  Portuguese, `ita` Italian.
- Give the user the text as it came back. OCR can misread letters; say so if the text looks broken, and do
  not fill gaps yourself.
- A PDF is not read: ask for the page as an image. If the answer says Tesseract is not installed, give the
  user the install line from the error; never install it yourself.
- `psm`: 6 for one block of text, 7 for one line, 11 for scattered text such as a screenshot.
