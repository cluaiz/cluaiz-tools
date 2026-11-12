# Git details

## Secrets

- **Never stage:**
  - `.env` and `.env.*.local`, `credentials.json`, `id_rsa`, `id_ed25519`, `*.pem`, `*.key`;
  - build output (`target/`, `node_modules/`, `dist/`, `build/`, `.next/`);
  - OS files (`.DS_Store`, `Thumbs.db`);
  - large binaries (`*.gguf`, `*.bin`, `*.onnx`, database dumps).
- **A committed secret is compromised.** Rotate the key with the provider first. Removing it from git does not undo the leak.
- **Not pushed, and it is the last commit:**
  1. `git reset --soft HEAD~1`
  2. Move the value to an ignored `.env` file.
  3. Stage the cleaned files and commit again.
- **Pushed:** removing it means rewriting history with `git filter-repo` or BFG and force pushing, which affects everyone on the branch. Explain this and let the user decide; never run it on your own.
- **Afterwards:** add the file to `.gitignore`, and keep a `.env.example` with placeholder values.

## One change per commit

- Each commit builds and passes the tests on its own, so it can be reverted or bisected.
- Unrelated changes, such as a style tweak and a bug fix, go in separate commits. Stage each set of files and commit them one at a time.
- Two changes in one file need `git add -p`. It is interactive, so suggest it to the user.
- A breaking change gets `!` after the type (`feat(api)!:`) or a `BREAKING CHANGE:` footer.

## Branches and history

- **Rebase or merge:** the repository decides. Look at `git log --graph --oneline -20` and `CONTRIBUTING.md`.
- **Branch names:** follow the repository's pattern (check `git branch -r`). Otherwise use `feat/`, `fix/`, `refactor/` or `docs/` plus a short name, and add the issue number when the team links them (`fix/123-empty-average`).
- **Tidying a branch before merge:**
  1. Make fixup commits with `git commit --fixup=<sha>`.
  2. Let the user run `git rebase -i --autosquash main`.
- **Never** rewrite `main` or a branch that others use.

## Conflicts

- **Which side is which:**
  - In a merge, the first side is the current branch (`HEAD`) and the second is the branch being merged.
  - In a rebase, the second side is the commit being replayed.
- **Two different additions** (imports, fields, entries): keep both, in the project's order.
- **One side changes the logic, the other changes the signature:** apply the logic change to the new signature, then build.
- **Two incompatible designs:** stop, show both sides with files and lines, and ask the user.
- **Finishing:** `git merge --continue` or `git rebase --continue` goes on. `--abort` returns to where the user started.
