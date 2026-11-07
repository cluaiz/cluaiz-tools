# `@cluaiz/chat-widget` Extension

The official Cluaiz Embeddable Web Chat Widget extension.

## Overview
- **Zero-VRAM Client Ingestion**: Injected directly into any browser webpage via a lightweight client script without requiring heavy local runtime binaries.
- **Interactive Robot Launcher**: Includes animated SVG robot avatar with dynamic eye blinking, cheek blush, custom color schemes, and unread notification badge counters.
- **Embedded Chat Modal**: Provides expandable chat drawer communicating with local or hosted Cluaiz inference endpoints over standard REST / SSE.
- **Sound Alerts**: Plays non-intrusive audio chimes when new assistant responses or background alerts trigger.

## Architecture
```
Host Webpage (DOM)
  └── Launcher Button (Interactive Robot Avatar)
       ├── Unread Notification Badge
       └── Popover Preview Bubble
  └── Chat Window
       ├── Header & Status Indicator
       ├── Message Stream (SSE)
       └── Input Bar
```

## Manifest Specification
- `hub_type`: `"extension"`
- `permissions`: `["dom-injection", "storage", "audio-output"]`
- `main`: `src/index.ts`
