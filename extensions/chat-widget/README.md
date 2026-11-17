# `@cluaiz/chat-widget` Extension

> **Experimental, not registered.** This extension is not in `extensions/family.json`, is not installable
> from the registry, and has not been tested end to end.

The interface of a chat widget for a web page: launcher button, chat window and sound alerts. It is not
connected to the Engine yet: it reads its settings from `<apiUrl>/bots/default_bot/config`
(`src/services/ConfigService.ts:17`), a route the Engine does not have, and sends no chat messages.

## Overview
- **Launcher button**: an animated SVG avatar, colour settings and an unread-message badge.
- **Chat window**: an expandable panel (interface only).
- **Sound alerts**: a short sound when a new answer arrives.

## Architecture
```
Host Webpage (DOM)
  └── Launcher Button (Interactive Robot Avatar)
       ├── Unread Notification Badge
       └── Popover Preview Bubble
  └── Chat Window
       ├── Header & Status Indicator
       ├── Message list
       └── Input Bar
```

## Manifest Specification
- `hub_type`: `"extension"`
- `permissions`: `["dom-injection", "storage", "audio-output"]`
- `main`: `src/index.ts`
