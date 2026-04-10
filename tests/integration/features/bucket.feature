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

  Scenario: Force remove a bucket with objects
    Given bucket "bkt-force" exists
    And a local file "f1.txt" with content "alpha"
    And a local file "f2.txt" with content "bravo"
    And "f1.txt" is uploaded to "bkt-force/f1.txt"
    And "f2.txt" is uploaded to "bkt-force/f2.txt"
    When I force remove bucket "bkt-force"
    Then the command succeeds
    And the output contains "Bucket removed: bkt-force"
    When I list buckets
    Then the output does not contain "bkt-force"

  Scenario: Force remove an empty bucket
    Given bucket "bkt-force-empty" exists
    When I force remove bucket "bkt-force-empty"
    Then the command succeeds
    And the output contains "Bucket removed: bkt-force-empty"
