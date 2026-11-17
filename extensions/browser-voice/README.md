# Browser Voice (`browser-voice`)

> **Experimental, not registered.** This extension is not in `extensions/family.json`, is not installable
> from the registry, and has not been tested end to end.

Speech in the browser or the App's WebView through the browser's Web Speech API, so no speech model is
downloaded.

## What it does
- **Text to speech** (`src/BrowserTTS.ts`): speaks text with the voices the system or browser provides,
  with word boundary events for highlighting the word being spoken.
- **Speech to text** (`src/BrowserSTT.ts`): microphone input with interim and final results.

Where speech is processed depends on the browser and the voice: some voices and some browsers' speech
recognition send audio to an online service.
