import os
import shutil
import socket
import subprocess
import tempfile
import time
import urllib.request

AWRUST_IMAGE = os.environ.get("AWRUST_IMAGE", "ghcr.io/awrust/awrust:0.2.3")
PROJECT_ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))


def _free_port():
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        s.bind(("", 0))
        return s.getsockname()[1]


def _wait_for_health(url, timeout=15):
    deadline = time.time() + timeout
    while time.time() < deadline:
        try:
            resp = urllib.request.urlopen(f"{url}/health", timeout=2)
            if resp.status == 200:
                return
        except Exception:
            pass
        time.sleep(0.2)
    raise RuntimeError(f"awrust not healthy after {timeout}s")


def before_all(context):
    result = subprocess.run(
        ["cargo", "build"],
        cwd=PROJECT_ROOT,
        capture_output=True,
        text=True,
    )
    assert result.returncode == 0, f"cargo build failed:\n{result.stderr}"
    awr_bin = os.path.join(PROJECT_ROOT, "target", "debug", "awr")

    port = _free_port()
    container = f"awrust-cli-test-{port}"
    subprocess.run(
        [
            "docker", "run", "--rm", "-d",
            "--name", container,
            "-p", f"{port}:4566",
            "-e", "AWRUST_SERVICES=s3",
            AWRUST_IMAGE,
        ],
        check=True,
        capture_output=True,
    )
    context.container = container
    endpoint = f"http://localhost:{port}"
    _wait_for_health(endpoint)

    def awr(*args):
        cmd = [awr_bin, "--endpoint", endpoint] + list(args)
        r = subprocess.run(cmd, capture_output=True, text=True)
        context.result = r
        return r

    context.awr = awr
    context.endpoint = endpoint


def after_all(context):
    name = getattr(context, "container", None)
    if name:
        subprocess.run(["docker", "stop", name], capture_output=True)


def before_scenario(context, scenario):
    context.tmpdir = tempfile.mkdtemp(prefix="awr-test-")


def after_scenario(context, scenario):
    d = getattr(context, "tmpdir", None)
    if d:
        shutil.rmtree(d, ignore_errors=True)
