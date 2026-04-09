# awrust-cli

[![CI](https://github.com/awrust/awrust-cli/actions/workflows/ci.yml/badge.svg)](https://github.com/awrust/awrust-cli/actions/workflows/ci.yml)

Minimal, ergonomic CLI for interacting with [awrust](https://github.com/awrust/awrust) — a local AWS service emulator. Binary name: `awr`.

Status: **v0.1 — S3 support.**

## Quick start

```bash
# Start awrust
docker run --rm -p 4566:4566 ghcr.io/awrust/awrust:0.2.3

# Use it
awr s3 mb my-bucket
awr s3 cp file.txt my-bucket/file.txt
awr s3 ls my-bucket
awr s3 cp my-bucket/file.txt downloaded.txt
awr s3 rm my-bucket/file.txt
awr s3 rb my-bucket
awr status
```

## Documentation

| Document | Description |
|----------|-------------|
| [Usage Guide](docs/USAGE.md) | Configuration, commands, examples |
| [Architecture](docs/ARCHITECTURE.md) | Design decisions, project structure, data flow |

## Development

```bash
cargo fmt --all --check
cargo clippy -- -D warnings
cargo build
```

BDD integration tests (requires Docker):

```bash
pip install -r tests/integration/requirements.txt
cd tests/integration && behave
```

See [AGENTS.md](AGENTS.md) for the full development workflow.

## License

MIT
