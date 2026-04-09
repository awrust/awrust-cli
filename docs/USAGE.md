# awrust-cli — Usage Guide

## Installation

From source:

```bash
cargo install --path .
```

The binary is named `awr`.

---

## Configuration

| Variable | CLI flag | Default | Description |
|----------|----------|---------|-------------|
| `AWRUST_ENDPOINT` | `--endpoint` | `http://localhost:4566` | awrust service URL |

The `--endpoint` flag takes precedence over the environment variable.

---

## Commands

### `awr status`

Check service health.

```bash
$ awr status
{"status":"ok","services":{"s3":{"status":"ok"}}}
```

---

### `awr s3 mb <bucket> [--region <region>]`

Create a bucket. Sends `PUT /<bucket>` with `LocationConstraint` XML body when region is not `us-east-1`.

```bash
$ awr s3 mb my-bucket
Bucket created: my-bucket

$ awr s3 mb eu-bucket --region eu-west-1
Bucket created: eu-bucket
```

---

### `awr s3 rb <bucket>`

Remove a bucket. Sends `DELETE /<bucket>`. Bucket must be empty.

```bash
$ awr s3 rb my-bucket
Bucket removed: my-bucket
```

---

### `awr s3 ls [bucket[/prefix]]`

List buckets or objects.

**List all buckets:**

```bash
$ awr s3 ls
2025-01-01T00:00:00.000Z	my-bucket
2025-06-15T12:00:00.000Z	other-bucket
```

**List objects in a bucket:**

```bash
$ awr s3 ls my-bucket
2025-03-01T10:00:00.000Z	1024	file.txt
2025-03-02T10:00:00.000Z	2048	photo.jpg
```

**List objects with prefix filter:**

```bash
$ awr s3 ls my-bucket/logs/
2025-04-01T08:00:00.000Z	512	logs/app.log
2025-04-01T08:00:00.000Z	256	logs/err.log
```

---

### `awr s3 cp <source> <dest>`

Copy files between local filesystem and S3.

S3 paths use the format `<bucket>/<key>` — no `s3://` prefix. Direction is determined automatically: if the source exists as a local file, it uploads; otherwise it downloads.

**Upload:**

```bash
$ awr s3 cp report.pdf my-bucket/reports/q1.pdf
Uploaded: report.pdf -> my-bucket/reports/q1.pdf
```

**Download:**

```bash
$ awr s3 cp my-bucket/reports/q1.pdf local-copy.pdf
Downloaded: my-bucket/reports/q1.pdf -> local-copy.pdf
```

---

### `awr s3 rm <bucket>/<key>`

Delete an object. Sends `DELETE /<bucket>/<key>`.

```bash
$ awr s3 rm my-bucket/old-file.txt
Deleted: my-bucket/old-file.txt
```

---

## Exit codes

| Code | Meaning |
|------|---------|
| `0` | Success |
| `1` | Failure (error details on stderr) |

---

## Output conventions

* Normal output goes to **stdout**.
* Errors go to **stderr**.
* Output is human-readable, not JSON.

---

## Authentication

awrust accepts unsigned requests. No SigV4 signing, no credentials needed.

---

## Connecting to a remote endpoint

```bash
awr --endpoint http://awrust.dev.local:4566 s3 ls
```

Or via environment variable:

```bash
export AWRUST_ENDPOINT=http://awrust.dev.local:4566
awr s3 ls
```

---

## Testing

### BDD integration tests (Behave)

```bash
pip install -r tests/integration/requirements.txt
cd tests/integration && behave
```

Tests spin up a real awrust Docker container, build the `awr` binary, and run 13 BDD scenarios covering all commands.
