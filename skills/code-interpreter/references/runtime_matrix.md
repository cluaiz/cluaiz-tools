# Runtime Probing Matrix & Platform Resolution

This reference defines the cross-platform command resolution sequence for the `code-interpreter` skill.

---

## 1. Fast Probing Sequence (<10ms Target)

When probing the environment, execute the candidate commands in order of priority:

| Priority | Runtime | Windows Candidate | Linux / macOS Candidate | Exit Check |
| :--- | :--- | :--- | :--- | :--- |
| **1. Primary** | Python 3 | `python --version` (or `py -3 --version`) | `python3 --version` (or `python --version`) | `Python 3.x.x` in output |
| **2. Fallback** | Node.js | `node --version` | `node --version` (or `nodejs -v`) | `vXX.x.x` in output |
| **3. VirtualEnv** | Local venv | `.venv\Scripts\python.exe --version` | `.venv/bin/python --version` | Virtual environment priority |

---

## 2. Platform-Specific Installation Fallbacks (Permission Required)

If neither runtime is found, suggest the standard package manager command appropriate for the user's operating system:

* **Windows:**
  ```powershell
  winget install Python.Python.3.12
  ```
* **macOS (Homebrew):**
  ```bash
  brew install python
  ```
* **Linux (Debian / Ubuntu):**
  ```bash
  sudo apt-get update && sudo apt-get install -y python3
  ```

> ⚠️ **MANDATORY PERMISSION GATE:** Never run any installation command automatically. Always present the detected platform command to the user and wait for explicit confirmation.

---

## 3. Language Selection Heuristics

* **Choose Python when:**
  - Task involves statistical analysis, math, tensor operations, matrix calculation, regex, or file parsing.
  - User explicitly specifies Python.
* **Choose Node.js when:**
  - Python is missing from the system.
  - Task involves JSON manipulation, web payload parsing, asynchronous event simulation, or string transformations.
  - User explicitly specifies JavaScript.
