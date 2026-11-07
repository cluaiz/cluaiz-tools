# Browser Voice Companion (`browser-voice`)

A lightweight, Zero-VRAM Audio Extension for the Cluaiz ecosystem.

## Features
- **Zero VRAM Footprint:** Leverages host-native Web Speech API. Requires 0 MB of GPU memory and zero gigabyte neural weight downloads.
- **Dual Voice Pipeline:**
  - **Native OS Engine:** 100% offline, system-level speech synthesis.
  - **Microsoft Neural Engine:** Studio-grade natural speech voices (e.g. `AriaNeural`, `SwaraNeural`).
- **Live Text Highlighting (Karaoke Sync):** Real-time word-by-word tracking via speech synthesis boundary events.
- **Continuous STT:** Real-time microphone capture with interim and final transcription dispatch.

## Architecture & Integration
Compatible with both standard web browsers and the native desktop application (`cluaiz-app` on Tauri + Next.js).
