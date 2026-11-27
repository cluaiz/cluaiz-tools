<!-- name: plugins/ocr
description: The OCR plugin reads the text in an image file with Tesseract on the user's computer and downloads only the pinned language data that is asked for. -->

# OCR

Reads the text in an image file, such as a photo or scan of a page, with the Tesseract program on this
computer. Nothing is sent anywhere: the only network use is the one-time download of the language data the
user asks for.

```text
ocr_read { "path": "english.png" }
-> { "status": "ok", "text": "Invoice number 48213\nTotal due: 1,250.75 USD\n\nCluaiz OCR test, 27 November 2026",
     "characters": 79, "languages": "eng", "tesseract": "5.3.4" }
```

## What the user needs

| Need | Why | How |
|---|---|---|
| Python 3 | the plugin is a script plugin (`build_type: "none"` with `execution`) | usually present; the Engine runs `{python}` |
| Tesseract 4 or 5 | it does the OCR | Debian/Ubuntu `sudo apt install tesseract-ocr`, Fedora `sudo dnf install tesseract`, Windows the UB Mannheim installer |

The plugin finds Tesseract in this order: the `TESSERACT_PATH` setting (file or folder), `PATH`, then
`Program Files\Tesseract-OCR` and `%LOCALAPPDATA%\Programs\Tesseract-OCR` on Windows.

## Files

| File | What it is |
|---|---|
| `package.json` | identity, the two functions, permissions, the `TESSERACT_PATH` setting, the script command |
| `SKILL.md` | instructions for the model: read, and download the language first when asked to |
| `scripts/ocr.py` | the whole plugin, Python standard library only |
| `assets/icon.svg` | icon |
| `.gitignore` | keeps `data/` (downloaded language files) out of git |

## Functions

| Function | Arguments | Answer |
|---|---|---|
| `ocr_read` | `path` (required), `languages` such as `eng+hin` (default `eng`), `psm` 3 to 13 | `text`, `characters`, `languages`, `file`, `tesseract`; `note` when no text was found |
| `ocr_languages` | `action`: `list` (default), `download`, `remove`; `languages`; `timeout` | the list with sizes and what is downloaded, or what was downloaded or removed |

Supported codes (23): ara, ben, chi_sim, chi_tra, deu, eng, fra, guj, hin, ita, jpn, kan, kor, mal, mar,
nep, pan, por, rus, spa, tam, tel, urd. Sizes are 0.5 MB (pan) to 5.3 MB (mal).

## Decisions

| Decision | Why | Alternative not taken |
|---|---|---|
| Script plugin calling the `tesseract` program | the Engine runs WebAssembly (no file access) and scripts; native plugins cannot run yet (`docs/plugins/build-types.md`, "Native binary: Planned") | WebAssembly build of Tesseract (no file access, large); native library (not runnable) |
| Python standard library only | no virtual environment or `pip` step; a `requirements.txt` would make the Engine treat it as a Python tool and build a venv | `pytesseract` (only wraps the same program) |
| Language data from `tessdata_fast` tag `4.1.0`, each file pinned by size and SHA-256 | fixed bytes forever; a file that does not match is deleted, never used. `fast` files are 0.5 to 5.3 MB, which suits small machines | `tessdata` / `tessdata_best` (larger, slower); following `main` (unpinned) |
| Download only on request (`ocr_languages download`), and `ocr_read` fails with a clear message when data is missing | the user's network and disk are used only for languages they need; no hidden download inside a read | download every language at install |
| Data kept in the plugin's own folder, `data/tessdata/`, passed with `--tessdata-dir` | the system Tesseract's own data is never touched or needed | the system `tessdata` folder (needs admin, varies by OS) |
| `psm` limited to 3 to 13 | modes 0 to 2 need the `osd` data, which is not offered | offer `osd` (10.6 MB) |
| PDF refused with a message | reading PDF pages needs a renderer the plugin does not ship | call an external PDF tool |

## Tested

2026-11-27, `scripts/ocr.py` run directly with a JSON argument file (the same way the Engine runs it).
Test images were made with Pillow: `english.png` (Arial 44 px, three lines) and `hindi.png` (Nirmala UI
56 px, two lines; Pillow without a shaping library draws conjuncts with a visible halant).

| Machine | Test | Result |
|---|---|---|
| Windows 11 laptop, Python 3.13.5, no Tesseract | read with `eng` not downloaded | error "Language data not downloaded yet: eng ..." |
| same | `.pdf` file | error "PDF files are not read by this plugin yet ..." |
| same | unknown code `xyz` | error listing the 23 codes |
| same | download `hin` | 1,122,751 bytes, SHA-256 matched |
| same | read `hindi.png` | error "Tesseract is not installed or not found ..." with install lines |
| GCP `cluaiz-l4b`, Ubuntu 24.04, Tesseract 5.3.4 (apt), Python 3 | download `eng+hin` | both matched; 0.40 s |
| same | read `english.png`, `eng` | exact text, 79 characters, 0.29 s |
| same | read `hindi.png`, `hin` | `नमसूते दुनया` / `यह एक परीकषण है`: three letters wrong (from the unshaped test image), 0.17 s |

Rerun: write `{"_fn":"ocr_read","path":"<image>","languages":"eng"}` to `a.json`, then
`python scripts/ocr.py a.json`.

Not proven: OCR on Windows (no Tesseract was installed on the test laptop), real photos and scans, macOS.

## Failure and recovery

| Where | What happens |
|---|---|
| Tesseract missing | error with the install line for each system and the `TESSERACT_PATH` setting |
| Tesseract older than 4 | error: version 4 or 5 is needed |
| Language not downloaded | error naming the codes and the exact `ocr_languages` call |
| Download fails, is too long, or does not match the pin | the partial file is deleted; error says why; run the download again |
| Download slower than the Engine's 30 s limit | the model passes `timeout` (120 suggested in `SKILL.md`); an interrupted download leaves no file |
| File missing, wrong type, PDF, over 50 MB | error, Tesseract is not started |
| Tesseract runs over 25 s or exits non-zero | error with its message |
| No text in the image | `status: ok`, empty `text`, and a `note` |

## Sources

- Tesseract command line (`tesseract <image> stdout -l <langs> --tessdata-dir <dir> --psm <n>`):
  https://tesseract-ocr.github.io/tessdoc/Command-Line-Usage.html
- Language data: https://github.com/tesseract-ocr/tessdata_fast/tree/4.1.0
- Script plugin contract: `docs/plugins/build-types.md` in this repository.

## Pending

| Item | Why it matters | Needs |
|---|---|---|
| Plugin update removes `data/` | the installer replaces the plugin folder, so languages must be downloaded again | an Engine data folder per tool |
| OCR not run on Windows | the Windows path search and output encoding are tested only up to "Tesseract not found" | a Windows run with Tesseract installed |
| No PDF pages | scanned PDFs are common | a pinned PDF-to-image step |
| Tesseract is not bundled | users must install it themselves | binary plugin support in the Engine, or a pinned portable build |
