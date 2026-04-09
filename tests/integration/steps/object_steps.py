import os

from behave import given, when, then


def _path(context, name):
    return os.path.join(context.tmpdir, name)


@given('a local file "{name}" with content "{content}"')
def step_create_file(context, name, content):
    path = _path(context, name)
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w") as f:
        f.write(content)


@given('a local binary file "{name}" with bytes {lo:d} to {hi:d}')
def step_create_binary_file(context, name, lo, hi):
    path = _path(context, name)
    with open(path, "wb") as f:
        f.write(bytes(range(lo, hi + 1)))


@given('"{name}" is uploaded to "{remote}"')
def step_upload_given(context, name, remote):
    r = context.awr("s3", "cp", _path(context, name), remote)
    assert r.returncode == 0, f"Setup upload failed: {r.stderr}"


@when('I upload "{name}" to "{remote}"')
def step_upload(context, name, remote):
    context.awr("s3", "cp", _path(context, name), remote)


@when('I download "{remote}" to "{name}"')
def step_download(context, remote, name):
    context.awr("s3", "cp", remote, _path(context, name))


@when('I list "{path}"')
def step_list(context, path):
    context.awr("s3", "ls", path)


@when('I delete "{path}"')
def step_delete(context, path):
    context.awr("s3", "rm", path)


@then('local file "{name}" contains "{expected}"')
def step_file_contains(context, name, expected):
    with open(_path(context, name)) as f:
        actual = f.read()
    assert actual == expected, f"Expected '{expected}', got '{actual}'"


@then('local file "{a}" is identical to "{b}"')
def step_files_identical(context, a, b):
    with open(_path(context, a), "rb") as fa, open(_path(context, b), "rb") as fb:
        assert fa.read() == fb.read(), f"Files '{a}' and '{b}' differ"
