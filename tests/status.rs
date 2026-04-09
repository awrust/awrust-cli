use assert_cmd::Command;
use predicates::str::contains;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn given_healthy_service_when_status_then_displays_response() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/health"))
        .respond_with(ResponseTemplate::new(200).set_body_string("all systems operational"))
        .mount(&server)
        .await;

    Command::cargo_bin("awr")
        .unwrap()
        .args(["--endpoint", &server.uri(), "status"])
        .assert()
        .success()
        .stdout(contains("all systems operational"));
}

#[tokio::test]
async fn given_unhealthy_service_when_status_then_exits_with_error() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/health"))
        .respond_with(ResponseTemplate::new(503).set_body_string("service degraded"))
        .mount(&server)
        .await;

    Command::cargo_bin("awr")
        .unwrap()
        .args(["--endpoint", &server.uri(), "status"])
        .assert()
        .failure()
        .stderr(contains("503"));
}
