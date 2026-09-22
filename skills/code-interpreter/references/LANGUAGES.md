# Language Execution Configuration Matrix

This reference maps programming languages to their respective interpreters, compilers, execution commands, and zero-dependency OS detection commands.

---

## 1. Primary Supported Runtimes

| Language | Interpreter / Compiler | Extension | Execution Command | Windows Probe | Linux / macOS Probe |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **Python** (Priority 1) | Python 3 | `.py` | `python scratch/eval.py` | `where.exe python` | `command -v python3 \|\| command -v python` |
| **JavaScript** (Priority 2) | Node.js | `.js` | `node scratch/eval.js` | `where.exe node` | `command -v node` |
| **TypeScript** | ts-node / Node | `.ts` | `npx ts-node scratch/eval.ts` | `where.exe npx` | `command -v npx` |
| **PowerShell** | PowerShell | `.ps1` | `pwsh -File scratch/eval.ps1` | `where.exe pwsh` | `command -v pwsh` |
| **Bash** | Bourne-Again SH | `.sh` | `bash scratch/eval.sh` | `where.exe bash` | `command -v bash` |

---

## 2. Compiled & Systems Languages

| Language | Compiler / Runner | Extension | Compile & Run Command (POSIX) | Compile & Run Command (Windows) | Probe Command |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **Rust** | `rustc` | `.rs` | `rustc scratch/eval.rs -o scratch/eval && scratch/eval` | `rustc scratch/eval.rs -o scratch/eval.exe && scratch\eval.exe` | `where.exe rustc` / `command -v rustc` |
| **Go** | `go run` | `.go` | `go run scratch/eval.go` | `go run scratch/eval.go` | `where.exe go` / `command -v go` |
| **C** | `gcc` / `clang` | `.c` | `gcc scratch/eval.c -o scratch/eval && scratch/eval` | `gcc scratch/eval.c -o scratch/eval.exe && scratch\eval.exe` | `where.exe gcc` / `command -v gcc` |
| **C++** | `g++` / `clang++` | `.cpp` | `g++ scratch/eval.cpp -o scratch/eval && scratch/eval` | `g++ scratch/eval.cpp -o scratch/eval.exe && scratch\eval.exe` | `where.exe g++` / `command -v g++` |
| **Java** | `java` (single-file) | `.java` | `java scratch/eval.java` | `java scratch/eval.java` | `where.exe java` / `command -v java` |

---

## 3. Scripting & Interpreted Alternatives

| Language | Interpreter | Extension | Execution Command | Probe Command |
| :--- | :--- | :--- | :--- | :--- |
| **PHP** | `php` | `.php` | `php scratch/eval.php` | `where.exe php` / `command -v php` |
| **Ruby** | `ruby` | `.rb` | `ruby scratch/eval.rb` | `where.exe ruby` / `command -v ruby` |
| **Lua** | `lua` | `.lua` | `lua scratch/eval.lua` | `where.exe lua` / `command -v lua` |
| **R** | `Rscript` | `.r` | `Rscript scratch/eval.r` | `where.exe Rscript` / `command -v Rscript` |

---

## 4. Zero-Dependency Probing Heuristics

1. **Native Shell Execution:** Always use the host OS terminal built-in (`where.exe` on Windows, `command -v` on POSIX) to test if an interpreter exists.
2. **Never Run Chicken-and-Egg Scripts:** Do not write Python or Node scripts just to test if Python or Node is installed.
3. **Exit Code Check:**
   - Exit code `0`: Executable is installed and in PATH.
   - Non-zero: Executable is absent; move to fallback runtime.
