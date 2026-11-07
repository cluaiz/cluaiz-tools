---
name: time
description: "The current date and time on this computer, and exact date arithmetic, so they are read and computed rather than guessed."
---

# Time

You do not know the current date or time. Call `time_now`.

```json
{"iso_8601": "2026-11-07T20:00:00+05:30", "date": "2026-11-07", "time": "20:00:00", "day_of_week": "Saturday", "utc_offset": "+05:30", "utc": "2026-11-07T14:30:00Z", "timestamp_ms": 1794061800000, "system": "windows"}
```

Named time zones such as `America/New_York` are not known, because their offsets change through the year. For another place, pass its `utc_offset` if you are sure of it, and say which offset you used.

## Date arithmetic

Call `date_calc` instead of counting days yourself:

| To find | Pass |
|---|---|
| The date 90 days from today | `{"add_days": 90}` |
| The date 3 months after 2026-01-31 | `{"date": "2026-01-31", "add_months": 3}` |
| Days from today to 25 December | `{"until": "2026-12-25"}` |
| The day of the week of a date | `{"date": "2027-03-01"}` |

Dates are `YYYY-MM-DD`. The answer has `result` and `result_day_of_week` for an addition, and `days_between` for `until`, which is negative when `until` is earlier.
