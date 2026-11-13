---
name: math
description: "Evaluates math expressions and converts units, so results are computed rather than guessed. Use whenever an answer needs arithmetic, a formula or a unit conversion."
---

# Math

Do not work out a result yourself. Call a function, then give its answer.

- `math_evaluate` with `expression`, and `precision` for the digits after the point. The answer has `result` and `formatted`, or `error`.
- `math_convert` with `value`, `from` and `to`. The answer has `result`, `formatted` and the `quantity`, or `error` when the units do not match or are unknown.

Rules for expressions:

- Angles are in radians. To use degrees, convert first with `math_convert` (`deg` to `rad`).
- A bare `log(x)` is refused, because its base is unclear. Use `log(x, base)`, `log10` or `ln`.
- `n!`, `factorial(n)`, `nCr(n, r)` and `nPr(n, r)` need whole numbers. `170!` is the largest.
- `min` and `max` take any number of arguments.
- Numbers are 64-bit floats. A `note` in the answer means the result is too large to be exact.

Rules for units:

- Case matters where two units differ only by case: `MB` is megabytes, `Mb` is megabits. When the answer says a unit is unclear, ask the user or use the exact symbol.
- Gallons, pints and cups are US units unless the user means imperial (`imp_gal`, `imp_pt`). Say which you used.
- Money and currencies are not units here: their rates change daily.
