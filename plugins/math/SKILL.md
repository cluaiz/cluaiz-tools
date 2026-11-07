---
name: math
description: "Evaluates math expressions, so results are computed rather than guessed."
---

# Math

Call `math_evaluate` whenever a result needs arithmetic. Do not work it out yourself.

```json
{"expression": "(2^10 + sqrt(144)) / 4", "precision": 2}
```

The answer has `result`, `formatted`, and `error` if the expression is not valid.

- Angles are in radians.
- `log` is not accepted, because it can mean base 10 or base e. Use `log10` or `ln`.
- Numbers are 64-bit floats. A `note` in the answer means the result is too large to be exact.
