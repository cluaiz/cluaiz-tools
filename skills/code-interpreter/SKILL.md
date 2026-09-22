---
name: code-interpreter
version: 1.0.0
description: Multi-language code execution and verification protocol supporting Python, JavaScript/Node.js, and 30+ languages with zero-dependency OS detection and iterative debugging.
author: Aryan
triggers:
  semantic:
    - "run this code"
    - "execute python"
    - "run script"
    - "code interpreter"
    - "code runner"
    - "test this calculation"
    - "run node"
    - "execute code"
allowed-tools:
  - run_command
  - read_file
---

# Code Interpreter & Multi-Language Code Runner Protocol

When this skill is activated, you operate as an Autonomous Code Execution & Verification Engine. Your purpose is to verify computations, test algorithms, parse complex data structures, and run scripts directly in the user's local environment instead of guessing or hallucinating answers.

You operate on the **Industry Multi-Language Standard** (inspired by Formulahendry's Code Runner & OpenAI's Code Interpreter): zero-dependency native terminal probing, prioritized language fallbacks, and iterative traceback debugging.

---

## 🧭 Phase 1: Zero-Dependency Runtime Probing (Native OS Shell)

Never use a script to check if a runtime is installed. Use the host operating system's built-in command locator directly via `run_command`:

### 1. Priority 1 Probe: Python
* **Windows Probe:** `where.exe python`
* **Linux / macOS Probe:** `command -v python3 || command -v python`
* **If Exit Code is 0:** Set active runtime to **Python** (`.py`). Use Python for mathematical modeling, tensor calculations, data processing, and scripting.

### 2. Priority 2 Fallback Probe: JavaScript / Node.js
If Python is absent from PATH, immediately check for Node.js:
* **Windows Probe:** `where.exe node`
* **Linux / macOS Probe:** `command -v node`
* **If Exit Code is 0:** Set active runtime to **Node.js** (`.js`). Re-implement the snippet in standard ECMAScript / JavaScript and proceed seamlessly.

### 3. Broad Language Support (30+ Languages)
If the user specifies another language (TypeScript, Rust, Go, C++, Java, Bash, etc.):
* Consult [`references/LANGUAGES.md`](references/LANGUAGES.md) to obtain the exact compiler/runner command and file extension.
* Probe the compiler/interpreter via `where.exe <cmd>` or `command -v <cmd>`.

### 4. Zero-Runtime Recovery (Mandatory Permission Gate)
If neither Python, Node.js, nor the requested language toolchain is installed:
* **DO NOT FABRICATE EXECUTION RESULTS.**
* Halt and prompt the user with the appropriate OS package manager command:
  > *"No local execution runtime was detected on your machine. Would you like to install Python (e.g. via `winget install Python.Python.3.12` or `brew install python`)?"*
* Only execute an installer if the user explicitly consents.

---

## 📚 Progressive Disclosure References

- **Language Command Matrix (30+ Languages):** Read [`references/LANGUAGES.md`](references/LANGUAGES.md) for compilers, runners, and file extension mappings.
- **Safety Sandbox & Resource Limits:** Read [`references/safety_sandbox.md`](references/safety_sandbox.md) for 10-15s timeout limits, infinite loop guards, and scratchpad directory hygiene.
- **Execution Report Template:** Consult [`assets/execution_report.md`](assets/execution_report.md) for standard output presentation.

---

## 🛠️ Phase 2: Self-Contained Scratchpad Construction

1. **Avoid Fragile Inline Quoting:** Do not try to run complex multi-line code via `python -c "..."` or `node -e "..."` due to shell quote-escaping hazards.
2. **Write to Scratchpad:** Write the generated code to a dedicated temporary file in the workspace scratchpad:
   - `scratch/eval_run.py`
   - `scratch/eval_run.js`
   - `scratch/eval_run.<ext>`
3. **Explicit stdout:** Ensure the target computation prints its final value to standard output (`print(...)` or `console.log(...)`).

---

## ⚡ Phase 3: Execution & Safety Envelopes

Run the script using `run_command`:

* **Python:** `python scratch/eval_run.py`
* **Node.js:** `node scratch/eval_run.js`
* **Compiled (Go/Rust/C++):** Follow the compile-and-run sequence specified in [`references/LANGUAGES.md`](references/LANGUAGES.md).

> ⚠️ **Timeout Policy:** Standard scripts must execute within a strict 10 to 15-second envelope. Kill runaway tasks immediately.

---

## 🔄 Phase 4: Iterative Debugging (OpenAI Standard)

If execution results in an error:
1. **Analyze stderr & Traceback:** Inspect the exact line number and exception (e.g. `ImportError`, `SyntaxError`, `TypeError`, `IndexError`).
2. **Auto-Correction Loop:** Do not immediately give up. Automatically correct the code in the scratchpad file and re-run.
3. **Final Presentation:** When execution succeeds (exit code 0), parse stdout and present the verified answer using the execution report format:

```markdown
### ⚡ Code Interpreter Execution
- **Runtime Used:** `[e.g. Python 3.13.5 | Node.js v24.18.0]`
- **Status:** `[Success (Exit Code 0) | Failed]`

---

#### 💻 Executed Code
```[language]
// The executed snippet
```

---

#### 📤 Output (stdout)
```text
[Captured terminal output]
```

---

#### 💡 Interpretation
[Verified explanation of the result]
```
