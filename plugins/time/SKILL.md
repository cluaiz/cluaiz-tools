---
name: time
description: "The current date and time here or in any time zone, time zone conversion, date arithmetic, calendars, countdowns, a clock check and public holidays, read and computed rather than guessed. Use whenever an answer depends on the date, the time, a time zone or counting days."
---

# Time

You do not know the current date or time, and you cannot count days reliably. Call a function.

| To | Call |
|---|---|
| Know the date or time, here or in a place | `time_now`, with `timezone` for another place |
| Move a time between zones, as for a meeting | `time_convert` with `time`, `from_timezone` and `to_timezone` (a list) |
| Add or subtract time, count days between dates, find an age or the next weekday | `date_calc` |
| Show a month | `calendar` with `month` |
| Say how long until or since a moment | `countdown` with `target` |
| List a country's public holidays | `holidays` with `country` |
| Check whether this computer's clock is right | `time_now` with `check_clock: true` |

## Time zones

- Give a zone as an IANA name (`Asia/Kolkata`), a city that is part of one (`Tokyo`, `New York`), `UTC`, or an offset (`+05:30`). For a city that is not in a zone's name, pass the name of its zone.
- Abbreviations such as IST or CST are refused, because several zones share them. Ask the user which place they mean.
- This computer gives only its UTC offset, not its zone's name. When daylight saving or the user's place matters, ask the user for their city.
- When an answer has `day_change` or `note`, tell the user: the date is different there, or the time does not exist or happens twice because the clocks change.

## Dates

- Dates are `YYYY-MM-DD`; `today` is accepted. An age is `date_calc` with the birth date as `date` and `until: "today"`; read `difference`.
- Working days are Monday to Friday. Public holidays are not skipped: say so, or check `holidays` first.

## Internet

`check_clock` and `holidays` need the `network` permission. If the answer says it is off, pass on the command it gives to turn it on. If the holiday service does not cover a country, say so; do not list holidays from memory.

## What this tool cannot do

It cannot save events or wake up later, so it cannot set an alarm, a reminder or a meeting. Say so instead of promising one.
