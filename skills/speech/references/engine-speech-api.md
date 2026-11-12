# How the Engine runs speech

The `speech` function calls the local Cluaiz Engine. It finds the address and token from the Cluaiz home (`CLUAIZ_HOME`, default `~/.cluaiz`) and `CLUAIZ_PORT` (default from the settings, else 8000).

| Action | Engine call |
|---|---|
| `transcribe` | `POST /v1/audio/transcriptions` with the file path |
| `speak` | `POST /v1/audio/speech`; the answer holds a WAV, which the function saves |
| `voices` | CEP `audio.voices` on `POST /v1/cep`; it reads the model's voice list without loading it |

## Transcription

- Input: a local file. The Engine decodes WAV, MP3, FLAC, OGG, Opus, M4A and AAC, converts to mono at the model's sample rate, and splits long audio into windows. No conversion or cutting is needed first.
- If the Engine says a format cannot be read, and `ffmpeg` is installed, convert it: `ffmpeg -i input.xyz -ar 16000 -ac 1 output.wav`. Ask before installing ffmpeg.
- `language`: a code such as `en` or `hi`. Without it, a multilingual model such as Whisper detects the language. `moonshine-tiny-ONNX` is English only.
- Timestamps per segment are not returned yet.
- Models: `onnx-community/moonshine-tiny-ONNX` (small, English), `onnx-community/whisper-large-v3-turbo` (many languages, larger). The user installs one with `cluaiz pull <id>`.

## Speech

- Kokoro voices are chosen with `voice`; an unknown name falls back to the model's default voice. Piper models have one voice.
- `language` selects the reading rules (numbers, spelling). Without it, the Engine infers it from the text and voice.
- The model: `onnx-community/Kokoro-82M-v1.0-ONNX`, installed with `cluaiz pull`.

## Errors

| The answer says | Do |
|---|---|
| The Engine is not running | Tell the user to run `cluaiz serve`, then stop |
| No speech model is installed | Tell the user the `cluaiz pull` command it gives; do not run it |
| No audio file at the path | Check the path with the user |
