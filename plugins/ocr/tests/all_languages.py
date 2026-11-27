"""Reads one rendered line in every supported language through a running Engine.

Needs: a running `cluaiz serve` with the ocr plugin installed, Pillow with libraqm (text shaping) and the Noto
fonts (Debian/Ubuntu: python3-pil libraqm0 fonts-noto-core fonts-noto-cjk).

    CLUAIZ_HOME=<home> CLUAIZ_PORT=<port> python3 all_languages.py <output folder>

For each language: render the line, call ocr_read once (the first use downloads that language) and once more
(OCR only), and compare the text with the line. Character error rate = edit distance / length of the line,
with all whitespace removed from both (Tesseract puts spaces between CJK characters). Writes results.json.
"""

import json
import os
import sys
import time
import urllib.request
from pathlib import Path

from PIL import Image, ImageDraw, ImageFont, features

F = "/usr/share/fonts/truetype/noto/"
CJK = "/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc"
LINES = {
    "eng": ("The quick brown fox jumps over the lazy dog", F + "NotoSans-Regular.ttf", 0),
    "deu": ("Der schnelle braune Fuchs springt über den Hund", F + "NotoSans-Regular.ttf", 0),
    "fra": ("Le renard brun saute par-dessus le chien paresseux", F + "NotoSans-Regular.ttf", 0),
    "spa": ("El rápido zorro marrón salta sobre el perro", F + "NotoSans-Regular.ttf", 0),
    "por": ("A rápida raposa marrom pula sobre o cão preguiçoso", F + "NotoSans-Regular.ttf", 0),
    "ita": ("La volpe marrone salta sopra il cane pigro", F + "NotoSans-Regular.ttf", 0),
    "rus": ("Быстрая коричневая лиса прыгает через собаку", F + "NotoSans-Regular.ttf", 0),
    "hin": ("नमस्ते दुनिया, यह एक परीक्षण है", F + "NotoSansDevanagari-Regular.ttf", 0),
    "mar": ("आज हवामान खूप छान आहे", F + "NotoSansDevanagari-Regular.ttf", 0),
    "nep": ("म नेपालमा बस्छु र किताब पढ्छु", F + "NotoSansDevanagari-Regular.ttf", 0),
    "ben": ("আমি বাংলায় কথা বলি এবং বই পড়ি", F + "NotoSansBengali-Regular.ttf", 0),
    "guj": ("હું ગુજરાતી ભાષા વાંચું છું", F + "NotoSansGujarati-Regular.ttf", 0),
    "pan": ("ਮੈਂ ਪੰਜਾਬੀ ਵਿੱਚ ਕਿਤਾਬ ਪੜ੍ਹਦਾ ਹਾਂ", F + "NotoSansGurmukhi-Regular.ttf", 0),
    "tam": ("நான் தமிழ் புத்தகம் படிக்கிறேன்", F + "NotoSansTamil-Regular.ttf", 0),
    "tel": ("నేను తెలుగు పుస్తకం చదువుతున్నాను", F + "NotoSansTelugu-Regular.ttf", 0),
    "kan": ("ನಾನು ಕನ್ನಡ ಪುಸ್ತಕ ಓದುತ್ತೇನೆ", F + "NotoSansKannada-Regular.ttf", 0),
    "mal": ("ഞാൻ മലയാളം പുസ്തകം വായിക്കുന്നു", F + "NotoSansMalayalam-Regular.ttf", 0),
    "urd": ("میں اردو کتاب پڑھتا ہوں", F + "NotoNaskhArabic-Regular.ttf", 0),
    "ara": ("أنا أقرأ كتابا باللغة العربية", F + "NotoNaskhArabic-Regular.ttf", 0),
    "jpn": ("私は毎日日本語の本を読みます", CJK, 0),
    "kor": ("나는 매일 한국어 책을 읽습니다", CJK, 1),
    "chi_sim": ("我每天读中文书", CJK, 2),
    "chi_tra": ("我每天讀中文書", CJK, 3),
}


def cep(home, port, method, params, timeout):
    token = (Path(home) / "session.token").read_text().strip()
    body = json.dumps({"jsonrpc": "2.0", "id": 1, "method": method, "params": params}).encode()
    req = urllib.request.Request(f"http://127.0.0.1:{port}/v1/cep", body,
                                 {"Content-Type": "application/json", "Authorization": "Bearer " + token})
    return json.load(urllib.request.urlopen(req, timeout=timeout))


def read(home, port, path, code):
    t = time.time()
    res = cep(home, port, "tools.call", {"kind": "plugin", "name": "ocr", "fn": "ocr_read",
                                         "input": {"path": str(path), "languages": code, "psm": 7, "timeout": 120}}, 180)
    wall = round((time.time() - t) * 1000)
    out = json.loads(res["result"]["stdout"])
    return out, wall


def distance(a, b):
    prev = list(range(len(b) + 1))
    for i, ca in enumerate(a, 1):
        cur = [i]
        for j, cb in enumerate(b, 1):
            cur.append(min(prev[j] + 1, cur[j - 1] + 1, prev[j - 1] + (ca != cb)))
        prev = cur
    return prev[-1]


def main():
    assert features.check("raqm"), "Pillow has no libraqm: Indic and Arabic text would not be shaped"
    out_dir = Path(sys.argv[1])
    out_dir.mkdir(parents=True, exist_ok=True)
    home, port = os.environ["CLUAIZ_HOME"], os.environ.get("CLUAIZ_PORT", "8016")
    rows = []
    for code, (line, font_file, index) in LINES.items():
        font = ImageFont.truetype(font_file, 48, index=index, layout_engine=ImageFont.Layout.RAQM)
        box = ImageDraw.Draw(Image.new("L", (1, 1))).textbbox((0, 0), line, font=font)
        img = Image.new("L", (box[2] - box[0] + 80, box[3] - box[1] + 80), 255)
        ImageDraw.Draw(img).text((40 - box[0], 40 - box[1]), line, font=font, fill=0)
        path = out_dir / f"{code}.png"
        img.save(path)
        first, first_wall = read(home, port, path, code)
        second, second_wall = read(home, port, path, code)
        text = second.get("text", "")
        want = "".join(line.split())
        got = "".join(text.split())
        dl = (first.get("downloaded") or [{}])[0]
        rows.append({
            "code": code, "status": second.get("status"), "error": second.get("error"),
            "download_bytes": dl.get("bytes"), "first_read_ms": first_wall, "read_ms": second_wall,
            "expected": line, "ocr": text, "cer": round(distance(want, got) / len(want), 3),
        })
        print(json.dumps(rows[-1], ensure_ascii=False), flush=True)
    (out_dir / "results.json").write_text(json.dumps(rows, ensure_ascii=False, indent=2), encoding="utf-8")


if __name__ == "__main__":
    main()
