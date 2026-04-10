# awrust-cli

[![CI](https://github.com/awrust/awrust-cli/actions/workflows/ci.yml/badge.svg)](https://github.com/awrust/awrust-cli/actions/workflows/ci.yml)

Minimal, ergonomic CLI for interacting with [awrust](https://github.com/awrust/awrust) — a local AWS service emulator. Binary name: `awr`.

Status: **v0.1 — S3 support.**

## Installation

### Homebrew (macOS / Linux)

```bash
brew install awrust/tap/awrust-cli
```

### Shell (macOS / Linux)

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/awrust/awrust-cli/releases/latest/download/awrust-cli-installer.sh | sh
```

### PowerShell (Windows)

```powershell
powershell -ExecutionPolicy Bypass -c "irm https://github.com/awrust/awrust-cli/releases/latest/download/awrust-cli-installer.ps1 | iex"
```

### Cargo (from source)

```bash
cargo install --git https://github.com/awrust/awrust-cli
```

### GitHub Releases

Pre-built binaries for macOS, Linux, and Windows are available on the [Releases](https://github.com/awrust/awrust-cli/releases) page.

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
| [ADRs](docs/adr) | Architecture Decision Records |

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
