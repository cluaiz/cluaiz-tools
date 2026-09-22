# Code Review Rubric & Severity Calibration

This rubric defines the scoring criteria, severity levels, and noise-filtering policy for the Code Reviewer skill.

---

## 1. Severity Levels

| Level | Criteria | Example |
| :--- | :--- | :--- |
| 🔴 **Critical** | Crashes, memory corruption, remote code execution, unhandled data races, deadlocks under concurrency load, data loss. | Holding `MutexGuard` across an async `.await` boundary, out-of-bounds slice index without checks. |
| 🟠 **High** | Architectural DRY violations, missing subprocess timeouts, broken error propagation, logic bugs affecting core flows. | Spawning child process without `timeout`, duplicate parsing logic across 3 crates instead of shared module. |
| 🟡 **Medium** | Missing edge-case branches, suboptimal memory allocation (excessive cloning on hot paths), missing resource cleanup. | Unbounded in-memory buffer growth, missing fallbacks for optional parameters. |
| 🟢 **Low** | Minor naming inconsistencies, missing documentation comments for public items, non-critical abstraction cleanup. | Inconsistent enum variant naming, missing docstring on public trait method. |

---

## 2. Noise & Nitpick Filtering Policy

The Principal Reviewer focuses strictly on correctness, security, concurrency invariants, and architecture:

* 🚫 **Strictly Banned Nitpicks:**
  - Indentation, line breaks, or brace styles.
  - Personal formatting preferences handled by automated tools (`rustfmt`, `prettier`, `black`).
  - Theoretical micro-optimizations that have zero measurable benchmark impact.
  - Vague complaints without concrete, copy-pasteable replacement code.

* ✅ **Mandatory Reporting Criteria:**
  - Every reported finding MUST include the exact `file:line` location.
  - Every reported finding MUST explain the concrete risk or failure mode.
  - Every reported finding MUST provide a complete drop-in replacement code snippet.

---

## 3. DRY Score Calibration (1 to 10)

* **9 - 10 (Exemplary):** Logic is fully unified into modular shared traits or crates. Zero duplicate routines. Centralized dispatchers utilized.
* **7 - 8 (Acceptable):** Minimal localized duplication within non-critical utility helpers; core domains are well decoupled and isolated.
* **4 - 6 (Needs Refactoring):** Noticeable duplication of algorithmic logic, string parsing, or protocol dispatch across multiple crates or files.
* **1 - 3 (Critical Regression):** Copy-pasted multi-line routines across multiple domains, static paths hardcoded in core logic, tight cross-coupling.
