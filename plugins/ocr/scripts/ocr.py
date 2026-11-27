"""OCR plugin: reads text from an image with the Tesseract program, offline.

The Engine writes the call's arguments as JSON to a file and runs:
    python ocr.py <args.json>
`_fn` in that JSON names the function. The answer is one JSON object on standard output.
Only the Python standard library is used, so nothing has to be installed for the script itself.
"""

import hashlib
import json
import os
import re
import shutil
import subprocess
import sys
import tempfile
import urllib.request
from pathlib import Path

# Language data: tessdata_fast, tag 4.1.0 (the LSTM models Tesseract 4 and 5 read).
# Each file is pinned by size and SHA-256, measured on 2026-11-27 from the URL below.
TESSDATA_REPO = "tesseract-ocr/tessdata_fast"
TESSDATA_TAG = "4.1.0"
TESSDATA_URL = "https://raw.githubusercontent.com/" + TESSDATA_REPO + "/" + TESSDATA_TAG + "/{code}.traineddata"
LANGUAGES = {
    # code: (name, size in bytes, sha256)
    "ara": ("Arabic", 1432056, "e3206d3dc87fd50c24a0fb9f01838615911d25168f4e64415244b67d2bb3e729"),
    "ben": ("Bengali", 855841, "31163084c279aaebd376216f0c3d5c17ad4b5fee8db49dae79c20000b5de5964"),
    "chi_sim": ("Chinese (simplified)", 2469156, "a5fcb6f0db1e1d6d8522f39db4e848f05984669172e584e8d76b6b3141e1f730"),
    "chi_tra": ("Chinese (traditional)", 2366642, "529c5b5797d64b126065cd55f2bb4c7fd7b15790798091b1ff259941a829330b"),
    "deu": ("German", 1525436, "19d219bbb6672c869d20a9636c6816a81eb9a71796cb93ebe0cb1530e2cdb22d"),
    "eng": ("English", 4113088, "7d4322bd2a7749724879683fc3912cb542f19906c83bcc1a52132556427170b2"),
    "fra": ("French", 1130365, "ced037562e8c80c13122dece28dd477d399af80911a28791a66a63ac1e3445ca"),
    "guj": ("Gujarati", 1418394, "fa69658614b4946a9afae8853d67e0689838803dfa3d12c2e35ec53ee6f8df34"),
    "hin": ("Hindi", 1122751, "4c73ffc59d497c186b19d1e90f5d721d678ea6b2e277b719bee4e2af12271825"),
    "ita": ("Italian", 2701314, "b8f89e1e785118dac4d51ae042c029a64edb5c3ee42ef73027a6d412748d8827"),
    "jpn": ("Japanese", 2471260, "1f5de9236d2e85f5fdf4b3c500f2d4926f8d9449f28f5394472d9e8d83b91b4d"),
    "kan": ("Kannada", 3608331, "bd31e6b6ae93271e3bcf5383d306d8eefbb91542937cd6d735a5930c970e61d8"),
    "kor": ("Korean", 1677415, "6b85e11d9bbf07863b97b3523b1b112844c43e713df8b66418a081fd1060b3b2"),
    "mal": ("Malayalam", 5275996, "bd05cbf1b197e7810d2903419aedb06f9ef77bfedf50b358673c1d18d707cdb4"),
    "mar": ("Marathi", 2118233, "0ba3f2d116972e72fe9e176bc84c38e81dfb6670f4ed1f7f6c8e16a27da7cb61"),
    "nep": ("Nepali", 1002911, "280ba9450b4f21afbf5985e0de87857b75a972577c50ef0603c141ddde4f1cb8"),
    "pan": ("Punjabi", 497721, "1ec0907fc3534065ea9ae190c6bb7ec9e5c74fd9d2fa996aaec7407f11ad8131"),
    "por": ("Portuguese", 1982756, "c4932b937207a9514b7514d518b931a99938c02a28a5a5a553f8599ed58b7deb"),
    "rus": ("Russian", 3861738, "e16e5e036cce1d9ec2b00063cf8b54472625b9e14d893a169e2b0dedeb4df225"),
    "spa": ("Spanish", 2294433, "6f2e04d02774a18f01bed44b1111f2cd7f3ba7ac9dc4373cd3f898a40ea6b464"),
    "tam": ("Tamil", 3237963, "d02fbec24be4b07e32e80d0ccfc3b6b67a3c5d61c9d0a7c8532677990912c6ec"),
    "tel": ("Telugu", 2769654, "d10691fddd5b67802e1c12800ebb321d3b8bcd8d24a2ac3ff206f93188c04ab5"),
    "urd": ("Urdu", 1398718, "62e8250ce2a994106e313a82e26a516a39e2cf159d0ce3c5b5008387fd0d555f"),
}

IMAGE_TYPES = {".png", ".jpg", ".jpeg", ".tif", ".tiff", ".bmp", ".gif", ".webp", ".pnm", ".pbm", ".pgm", ".ppm"}
MAX_IMAGE_BYTES = 50 * 1024 * 1024
OCR_TIMEOUT_SECONDS = 25
DOWNLOAD_TIMEOUT_SECONDS = 20

DATA_DIR = Path(__file__).resolve().parent.parent / "data" / "tessdata"


class OcrError(Exception):
    pass


def answer(obj, code=0):
    # The Engine reads UTF-8; Windows consoles default to another code page
    try:
        sys.stdout.reconfigure(encoding="utf-8")
    except (AttributeError, ValueError):
        pass
    sys.stdout.write(json.dumps(obj, ensure_ascii=False, indent=2))
    sys.stdout.write("\n")
    sys.exit(code)


def find_tesseract():
    """TESSERACT_PATH, then PATH, then the usual Windows install folders."""
    candidates = []
    setting = os.environ.get("TESSERACT_PATH", "").strip().strip('"')
    if setting:
        p = Path(setting)
        candidates.append(p / ("tesseract.exe" if os.name == "nt" else "tesseract") if p.is_dir() else p)
    found = shutil.which("tesseract")
    if found:
        candidates.append(Path(found))
    if os.name == "nt":
        for base in (os.environ.get("ProgramFiles"), os.environ.get("ProgramFiles(x86)"),
                     os.path.join(os.environ.get("LOCALAPPDATA", ""), "Programs")):
            if base:
                candidates.append(Path(base) / "Tesseract-OCR" / "tesseract.exe")
    for c in candidates:
        if c.is_file():
            return c
    raise OcrError(
        "Tesseract is not installed or not found. Install Tesseract 5: on Debian or Ubuntu "
        "`sudo apt install tesseract-ocr`, on Fedora `sudo dnf install tesseract`, on Windows the installer from "
        "https://github.com/UB-Mannheim/tesseract/wiki. If it is installed in another folder, set the plugin "
        "setting TESSERACT_PATH to tesseract's file or folder."
    )


def tesseract_version(exe):
    try:
        out = subprocess.run([str(exe), "--version"], capture_output=True, text=True, timeout=10)
    except (OSError, subprocess.TimeoutExpired) as e:
        raise OcrError(f"Tesseract at {exe} could not be started: {e}")
    text = (out.stdout or "") + (out.stderr or "")
    m = re.search(r"tesseract v?(\d+)\.(\d+)\.?(\d*)", text)
    if not m:
        raise OcrError(f"Tesseract at {exe} did not report a version: {text.strip()[:200]}")
    version = ".".join(x for x in m.groups() if x)
    if int(m.group(1)) < 4:
        raise OcrError(f"Tesseract {version} is too old; version 4 or 5 is needed for these language files.")
    return version


def parse_languages(value):
    if value is None or value == "":
        return ["eng"]
    if isinstance(value, list):
        parts = [str(v) for v in value]
    else:
        parts = re.split(r"[+,\s]+", str(value))
    codes = [p.strip() for p in parts if p.strip()]
    for c in codes:
        if c not in LANGUAGES:
            raise OcrError(f"Unknown language code '{c}'. Supported: {', '.join(sorted(LANGUAGES))}.")
    return codes


def is_installed(code):
    f = DATA_DIR / f"{code}.traineddata"
    return f.is_file() and f.stat().st_size == LANGUAGES[code][1]


def download(code):
    name, size, sha = LANGUAGES[code]
    if is_installed(code):
        return {"code": code, "name": name, "status": "already downloaded", "bytes": size}
    DATA_DIR.mkdir(parents=True, exist_ok=True)
    url = TESSDATA_URL.format(code=code)
    digest = hashlib.sha256()
    got = 0
    fd, tmp = tempfile.mkstemp(prefix=f"{code}.", suffix=".part", dir=DATA_DIR)
    try:
        req = urllib.request.Request(url, headers={"User-Agent": "cluaiz-ocr-plugin"})
        with urllib.request.urlopen(req, timeout=DOWNLOAD_TIMEOUT_SECONDS) as resp, os.fdopen(fd, "wb") as out:
            while True:
                chunk = resp.read(1 << 16)
                if not chunk:
                    break
                got += len(chunk)
                if got > size:
                    raise OcrError(f"{url} sent more than the expected {size} bytes; the file was not kept.")
                digest.update(chunk)
                out.write(chunk)
        if got != size or digest.hexdigest() != sha:
            raise OcrError(f"{url} did not match its pinned size and SHA-256 (got {got} bytes, "
                           f"sha256 {digest.hexdigest()}); the file was not kept.")
        os.replace(tmp, DATA_DIR / f"{code}.traineddata")
    except OSError as e:
        raise OcrError(f"Download of {code} from {url} failed: {e}. Nothing was kept; try again.")
    finally:
        if os.path.exists(tmp):
            os.remove(tmp)
    return {"code": code, "name": name, "status": "downloaded", "bytes": got, "sha256": sha}


def fn_languages(args):
    action = (args.get("action") or "list").strip().lower()
    if action == "list":
        return {
            "source": f"https://github.com/{TESSDATA_REPO}/tree/{TESSDATA_TAG}",
            "folder": str(DATA_DIR),
            "languages": [
                {"code": c, "name": v[0], "megabytes": round(v[1] / 1e6, 1), "downloaded": is_installed(c)}
                for c, v in sorted(LANGUAGES.items())
            ],
        }
    if action == "download":
        codes = parse_languages(args.get("languages"))
        return {"results": [download(c) for c in codes], "folder": str(DATA_DIR)}
    if action == "remove":
        codes = parse_languages(args.get("languages"))
        removed = []
        for c in codes:
            f = DATA_DIR / f"{c}.traineddata"
            if f.is_file():
                f.unlink()
                removed.append(c)
        return {"removed": removed}
    raise OcrError("action must be list, download or remove.")


def fn_read(args):
    raw = args.get("path") or args.get("file") or args.get("image")
    if not raw:
        raise OcrError("path is required: the image file to read.")
    path = Path(str(raw).strip().strip('"')).expanduser()
    if not path.is_absolute():
        path = Path.cwd() / path
    if not path.is_file():
        raise OcrError(f"No file at {path}.")
    ext = path.suffix.lower()
    if ext == ".pdf":
        raise OcrError("PDF files are not read by this plugin yet. Save the page as a PNG or JPEG image and read that.")
    if ext not in IMAGE_TYPES:
        raise OcrError(f"Unsupported file type '{ext or 'none'}'. Supported: {', '.join(sorted(IMAGE_TYPES))}.")
    if path.stat().st_size > MAX_IMAGE_BYTES:
        raise OcrError(f"The image is larger than {MAX_IMAGE_BYTES // (1024 * 1024)} MB.")

    codes = parse_languages(args.get("languages"))
    # Tesseract is checked first, so nothing is downloaded on a machine that cannot use it
    exe = find_tesseract()
    version = tesseract_version(exe)
    # Only the languages this call names, and only those not already here
    downloaded = [download(c) for c in codes if not is_installed(c)]
    cmd = [str(exe), str(path), "stdout", "--tessdata-dir", str(DATA_DIR), "-l", "+".join(codes)]
    psm = args.get("psm")
    if psm is not None and str(psm).strip() != "":
        try:
            psm = int(psm)
        except ValueError:
            raise OcrError("psm must be a whole number from 3 to 13.")
        if psm < 3 or psm > 13:
            raise OcrError("psm must be from 3 to 13 (0-2 need extra data this plugin does not download).")
        cmd += ["--psm", str(psm)]
    try:
        run = subprocess.run(cmd, capture_output=True, timeout=OCR_TIMEOUT_SECONDS)
    except subprocess.TimeoutExpired:
        raise OcrError(f"Tesseract took longer than {OCR_TIMEOUT_SECONDS} s on this image and was stopped. "
                       "Try a smaller or cropped image.")
    stderr = run.stderr.decode("utf-8", "replace").strip()
    if run.returncode != 0:
        raise OcrError(f"Tesseract failed (exit {run.returncode}): {stderr[:500]}")
    text = run.stdout.decode("utf-8", "replace").replace("\f", "").strip()
    result = {
        "text": text,
        "characters": len(text),
        "languages": "+".join(codes),
        "file": str(path),
        "tesseract": version,
    }
    if downloaded:
        result["downloaded"] = downloaded
    if not text:
        result["note"] = "Tesseract found no text in this image."
    return result


def main():
    if len(sys.argv) < 2:
        answer({"status": "error", "error": "usage: ocr.py <args.json>"}, 2)
    try:
        args = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8") or "{}")
    except (OSError, ValueError) as e:
        answer({"status": "error", "error": f"Could not read the arguments: {e}"}, 2)
    fn = args.get("_fn") or "ocr_read"
    try:
        if fn == "ocr_read":
            result = fn_read(args)
        elif fn == "ocr_languages":
            result = fn_languages(args)
        else:
            raise OcrError(f"Unknown function '{fn}'.")
    except OcrError as e:
        answer({"status": "error", "error": str(e)}, 1)
    answer({"status": "ok", **result})


if __name__ == "__main__":
    main()
