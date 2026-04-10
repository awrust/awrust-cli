from behave import given, when


@given('bucket "{name}" exists')
def step_bucket_exists(context, name):
    context.awr("s3", "mb", name)


@when('I create bucket "{name}"')
def step_create_bucket(context, name):
    context.awr("s3", "mb", name)


@when('I create regional bucket "{name}" in "{region}"')
def step_create_regional_bucket(context, name, region):
    context.awr("s3", "mb", name, "--region", region)


@when('I remove bucket "{name}"')
def step_remove_bucket(context, name):
    context.awr("s3", "rb", name)


@when('I force remove bucket "{name}"')
def step_force_remove_bucket(context, name):
    context.awr("s3", "rb", name, "--force")


@when("I list buckets")
def step_list_buckets(context):
    context.awr("s3", "ls")
