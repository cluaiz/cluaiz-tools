# Safe runs

## What the Engine does

- Writes the code to a temporary file in `scratch/` inside the open folder, runs it there, and deletes the file afterwards.
- Stops the run after 30 seconds, or after `timeout`, together with the programs it started.
- Does not limit memory or disk, and does not sandbox. The code runs with the user's rights.

## What the code must do

- **Bounded work.**
  - Every loop has an end.
  - A long job prints progress.
  - When the full job would not finish in time, it stops early with a sample, and the report says so.
- **Memory.** Read big files line by line or in chunks, for example `pandas.read_csv(..., chunksize=100_000)` or `nrows=` for a first look.
- **Output.** Print summaries; everything printed comes back into the conversation.
- **Child processes.**
  - Give each one a timeout.
  - Pass an argument list, not a shell string.
  - Never build a command from user text.
- **Files.**
  - Read what the user points to.
  - Write only what the task needs, under a new name, where the user expects it.
  - Never use `rm -rf`, `shutil.rmtree`, `del /s` or `format` unless the user asked for exactly that path.
- **Network.**
  - Fetch only what the task needs, from the address given or a well-known source, and say which addresses were contacted.
  - Read keys from environment variables, and never print them.
