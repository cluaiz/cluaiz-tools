# Runtimes

The Engine writes the code to a temporary file in `scratch/` and runs it with the command below. The runtime must be on `PATH`.

| `language` | Runs | Needs |
|---|---|---|
| `python`, `py` | the Python the Engine found | Python 3 |
| `javascript`, `js`, `node` | `node` | Node.js |
| `typescript`, `ts` | `npx ts-node` | Node.js; `ts-node` is fetched on first use (network) |
| `powershell`, `ps1` / `pwsh` | `powershell -File` / `pwsh -File` | Windows PowerShell / PowerShell 7 |
| `bash`, `sh` | `bash` | bash (Git Bash or WSL on Windows) |
| `go` | `go run` | Go |
| `java` | `java <file>.java` (one file with `main`) | Java 11+ |
| `rust` | `rustc`, then the program | Rust |
| `c`, `cpp` | `gcc` / `g++`, then the program | GCC or compatible |
| `php`, `ruby`, `lua`, `r` | `php`, `ruby`, `lua`, `Rscript` | that runtime |

On Windows, `rust`, `c` and `cpp` compile and run in one command joined by `&&`, which Windows PowerShell 5.1 rejects. If that run fails with a parser error, compile and run in two `run_command` calls instead.

## Choosing

- **Python** for numbers, statistics, data files, text and charts.
- **Node.js** when Python is missing, when the user asks for it, or for code from a JavaScript project.
- **The project's own language** when you are checking the project's code. To run the project's tests, use `run_command`.
- **To see what is installed,** run something in an installed language, for example Python's `shutil.which` on `node`, `go`, `java`, `gcc` and `rustc`. A project's virtual environment is at `.venv\Scripts\python.exe` on Windows and `.venv/bin/python` elsewhere. Use it when the code needs the project's packages.

## Installing

Install only when the user says so.

- **Python:** `winget install Python.Python.3.12` on Windows, `brew install python` on macOS, `sudo apt-get install -y python3` on Debian or Ubuntu.
- **A missing package** (`ModuleNotFoundError`): do the task with the standard library, or ask before running `python -m pip install <package>`.
