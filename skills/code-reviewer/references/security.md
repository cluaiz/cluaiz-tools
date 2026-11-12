# Security

Check only the items that apply to the change. Validate untrusted data at the point where it enters the system.

## Input from outside

- **Commands:** user input, file content or web data must never reach a shell as a string (`exec`, `system`, `sh -c`, `shell=True`). Pass an argument list straight to the program.
- **Queries:** queries are parameterised, never built by string concatenation.
- **Paths:** a path that comes from input is resolved and checked to stay inside its folder.
- **Untrusted formats:** deserializers and template engines never receive untrusted data in an unsafe mode.
- **Size limits:** request bodies, uploads, parsed files and page sizes all have limits.

## Fetching URLs from users (SSRF)

- Allow only the schemes the feature needs, usually `https:`.
- Block loopback addresses (`127.0.0.1`, `localhost`, `::1`), private ranges (`10.0.0.0/8`, `172.16.0.0/12`, `192.168.0.0/16`) and the metadata address `169.254.169.254`.
- Check the address after DNS resolution and after every redirect, not only the URL text.

## Auth

- Every new route has the same authentication as its neighbours.
- Every query filters by tenant or user, so one user cannot read another user's records (IDOR).
- Secrets and tokens are compared in constant time (`hmac.compare_digest`, `crypto.timingSafeEqual`, `subtle::ConstantTimeEq`).
- Cookies that carry a session are `HttpOnly`, `Secure` and `SameSite`. State-changing requests are protected against CSRF when cookies authenticate.
- CORS allows only the origins the app needs. `*` together with credentials is a hole.

## Web clients

- `innerHTML`, `dangerouslySetInnerHTML` and `v-html` with user data are unsafe unless the data passes a strict sanitizer such as DOMPurify.
- A redirect to a URL taken from a query parameter is an open redirect unless the target domain is on an allow list.

## Secrets and errors

- No keys, passwords or tokens in code, tests or logs.
- Error responses do not leak stack traces, SQL or internal paths.
- Errors are never turned into a fake success.
- Retries have a limit and waits have a timeout.

## Resources and concurrency

- Files, sockets, connections and child processes are released on every path, including errors and cancellation.
- A child process that reads its input gets that input closed, so it does not wait forever.
- Shared state is protected, and locks are taken in the same order everywhere. No lock is held across a slow call or an `await`.
- Queues and buffers have a limit.

## Numbers

- Sizes, offsets and money cannot overflow or lose precision.
- Indexes are checked against the length, and narrowing conversions are checked.

## Weak crypto

- MD5 and SHA-1 are not used for security.
- Randomness for secrets comes from a cryptographic source.
- TLS certificate checks are never turned off.
