#!/usr/bin/env python3
"""Speech to text and text to speech through the local Cluaiz Engine. The Engine calls it with a JSON file of arguments."""

import base64
import json
import os
import sys
import urllib.error
import urllib.request
from pathlib import Path

PULL = {
    "transcribe": "cluaiz pull onnx-community/moonshine-tiny-ONNX",
    "speak": "cluaiz pull onnx-community/Kokoro-82M-v1.0-ONNX",
}


def finish(answer, code=0):
    print(json.dumps(answer, ensure_ascii=False))
    sys.exit(code)


def fail(message):
    finish({"status": "error", "message": message}, 1)


def engine():
    """The Engine's address and token, from the Cluaiz home it runs with."""
    home = Path(os.environ.get("CLUAIZ_HOME") or Path.home() / ".cluaiz")
    port = os.environ.get("CLUAIZ_PORT")
    if not port:
        try:
            port = json.loads((home / "config" / "permission.json").read_text(encoding="utf-8")).get("api_port")
        except (OSError, ValueError):
            port = None
    try:
        token = (home / "session.token").read_text(encoding="utf-8").strip()
    except OSError:
        fail(f"The Cluaiz Engine is not running (no session token in {home}). Tell the user to start it with: cluaiz serve. Then stop: do not install packages or write your own speech script.")
    return f"http://127.0.0.1:{port or 8000}", token


def post(path, body):
    address, token = engine()
    request = urllib.request.Request(address + path, json.dumps(body).encode("utf-8"),
                                     {"Authorization": "Bearer " + token, "Content-Type": "application/json"})
    try:
        with urllib.request.urlopen(request, timeout=600) as response:
            return json.loads(response.read().decode("utf-8"))
    except urllib.error.HTTPError as error:
        text = error.read().decode("utf-8", "replace")
        try:
            detail = json.loads(text)
            detail = detail.get("error", detail)
            text = detail.get("message", text) if isinstance(detail, dict) else text
        except ValueError:
            pass
        return {"error": text, "http_status": error.code}
    except urllib.error.URLError as error:
        fail(f"The Cluaiz Engine did not answer at {address}: {error.reason}. Tell the user to start it with: cluaiz serve. Then stop: do not install packages or write your own speech script.")


def explain(answer, action):
    message = str(answer.get("error"))
    if "model_not_found" in message or "No installed model" in message or answer.get("http_status") == 404:
        fail(f"No speech model is installed for this. Install one with: {PULL[action]}")
    fail(message)


def main():
    if len(sys.argv) < 2:
        fail("No arguments were given.")
    arguments = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
    action = arguments.get("action")
    language = arguments.get("language") or None
    if action == "transcribe":
        source = arguments.get("file") or ""
        path = Path(source).expanduser()
        if not path.is_file():
            fail(f"No audio file at {source or '(nothing given)'}.")
        body = {"file": str(path.resolve())}
        if language:
            body["language"] = language
        answer = post("/v1/audio/transcriptions", body)
        if "error" in answer:
            explain(answer, action)
        output = answer.get("output") or answer.get("result", {}).get("output") or answer
        finish({"status": "success", "text": (output.get("text") or "").strip(), "model": answer.get("model")})
    if action == "speak":
        text = (arguments.get("text") or "").strip()
        if not text:
            fail("No text to speak.")
        target = Path(arguments.get("output") or "speech.wav").expanduser()
        if target.suffix.lower() != ".wav":
            target = target.with_suffix(".wav")
        body = {"input": text}
        for key in ("voice", "language"):
            if arguments.get(key):
                body[key] = arguments[key]
        answer = post("/v1/audio/speech", body)
        if "error" in answer:
            explain(answer, action)
        output = answer.get("output") or answer.get("result", {}).get("output") or {}
        audio = output.get("audio_data") or ""
        if "," in audio[:100] and audio.startswith("data:"):
            audio = audio.split(",", 1)[1]
        if not audio:
            fail("The speech model returned no audio.")
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_bytes(base64.b64decode(audio))
        finish({"status": "success", "file": str(target.resolve()), "bytes": target.stat().st_size, "model": answer.get("model")})
    if action == "voices":
        # The voices of the installed speech model, read without loading it (CEP audio.voices)
        answer = post("/v1/cep", {"jsonrpc": "2.0", "id": 1, "method": "audio.voices", "params": {}})
        if "error" in answer:
            error = answer["error"]
            message = error.get("message", str(error)) if isinstance(error, dict) else str(error)
            if "model" in message.lower() and ("no " in message.lower() or "not" in message.lower()):
                fail(f"No speech model is installed for this. Install one with: {PULL['speak']}")
            fail(message)
        result = answer.get("result") or {}
        voices = [
            {"id": v.get("id"), "language": v.get("language"), "default": v.get("default", False)}
            for v in (result.get("data") or [])[:80]
        ]
        wanted = (language or "").lower()
        if wanted:
            voices = [v for v in voices if not v["language"] or str(v["language"]).lower().startswith(wanted)] or voices
        finish({"status": "success", "languages": (result.get("capabilities") or {}).get("languages"), "voices": voices})
    fail("action must be transcribe, speak or voices.")


if __name__ == "__main__":
    main()
