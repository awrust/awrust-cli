Feature: Service health check

  Scenario: Healthy service reports status
    When I check service status
    Then the command succeeds
    And the output contains "ok"
