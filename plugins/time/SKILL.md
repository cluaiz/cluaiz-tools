---
name: time
description: Provides real-time local date, system clock, UTC timestamp, timezone, and OS locale offline across Windows, macOS, and Linux without network requests.
version: 1.0.0
triggers:
  - "what time is it"
  - "current time"
  - "today's date"
  - "current date"
  - "what day is today"
  - "system clock"
  - "local time"
execution_mode: auto
default_turns: 1
---

# 🕒 Time & System Locale Plugin

This plugin supplies accurate, real-time operating system clock and locale metadata to ensure the model never hallucinates current time, day of the week, year, or geographic timezone.

## ⚙️ Capabilities
- **Local Time & Date:** Accurate year, month, day, hours, minutes, and seconds.
- **UTC Timestamp:** Standardized ISO-8601 formatted datetime string.
- **Timezone & Offset:** Accurate local timezone abbreviation and UTC offset (e.g. `+05:30`, `UTC`, `EST`).
- **System Locale:** OS language and region setting.

## 📋 Response Schema (JSON)
```json
{
  "iso_8601": "2026-08-29T20:00:00+05:30",
  "date": "2026-08-29",
  "time": "20:00:00",
  "day_of_week": "Saturday",
  "timezone": "IST",
  "utc_offset": "+05:30"
}
```

## ⚡ Usage
Emit standard tool call when local date, clock, UTC timestamp, or timezone is required:
```xml
<tool_call>
{"name": "time_now", "arguments": {}}
</tool_call>
```
