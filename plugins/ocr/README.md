<!-- name: plugins/ocr
description: The OCR plugin reads the text in an image file with Tesseract on the user's computer and downloads, by itself, only the pinned language file a read needs. -->

# OCR

Reads the text in an image file, such as a photo or scan of a page, with the Tesseract program on this
computer. Nothing is sent anywhere. The only network use is the first read in a language: that one
language file (0.5 to 5.3 MB, pinned) is downloaded, and no other.

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
| `SKILL.md` | instructions for the model: read, and pass `timeout: 120` on a language's first use |
| `scripts/ocr.py` | the whole plugin, Python standard library only |
| `assets/icon.svg` | icon |
| `tests/all_languages.py` | renders one line per language and reads it through a running Engine (not in the release zip) |
| `.gitignore` | keeps `data/` (downloaded language files) out of git |

## Functions

| Function | Arguments | Answer |
|---|---|---|
| `ocr_read` | `path` (required), `languages` such as `eng+hin` (default `eng`), `psm` 3 to 13, `timeout` | `text`, `characters`, `languages`, `file`, `tesseract`; `downloaded` when it fetched a language; `note` when no text was found |
| `ocr_languages` | `action`: `list` (default), `download`, `remove`; `languages`; `timeout` | the list with sizes and what is downloaded, or what was downloaded or removed |

Supported codes (23): ara, ben, chi_sim, chi_tra, deu, eng, fra, guj, hin, ita, jpn, kan, kor, mal, mar,
nep, pan, por, rus, spa, tam, tel, urd. Sizes are 0.5 MB (pan) to 5.3 MB (mal).

## Decisions

| Decision | Why | Alternative not taken |
|---|---|---|
| Script plugin calling the `tesseract` program | the Engine runs WebAssembly (no file access) and scripts; native plugins cannot run yet (`docs/plugins/build-types.md`, "Native binary: Planned") | WebAssembly build of Tesseract (no file access, large); native library (not runnable) |
| Python standard library only | no virtual environment or `pip` step; a `requirements.txt` would make the Engine treat it as a Python tool and build a venv | `pytesseract` (only wraps the same program) |
| Language data from `tessdata_fast` tag `4.1.0`, each file pinned by size and SHA-256 | fixed bytes forever; a file that does not match is deleted, never used. `fast` files are 0.5 to 5.3 MB, which suits small machines | `tessdata` / `tessdata_best` (larger, slower); following `main` (unpinned) |
| `ocr_read` downloads a missing language itself, only the codes in that call (owner's order, 2026-11-27: install only what is needed, automatically, nothing extra) | the user does not have to know about language files; nothing is fetched for a language nobody read. Tesseract is checked first, so a machine without it downloads nothing | an error asking for `ocr_languages download` first (version 1.0.0); every language at install |
| Data kept in the plugin's own folder, `data/tessdata/`, passed with `--tessdata-dir` | the system Tesseract's own data is never touched or needed. The Engine has no per-tool data folder outside the replaced plugin folder (`agents/src/installer/tools_client.rs` knows only the folder and its `.cache`), so this is the only place the plugin owns | the system `tessdata` folder (needs admin, varies by OS); a hand-made path under the Engine home (never removed on uninstall) |
| Tesseract stays a user install | the plugin never installs system programs | download a Tesseract build |
| `psm` limited to 3 to 13 | modes 0 to 2 need the `osd` data, which is not offered | offer `osd` (10.6 MB) |
| PDF refused with a message | reading PDF pages needs a renderer the plugin does not ship | call an external PDF tool |

## Tested

### All 23 languages, through the Engine (2026-11-27)

GCP `cluaiz-l4b`, Ubuntu 24.04, Tesseract 5.3.4 (apt), version 1.0.1 installed with
`cluaiz plugin install ocr --yes` into an isolated `CLUAIZ_HOME`, server on port 8016, every call made with
CEP `tools.call` (`kind: plugin`, `name: ocr`, `fn: ocr_read`, `psm: 7`, `timeout: 120`). The plugin folder
had no `data/` before the run, so the first read of each language downloaded it.

Each image is one line, 48 px, drawn by Pillow with libraqm (real shaping for Indic and Arabic scripts) in
the script's own Noto Sans font (Noto Naskh Arabic for Arabic and Urdu; Noto Sans CJK for Chinese, Japanese
and Korean). CER = edit distance / length of the expected line, all whitespace removed from both.

| Code | Download (bytes) | First read, with download (ms) | Read (ms) | Expected | OCR output | CER |
|---|---|---|---|---|---|---|
| eng | 4,113,088 | 443 | 257 | The quick brown fox jumps over the lazy dog | same | 0.0 |
| deu | 1,525,436 | 490 | 254 | Der schnelle braune Fuchs springt über den Hund | same | 0.0 |
| fra | 1,130,365 | 379 | 252 | Le renard brun saute par-dessus le chien paresseux | same | 0.0 |
| spa | 2,294,433 | 384 | 255 | El rápido zorro marrón salta sobre el perro | same | 0.0 |
| por | 1,982,756 | 489 | 256 | A rápida raposa marrom pula sobre o cão preguiçoso | same | 0.0 |
| ita | 2,701,314 | 435 | 250 | La volpe marrone salta sopra il cane pigro | same | 0.0 |
| rus | 3,861,738 | 528 | 184 | Быстрая коричневая лиса прыгает через собаку | same | 0.0 |
| hin | 1,122,751 | 359 | 173 | नमस्ते दुनिया, यह एक परीक्षण है | same | 0.0 |
| mar | 2,118,233 | 423 | 169 | आज हवामान खूप छान आहे | same | 0.0 |
| nep | 1,002,911 | 686 | 164 | म नेपालमा बस्छु र किताब पढ्छु | same | 0.0 |
| ben | 855,841 | 575 | 163 | আমি বাংলায় কথা বলি এবং বই পড়ি | same | 0.0 |
| guj | 1,418,394 | 578 | 161 | હું ગુજરાતી ભાષા વાંચું છું | same | 0.0 |
| pan | 497,721 | 439 | 143 | ਮੈਂ ਪੰਜਾਬੀ ਵਿੱਚ ਕਿਤਾਬ ਪੜ੍ਹਦਾ ਹਾਂ | (empty) | 1.0 |
| tam | 3,237,963 | 559 | 175 | நான் தமிழ் புத்தகம் படிக்கிறேன் | same letters, plus an invisible U+200C after each word | 0.143 |
| tel | 2,769,654 | 591 | 184 | నేను తెలుగు పుస్తకం చదువుతున్నాను | same | 0.0 |
| kan | 3,608,331 | 548 | 168 | ನಾನು ಕನ್ನಡ ಪುಸ್ತಕ ಓದುತ್ತೇನೆ | same | 0.0 |
| mal | 5,275,996 | 728 | 184 | ഞാൻ മലയാളം പുസ്തകം വായിക്കുന്നു | ഞാന്‍ മലയാളം പുസ്തകം വായിക്കുന്നു (old chillu encoding, looks the same) | 0.107 |
| urd | 1,398,718 | 495 | 167 | میں اردو کتاب پڑھتا ہوں | same | 0.0 |
| ara | 1,432,056 | 506 | 164 | أنا أقرأ كتابا باللغة العربية | same | 0.0 |
| jpn | 2,471,260 | 419 | 273 | 私は毎日日本語の本を読みます | same | 0.0 |
| kor | 1,677,415 | 409 | 186 | 나는 매일 한국어 책을 읽습니다 | same | 0.0 |
| chi_sim | 2,469,156 | 494 | 282 | 我每天读中文书 | same | 0.0 |
| chi_tra | 2,366,642 | 478 | 293 | 我每天讀中文書 | same | 0.0 |

- 20 of 23 read exactly. Hindi, redone with correct shaping, reads exactly (the 3 wrong letters in the
  first test came from the unshaped test image).
- Tamil and Malayalam: the letters are right; the CER comes from encoding (U+200C after each Tamil word; the
  older two-character Malayalam chillu).
- Punjabi: empty for this line with both `tessdata_fast` and `tessdata_best` (11.9 MB, tried by hand), at
  psm 3, 6 and 7; psm 13 gave `ਮੋਪਜਬੀ ਵਿੱਚ ਕਿਤਾਬ ਪੜਹਦਰਾਂ`. Another line, `ਪੰਜਾਬ ਭਾਰਤ ਦਾ ਇੱਕ ਰਾਜ ਹੈ`, reads exactly
  with `tessdata_fast` at psm 7. So it depends on the line: the model, not the plugin.
- Only what a read names is downloaded: with `data/` removed, one `hin` read left only `hin.traineddata`
  (1,122,751 bytes).

Rerun: on a machine with Tesseract, `sudo apt install python3-pil libraqm0 fonts-noto-core fonts-noto-cjk`,
start `cluaiz serve` with an isolated `CLUAIZ_HOME` and port, install the plugin, then
`CLUAIZ_HOME=<home> CLUAIZ_PORT=<port> python3 tests/all_languages.py <out folder>` (writes `results.json`
and the 23 images).

### Errors and Windows

| Machine | Test | Result |
|---|---|---|
| Windows 11 laptop, Python 3.13.5, no Tesseract, Engine on port 8011 (1.0.0) | download `eng` with `ocr_languages` | matched, 19,994 ms (a slow line: close to the 30 s default, hence `timeout: 120` in `SKILL.md`) |
| same | read | error "Tesseract is not installed or not found ..." with install lines |
| same, script run directly | `.pdf` / unknown code `xyz` | PDF error / error listing the 23 codes |
| `cluaiz-l4b` (1.0.0) | read `english.png` (3 lines, Arial) | exact, 79 characters, 306 ms |

### Chat

`cluaiz-l4b`, Qwen3.5-4B Q4_K_M, `/v1/chat/completions` (1.0.0): the Engine suggested the plugin, the model
loaded it and called `ocr_read {"path":"english.png","languages":"eng","psm":11}`. The call then waited for
the user's approval (`files` and `programs` are sensitive permissions) and timed out after 120 s, because
nobody approved it in that non-interactive test.

Not proven: OCR on Windows (no Tesseract on the test laptop), real photos and scans, macOS, a chat where a
person approves the call.

## Failure and recovery

| Where | What happens |
|---|---|
| Tesseract missing | error with the install line for each system and the `TESSERACT_PATH` setting |
| Tesseract older than 4 | error: version 4 or 5 is needed |
| Language not downloaded | `ocr_read` downloads that language only, then reads; the answer lists it under `downloaded` |
| Download fails, is too long, or does not match the pin | the partial file is deleted; error says why; run the download again |
| Download slower than the Engine's 30 s limit | the model passes `timeout` (120 in `SKILL.md`); if the Engine still stops it, a `.part` file may stay in `data/tessdata/` but is never used (only a complete, matching `<code>.traineddata` counts) and the next read downloads again |
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
| Plugin update removes `data/` | the installer replaces the plugin folder, so a read downloads its language again after an update | an Engine data folder per tool that an update keeps |
| Punjabi misses some lines | the 23-language run read one Punjabi line as empty | a check of more Gurmukhi lines and fonts; maybe `tessdata_best` for `pan` |
| Chat with an approved call | the model reached `ocr_read`, but the approval was not answered in the test | an App or terminal chat where a person approves |
| OCR not run on Windows | the Windows path search and output encoding are tested only up to "Tesseract not found" | a Windows run with Tesseract installed |
| No PDF pages | scanned PDFs are common | a pinned PDF-to-image step |
| Tesseract is not bundled | users must install it themselves | binary plugin support in the Engine, or a pinned portable build |
