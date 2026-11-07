# My Assistant Companion Extension (`@cluaiz/extension-my-assistant`)

Zero-VRAM, 60fps HTML5 Canvas desktop companion and interactive character engine for Cluaiz.

## Architectural Overview

`my-assistant` provides an interactive character avatar for web apps, Tauri overlays, or desktop sidebars. Rather than requiring heavy video streams or 3D engines that consume gigabytes of VRAM, `my-assistant` runs completely on a 2D HTML5 Canvas rendering loop with minimal CPU and zero GPU memory overhead.

### Key Capabilities
1. **Interactive Canvas Character**:
   - Procedural rendering of head, expressive eyes, pupils with mouse tracking, dynamic mouth, blush, and antennas/ears.
   - Expression states: `idle`, `thinking`, `speaking`, `listening`, `happy`, `surprised`, `sleepy`.
   - Physics-based drag-and-drop with inertia, screen edge snapping, and persistent position storage (`localStorage`).
2. **Lip-Sync & Speech Synthesis Bridge**:
   - Integrates with `@cluaiz/extension-browser-voice` via `BrowserTTS.onboundary` and `SpeechSynthesisUtterance` word events.
   - Mouth shapes dynamically cycle (`viseme` simulation) based on phoneme speed, word cadence, and speech synthesis boundaries.
3. **Task & State Awareness**:
   - Reacts to engine events: changes expression during LLM generation, nods on tool execution, winks upon task completion.

## Usage

```typescript
import { AssistantCanvas } from '@cluaiz/extension-my-assistant';
import { BrowserTTS } from '@cluaiz/extension-browser-voice';

// 1. Initialize canvas companion
const companion = new AssistantCanvas({
    container: document.body,
    size: 140,
    theme: {
        bodyColor: '#3B82F6',
        eyeColor: '#1E293B',
        accentColor: '#F59E0B'
    }
});

// 2. Link speech synthesis to lip-sync
const tts = new BrowserTTS();
companion.attachTTS(tts);

// 3. Play speech with real-time lip sync
tts.speak('Task completed successfully.');
```

## Permissions
- `canvas`: DOM Canvas 2D rendering context.
- `audio-sync`: Speech synthesis boundary event hooks.
- `drag-drop`: Pointer/mouse coordinate tracking for drag physics.
- `storage`: Saving position coordinates and user companion settings.
