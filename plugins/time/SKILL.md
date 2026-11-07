---
name: time
description: "The current date and time on this computer, so they are read rather than guessed."
---

# Time

You do not know the current date or time. Call `time_now`.

```json
{"iso_8601": "2026-11-07T20:00:00+05:30", "date": "2026-11-07", "time": "20:00:00", "day_of_week": "Saturday", "utc_offset": "+05:30", "utc": "2026-11-07T14:30:00Z", "timestamp_ms": 1794061800000, "system": "windows"}
```

Named time zones such as `America/New_York` are not known, because their offsets change through the year. For another place, pass its `utc_offset` if you are sure of it, and say which offset you used.
