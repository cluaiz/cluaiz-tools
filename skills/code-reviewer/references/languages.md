# Language checks

Read the part for each language in the change. These are the mistakes reviews find most often in each one; the general passes in SKILL.md still apply. Check what the project's linter already enforces before reporting a style rule.

## Python

- Mutable default arguments (`def f(items=[])`) shared between calls.
- `except:` or `except Exception: pass` hiding errors; `raise` inside `except` without `from` losing the cause.
- `subprocess` with `shell=True` or a string built from input; `os.system`, `eval`, `exec`, `pickle.loads` and `yaml.load` (without `SafeLoader`) on untrusted data.
- Files and connections opened without `with`.
- `==` on floats for money; use `decimal.Decimal` or integer cents.
- Naive `datetime` mixed with aware ones; `datetime.now()` where UTC is meant.
- Blocking calls (`requests`, `time.sleep`, file I/O) inside `async def`.
- Changing a list or dict while looping over it.
- Imports with side effects, and circular imports.

## JavaScript and TypeScript

- A promise not awaited or not returned, so errors vanish; `forEach` with `async` callbacks (they are not awaited).
- `==` where `===` is meant; `||` used for defaults where `0` or `""` are valid (`??` is meant).
- `any`, `as` casts and `!` non-null assertions that hide a real `undefined` (TypeScript).
- `innerHTML`, `dangerouslySetInnerHTML`, `eval`, `new Function` with data from users.
- In React: a missing dependency in `useEffect`/`useMemo`, state changed in place, keys from the array index in lists that reorder, effects without cleanup (timers, listeners, subscriptions).
- In Node: `child_process.exec` with built strings, paths joined from input without a check, unhandled stream `error` events.

## Go

- An `err` ignored (`_ =`, or the result not checked) or checked after the value is used.
- `defer` in a loop (resources held until the function ends); `defer f.Close()` ignoring the close error on writes.
- Goroutines that never end (no `context` cancel, no channel close); a loop variable captured by a goroutine (before Go 1.22).
- Maps written from several goroutines without a lock; `sync.WaitGroup.Add` inside the goroutine.
- `panic` in library code instead of returning an error; nil maps written to; nil pointer from a type assertion without `ok`.
- HTTP clients without a timeout (`http.DefaultClient`), response bodies not closed.

## C and C++

- Buffer overflows: `strcpy`, `strcat`, `sprintf`, `gets`, unchecked indexes and lengths, `memcpy` sizes from input.
- Memory: use after free, double free, leaks on early returns; raw `new`/`delete` where `std::unique_ptr` or a container fits (C++).
- Integer overflow and signed/unsigned mixing in sizes and loops; `size_t` underflow (`len - 1` when `len == 0`).
- Undefined behaviour: uninitialized variables, dangling references to temporaries, iterator invalidation after `push_back` or `erase`, data races.
- Missing `virtual` destructor in a base class used through pointers; rule of three/five broken by a class that owns a resource.
- Return values of `malloc`, `fopen`, `read`, `write` not checked.

## Java and Kotlin

- `catch (Exception e) {}` or only `e.printStackTrace()`; resources without try-with-resources (`use` in Kotlin).
- `equals` without `hashCode`; `==` on strings or boxed numbers (Java).
- Shared mutable state without synchronization; `SimpleDateFormat` shared between threads.
- `!!` in Kotlin on values that can be null; platform types from Java treated as non-null.
- SQL built with string concatenation instead of `PreparedStatement` or bound parameters; `ObjectInputStream` on untrusted data.
- On Android: work on the main thread, leaked `Context` or `Activity` references, missing lifecycle handling.

## C#

- `async void` outside event handlers; `.Result` or `.Wait()` causing deadlocks; missing `ConfigureAwait` in libraries where the project uses it.
- `IDisposable` not disposed (`using` missing); `HttpClient` created per request.
- `catch {}` or catching `Exception` and going on; `BinaryFormatter` on untrusted data.
- LINQ queries run several times, or loading a whole table to filter in memory.

## Swift

- `try!`, `as!` and force unwrapping (`!`) on values that can be nil.
- Retain cycles in closures (`[weak self]` missing) and delegates that are not `weak`.
- UI changed off the main thread; `@MainActor` missing on UI code.
- Errors swallowed with `try?` where the caller needs to know.

## PHP and Ruby

- SQL built from request data instead of bound parameters; `unserialize` and `Marshal.load` on input.
- Output not escaped in templates (`{!! !!}` in Blade, `raw`/`html_safe` in Rails).
- Mass assignment of request parameters to models without an allow list.

## SQL and migrations

- A migration that locks a large table, drops or renames a column still in use, or cannot be rolled back.
- A new column `NOT NULL` without a default on a table with rows.
- Queries without an index on the filtered column; `SELECT *` in code that maps columns by position.
- Missing tenant or user filter (one user can read another's rows).

## Shell and PowerShell

- Unquoted variables (`$file` instead of `"$file"`) splitting on spaces; `rm -rf "$dir/"` with `$dir` possibly empty.
- No `set -euo pipefail` (bash) where a failed step must stop the script.
- Secrets passed on the command line (visible in process lists) or echoed into logs.
- Windows PowerShell 5.1: `&&` does not exist; `$?` and native exit codes behave differently.

## Dart and Flutter

- `setState` after `dispose`; controllers and streams not disposed.
- Work on the UI isolate that freezes frames; `!` on nullable values.
- `BuildContext` used across an `await` without checking `mounted`.

## Rust

- `unsafe` without a `// SAFETY:` comment that matches the code; raw pointer reads without established alignment, non-null and lifetime; `transmute` of a type that is not `#[repr(C)]` or `#[repr(transparent)]`.
- A `std::sync::MutexGuard` held across `.await` (blocks the executor, makes the future not `Send`): copy what is needed and drop the guard first, or use `tokio::sync::Mutex`. Locks taken in different orders on different threads.
- `.unwrap()` and `.expect()` on input from users, files or the network; `let _ = result;` or `.ok()` throwing an error away when the caller then acts as if it worked.
- A `Vec` growing in a hot loop when the size is known (`with_capacity`); memory-mapped files read past a truncated end (`SIGBUS`, `STATUS_IN_PAGE_ERROR`).
