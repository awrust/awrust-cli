from behave import when, then


@when("I check service status")
def step_check_status(context):
    context.awr("status")


@then("the command succeeds")
def step_command_succeeds(context):
    assert context.result.returncode == 0, (
        f"Expected exit 0, got {context.result.returncode}\n"
        f"stderr: {context.result.stderr}"
    )


@then("the command fails")
def step_command_fails(context):
    assert context.result.returncode != 0, (
        f"Expected failure but got exit 0\nstdout: {context.result.stdout}"
    )


@then('the output contains "{text}"')
def step_output_contains(context, text):
    assert text in context.result.stdout, (
        f"Expected '{text}' in stdout:\n{context.result.stdout}"
    )


@then('the output does not contain "{text}"')
def step_output_not_contains(context, text):
    assert text not in context.result.stdout, (
        f"Did not expect '{text}' in stdout:\n{context.result.stdout}"
    )
