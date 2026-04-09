# awrust-cli — Architecture

**Status:** v0.1 — S3 support
**License:** MIT
**Distribution:** Cargo binary (`awr`)

---

## 1. Goals

### 1.1 Product goals

* **Ergonomic** CLI for interacting with awrust — fewer keystrokes than the AWS CLI.
* **Zero configuration** for the common case (localhost:4566).
* **Simple path syntax**: `bucket/key` instead of `s3://bucket/key`.

### 1.2 Engineering goals

* Minimal dependencies, fast compile times.
* Extensible to new awrust services (SQS, etc.) without architectural changes.
* Testable end-to-end against a real awrust container.

---

## 2. Non-goals

* Full AWS CLI parity.
* SigV4 request signing.
* JSON output mode (may come later).
* Recursive operations or glob patterns.

---

## 3. High-level architecture

### 3.1 Data flow

```
CLI args → clap parser → Command dispatch → Client → HTTP → awrust
                                               ↓
                                         XML parse (ls)
                                         File I/O (cp)
                                               ↓
                                         stdout / stderr
```

### 3.2 Module structure

```
src/
├── main.rs        # Entry point, clap CLI definition, command dispatch
├── client.rs      # HTTP client wrapping reqwest (get/put/delete)
├── error.rs       # Unified error type (Http, Io, Xml, Api)
└── cmd/
    ├── mod.rs     # Module declarations
    ├── s3.rs      # S3 subcommands, XML types, path splitting
    └── status.rs  # Health check command
```

---

## 4. Key design decisions

### 4.1 Clap derive for argument parsing

Clap's derive API provides compile-time verified CLI definitions with minimal boilerplate. The `env` feature enables `AWRUST_ENDPOINT` environment variable fallback without custom code.

### 4.2 Thin HTTP client

`client.rs` wraps reqwest with three methods: `get`, `put`, `delete`. Error checking is centralized in a single `check` method that converts non-2xx responses into `Error::Api`. Callers receive the raw `reqwest::Response` for flexible body consumption (text for XML, bytes for downloads).

### 4.3 Direction detection for `cp`

Upload vs download is determined by checking if the source path exists on the local filesystem. If it exists, it's an upload; otherwise, the source is treated as an S3 path and downloaded to the destination.

### 4.4 No `s3://` prefix

S3 paths use bare `bucket/key` format. This is intentional — awrust-cli is not the AWS CLI and doesn't pretend to be. The simpler format reduces typing.

### 4.5 XML handling

`quick-xml` with serde for both serialization (`CreateBucketConfiguration`) and deserialization (`ListAllMyBucketsResult`, `ListBucketResult`). XML types are private to `cmd/s3.rs` — no other module needs to know about S3 wire format.

### 4.6 Unified error type

A single `Error` enum in `error.rs` covers all failure modes: HTTP transport, I/O, XML parsing, and API errors. The `?` operator propagates errors naturally. All errors exit with code 1 and print to stderr.

---

## 5. Command routing

```
awr [--endpoint URL] <command>
│
├── status          → GET /health → print body
│
└── s3 <action>
    ├── mb <bucket> [--region R]  → PUT /<bucket> [+ LocationConstraint XML]
    ├── rb <bucket>               → DELETE /<bucket>
    ├── ls                        → GET / → parse ListAllMyBucketsResult
    ├── ls <bucket>[/prefix]      → GET /<bucket>?list-type=2[&prefix=P]
    │                               → parse ListBucketResult
    ├── cp <local> <bucket/key>   → read file → PUT /<bucket>/<key>
    ├── cp <bucket/key> <local>   → GET /<bucket>/<key> → write file
    └── rm <bucket/key>           → DELETE /<bucket>/<key>
```

---

## 6. Extensibility

New awrust services follow the same pattern:

1. Add a subcommand variant to `Command` in `main.rs`
2. Create `cmd/<service>.rs` with a `Subcommand` enum and `execute` function
3. Reuse the existing `Client` for HTTP calls
4. Add BDD feature files in `tests/integration/features/`

The `Client` is service-agnostic — it knows nothing about S3, only HTTP.

---

## 7. Testing strategy

### 7.1 Integration tests

BDD tests via Python Behave in `tests/integration/`:

* 3 feature files: status, bucket operations, object operations
* 13 scenarios covering all commands including error paths
* Tests run against a real `ghcr.io/awrust/awrust` Docker container
* Server lifecycle managed by `environment.py` (random port, auto-teardown)
* CLI binary built via `cargo build` and invoked as a subprocess

### 7.2 CI

GitHub Actions:

* **build** — fmt, clippy, release build
* **integration** — behave against awrust Docker container

---

## 8. Dependencies

| Crate | Purpose |
|-------|---------|
| `clap` | CLI argument parsing (derive + env) |
| `reqwest` | HTTP client |
| `quick-xml` | S3 XML serialization/deserialization |
| `serde` | Serialization framework |
| `thiserror` | Error type derive |
| `tokio` | Async runtime |
