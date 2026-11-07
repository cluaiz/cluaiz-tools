# Checklist

Use what applies to the language and the change.

## Input from outside
- [ ] User input, file content and web data never reach a shell command, a query, HTML or a file path without escaping or checks.
- [ ] Commands take an argument list, not a string built from input.
- [ ] A path from input is resolved and checked to stay inside the folder it belongs to.
- [ ] Size limits exist for uploads, request bodies and parsed files.

## Secrets
- [ ] No key, password or token is in the code, the tests or the logs.
- [ ] Error messages do not print secrets or full internal paths to users.

## Errors
- [ ] Every failure is returned or reported, and never turned into a fake success.
- [ ] Code that cannot fail in practice still does not crash the process on bad input.
- [ ] Retries have a limit, and waits have a timeout.

## Resources
- [ ] Files, sockets, database connections and child processes are closed or stopped on every path, including errors and cancellation.
- [ ] A child process that reads input gets its input closed, so it does not wait forever.

## Concurrency
- [ ] Shared state is protected, and locks are taken in the same order everywhere.
- [ ] A lock is not held across a slow call or an `await`.
- [ ] Queues and buffers have a limit.

## Numbers and data
- [ ] Sizes, offsets and money cannot overflow or lose precision.
- [ ] Indexes are checked against the length.
- [ ] Narrowing conversions are checked.

## Structure
- [ ] Logic is not copied between places; a shared function is used.
- [ ] No machine-specific paths or settings are written into the code.
