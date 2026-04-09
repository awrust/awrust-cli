Feature: S3 object operations

  Background:
    Given bucket "obj-ops" exists

  Scenario: Upload a file
    Given a local file "up.txt" with content "hello awrust"
    When I upload "up.txt" to "obj-ops/up.txt"
    Then the command succeeds
    And the output contains "Uploaded"

  Scenario: Download a file
    Given a local file "src.txt" with content "download me"
    And "src.txt" is uploaded to "obj-ops/dl-test.txt"
    When I download "obj-ops/dl-test.txt" to "dst.txt"
    Then the command succeeds
    And the output contains "Downloaded"
    And local file "dst.txt" contains "download me"

  Scenario: List objects in bucket
    Given a local file "item.txt" with content "data"
    And "item.txt" is uploaded to "obj-ops/ls-item.txt"
    When I list "obj-ops"
    Then the command succeeds
    And the output contains "ls-item.txt"

  Scenario: List objects with prefix filter
    Given a local file "a.txt" with content "a"
    And a local file "b.txt" with content "b"
    And "a.txt" is uploaded to "obj-ops/logs/a.txt"
    And "b.txt" is uploaded to "obj-ops/data/b.txt"
    When I list "obj-ops/logs/"
    Then the command succeeds
    And the output contains "a.txt"
    And the output does not contain "b.txt"

  Scenario: Delete an object
    Given a local file "trash.txt" with content "gone"
    And "trash.txt" is uploaded to "obj-ops/rm-test.txt"
    When I delete "obj-ops/rm-test.txt"
    Then the command succeeds
    And the output contains "Deleted"

  Scenario: Binary data roundtrip preserves bytes
    Given a local binary file "data.bin" with bytes 0 to 255
    And "data.bin" is uploaded to "obj-ops/bin-rt.bin"
    When I download "obj-ops/bin-rt.bin" to "out.bin"
    Then the command succeeds
    And local file "out.bin" is identical to "data.bin"

  Scenario: Full object lifecycle
    Given a local file "life.txt" with content "create read delete"
    When I upload "life.txt" to "obj-ops/life.txt"
    Then the command succeeds
    When I list "obj-ops"
    Then the output contains "life.txt"
    When I download "obj-ops/life.txt" to "verify.txt"
    Then local file "verify.txt" contains "create read delete"
    When I delete "obj-ops/life.txt"
    Then the command succeeds
    When I list "obj-ops"
    Then the output does not contain "life.txt"
