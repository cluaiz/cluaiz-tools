# Security & System Safety Checklist

Use this checklist during Phase 2 of code review to audit systems code (Rust, C++, Python, TypeScript).

---

## 1. Memory Safety & Bounds Invariants
- [ ] **Slice Indexing:** All slice indices (`&slice[start..end]` or `slice[idx]`) are validated against `.len()` or wrapped in `.get()`.
- [ ] **Pointer Offsets:** Pointer offsets and buffer steps are verified against allocation boundaries.
- [ ] **Integer Arithmetic:** Operations on buffer capacities, allocations, and offsets use safe arithmetic (`checked_add`, `saturating_mul`, `checked_sub`) to prevent wrapping.
- [ ] **Zero-Copy Borrows:** Lifetimes of borrowed buffers are scoped to prevent dangling references or use-after-free conditions.

---

## 2. Concurrency, Locks & Deadlocks
- [ ] **Lock Invariants across Await:** Mutex guards (`tokio::sync::Mutex`, `std::sync::Mutex`) are dropped before `.await` points to prevent deadlocks and thread starvation.
- [ ] **Lock Ordering:** Multiple nested locks are acquired in a deterministic global order across threads.
- [ ] **Channel Buffering:** Bounded channels (`mpsc::channel(capacity)`) have explicit capacity limits and backpressure handling.
- [ ] **Atomic Ordering:** Atomics (`AtomicBool`, `AtomicUsize`) use appropriate memory orderings (`Acquire`/`Release` or `SeqCst`) rather than loose `Relaxed` where synchronization is required.

---

## 3. Subprocess & IPC Integrity
- [ ] **Timeout Wrappers:** Every child process execution (`tokio::process::Command`, stdio pipes) is bounded by an explicit timeout (`tokio::time::timeout`).
- [ ] **Stdin Closure:** Standard input pipes are explicitly closed or flushed to prevent child processes from hanging indefinitely awaiting EOF.
- [ ] **Zombie Process Harvesting:** Child process handles are explicitly killed and awaited upon cancellation or error.
- [ ] **Command Argument Injection:** Process execution uses structured argument vectors (`.args(&[...])`) rather than concatenating untrusted strings into a shell command.

---

## 4. Input Validation & Deserialization
- [ ] **Untrusted JSON/YAML Parsing:** Inputs from network requests or untrusted files enforce max payload size limits before deserialization.
- [ ] **Path Traversal Guards:** File paths derived from user input or external metadata are canonicalized and verified to stay within designated root directories.
- [ ] **Safe Casting:** Narrowing integer casts (e.g. `u64` as `u32`) are validated to prevent data truncation.

---

## 5. Honest Error Propagation & Recovery
- [ ] **Zero Synthetic Successes:** Unimplemented or stubbed logic must never return synthetic `Ok(json!({"status": "success"}))` values.
- [ ] **Explicit Result Bubbling:** Errors are captured and bubbled up using typed error structures rather than uninformative generic strings.
- [ ] **No Panic in Production Paths:** Avoid unchecked `.unwrap()` or `.expect()` calls in request-handling hot paths; use `?` or explicit match handling with fallbacks.

---

## 6. Architecture & Platform Abstraction
- [ ] **Absolute DRY:** Shared algorithms, serializers, and protocol adapters are unified into shared crates or modules.
- [ ] **No Hardcoded Host Paths:** File paths do not embed static directories like `C:\Users\...` or `/tmp/...`. Use runtime configuration injection or dynamic path resolvers.
- [ ] **Dynamic Hardware Queries:** Hardware capabilities (VRAM size, compute cores, SIMD instructions) are probed dynamically via system detectors rather than hardcoded constants.
