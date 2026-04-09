Feature: S3 bucket operations

  Scenario: Create a bucket
    When I create bucket "bkt-create"
    Then the command succeeds
    And the output contains "Bucket created: bkt-create"

  Scenario: Create a bucket with custom region
    When I create regional bucket "bkt-region" in "eu-west-1"
    Then the command succeeds
    And the output contains "Bucket created: bkt-region"

  Scenario: Remove a bucket
    Given bucket "bkt-remove" exists
    When I remove bucket "bkt-remove"
    Then the command succeeds
    And the output contains "Bucket removed: bkt-remove"

  Scenario: Remove nonexistent bucket fails
    When I remove bucket "bkt-ghost"
    Then the command fails

  Scenario: List buckets shows created buckets
    Given bucket "bkt-list-alpha" exists
    And bucket "bkt-list-bravo" exists
    When I list buckets
    Then the command succeeds
    And the output contains "bkt-list-alpha"
    And the output contains "bkt-list-bravo"
