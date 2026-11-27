---
name: ocr
description: "Reads the text in an image file, such as a photo or scan of a page, with Tesseract on this computer. Use when the user gives an image and wants its text, instead of guessing what the image says."
---

# OCR

1. Call `ocr_read` with `path` and `languages` (codes joined with `+`, default `eng`).
2. If the answer says the language data is not downloaded yet, call `ocr_languages` with `action: "download"`,
   the same `languages` and `timeout: 120`, then call `ocr_read` again.

| Function | Answer |
|---|---|
| `ocr_read` | `text`, `characters`, `languages`, `tesseract` (version); `note` when no text was found |
| `ocr_languages` `list` | every supported code, its size and whether it is downloaded |
| `ocr_languages` `download` / `remove` | what was downloaded or removed |

- Common codes: `eng` English, `hin` Hindi, `ben` Bengali, `mar` Marathi, `tam` Tamil, `urd` Urdu,
  `ara` Arabic, `chi_sim` Chinese (simplified), `jpn` Japanese, `deu` German, `fra` French, `spa` Spanish.
  Call `list` for all of them.
- Give the user the text as it came back. OCR can misread letters; say so if the text looks broken, and do
  not fill gaps yourself.
- A PDF is not read: ask for the page as an image. If the answer says Tesseract is not installed, give the
  user the install line from the error.
- `psm`: 6 for one block of text, 7 for one line, 11 for scattered text such as a screenshot.
