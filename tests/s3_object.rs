use assert_cmd::Command;
use predicates::str::contains;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn given_local_file_when_cp_to_s3_then_uploads() {
    let server = MockServer::start().await;
    Mock::given(method("PUT"))
        .and(path("/my-bucket/my-key.txt"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&server)
        .await;

    let dir = tempfile::tempdir().unwrap();
    let source = dir.path().join("upload.txt");
    std::fs::write(&source, "hello awrust").unwrap();

    Command::cargo_bin("awr")
        .unwrap()
        .args([
            "--endpoint",
            &server.uri(),
            "s3",
            "cp",
            source.to_str().unwrap(),
            "my-bucket/my-key.txt",
        ])
        .assert()
        .success()
        .stdout(contains("Uploaded"));

    let requests = server.received_requests().await.unwrap();
    assert_eq!(requests[0].body, b"hello awrust");
}

#[tokio::test]
async fn given_s3_object_when_cp_to_local_then_downloads() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/my-bucket/remote.txt"))
        .respond_with(ResponseTemplate::new(200).set_body_string("downloaded content"))
        .expect(1)
        .mount(&server)
        .await;

    let dir = tempfile::tempdir().unwrap();
    let dest = dir.path().join("local.txt");

    Command::cargo_bin("awr")
        .unwrap()
        .args([
            "--endpoint",
            &server.uri(),
            "s3",
            "cp",
            "my-bucket/remote.txt",
            dest.to_str().unwrap(),
        ])
        .assert()
        .success()
        .stdout(contains("Downloaded"));

    assert_eq!(std::fs::read_to_string(&dest).unwrap(), "downloaded content");
}

#[tokio::test]
async fn given_s3_object_when_rm_then_deletes() {
    let server = MockServer::start().await;
    Mock::given(method("DELETE"))
        .and(path("/my-bucket/trash.txt"))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&server)
        .await;

    Command::cargo_bin("awr")
        .unwrap()
        .args([
            "--endpoint",
            &server.uri(),
            "s3",
            "rm",
            "my-bucket/trash.txt",
        ])
        .assert()
        .success()
        .stdout(contains("Deleted: my-bucket/trash.txt"));
}

#[tokio::test]
async fn given_nonexistent_object_when_rm_then_exits_with_error() {
    let server = MockServer::start().await;
    Mock::given(method("DELETE"))
        .and(path("/my-bucket/ghost.txt"))
        .respond_with(ResponseTemplate::new(404).set_body_string("NoSuchKey"))
        .mount(&server)
        .await;

    Command::cargo_bin("awr")
        .unwrap()
        .args([
            "--endpoint",
            &server.uri(),
            "s3",
            "rm",
            "my-bucket/ghost.txt",
        ])
        .assert()
        .failure()
        .stderr(contains("404"));
}

#[tokio::test]
async fn given_binary_file_when_cp_roundtrip_then_preserves_content() {
    let server = MockServer::start().await;
    let binary_data: Vec<u8> = (0..=255).collect();

    Mock::given(method("PUT"))
        .and(path("/bin-bucket/data.bin"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&server)
        .await;

    let dir = tempfile::tempdir().unwrap();
    let source = dir.path().join("data.bin");
    std::fs::write(&source, &binary_data).unwrap();

    Command::cargo_bin("awr")
        .unwrap()
        .args([
            "--endpoint",
            &server.uri(),
            "s3",
            "cp",
            source.to_str().unwrap(),
            "bin-bucket/data.bin",
        ])
        .assert()
        .success();

    let requests = server.received_requests().await.unwrap();
    assert_eq!(requests[0].body, binary_data);
}
