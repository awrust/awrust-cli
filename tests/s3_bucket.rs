use assert_cmd::Command;
use predicates::str::contains;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn given_service_when_mb_then_creates_bucket() {
    let server = MockServer::start().await;
    Mock::given(method("PUT"))
        .and(path("/test-bucket"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&server)
        .await;

    Command::cargo_bin("awr")
        .unwrap()
        .args(["--endpoint", &server.uri(), "s3", "mb", "test-bucket"])
        .assert()
        .success()
        .stdout(contains("Bucket created: test-bucket"));
}

#[tokio::test]
async fn given_service_when_mb_with_region_then_sends_location_constraint() {
    let server = MockServer::start().await;
    Mock::given(method("PUT"))
        .and(path("/regional-bucket"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&server)
        .await;

    Command::cargo_bin("awr")
        .unwrap()
        .args([
            "--endpoint",
            &server.uri(),
            "s3",
            "mb",
            "regional-bucket",
            "--region",
            "eu-west-1",
        ])
        .assert()
        .success()
        .stdout(contains("Bucket created: regional-bucket"));

    let requests = server.received_requests().await.unwrap();
    let body = String::from_utf8(requests[0].body.clone()).unwrap();
    assert!(body.contains("LocationConstraint"));
    assert!(body.contains("eu-west-1"));
}

#[tokio::test]
async fn given_service_when_mb_default_region_then_sends_empty_body() {
    let server = MockServer::start().await;
    Mock::given(method("PUT"))
        .and(path("/default-bucket"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&server)
        .await;

    Command::cargo_bin("awr")
        .unwrap()
        .args(["--endpoint", &server.uri(), "s3", "mb", "default-bucket"])
        .assert()
        .success();

    let requests = server.received_requests().await.unwrap();
    assert!(requests[0].body.is_empty());
}

#[tokio::test]
async fn given_existing_bucket_when_rb_then_removes_bucket() {
    let server = MockServer::start().await;
    Mock::given(method("DELETE"))
        .and(path("/old-bucket"))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&server)
        .await;

    Command::cargo_bin("awr")
        .unwrap()
        .args(["--endpoint", &server.uri(), "s3", "rb", "old-bucket"])
        .assert()
        .success()
        .stdout(contains("Bucket removed: old-bucket"));
}

#[tokio::test]
async fn given_nonexistent_bucket_when_rb_then_exits_with_error() {
    let server = MockServer::start().await;
    Mock::given(method("DELETE"))
        .and(path("/ghost-bucket"))
        .respond_with(ResponseTemplate::new(404).set_body_string("NoSuchBucket"))
        .mount(&server)
        .await;

    Command::cargo_bin("awr")
        .unwrap()
        .args(["--endpoint", &server.uri(), "s3", "rb", "ghost-bucket"])
        .assert()
        .failure()
        .stderr(contains("404"));
}

#[tokio::test]
async fn given_buckets_exist_when_ls_then_lists_all_buckets() {
    let server = MockServer::start().await;
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<ListAllMyBucketsResult xmlns="http://s3.amazonaws.com/doc/2006-03-01/">
  <Buckets>
    <Bucket>
      <Name>alpha</Name>
      <CreationDate>2025-01-01T00:00:00.000Z</CreationDate>
    </Bucket>
    <Bucket>
      <Name>bravo</Name>
      <CreationDate>2025-06-15T12:00:00.000Z</CreationDate>
    </Bucket>
  </Buckets>
</ListAllMyBucketsResult>"#;

    Mock::given(method("GET"))
        .and(path("/"))
        .respond_with(ResponseTemplate::new(200).set_body_string(xml))
        .mount(&server)
        .await;

    Command::cargo_bin("awr")
        .unwrap()
        .args(["--endpoint", &server.uri(), "s3", "ls"])
        .assert()
        .success()
        .stdout(contains("alpha"))
        .stdout(contains("bravo"));
}

#[tokio::test]
async fn given_objects_exist_when_ls_bucket_then_lists_objects() {
    let server = MockServer::start().await;
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<ListBucketResult xmlns="http://s3.amazonaws.com/doc/2006-03-01/">
  <Name>data</Name>
  <Contents>
    <Key>file1.txt</Key>
    <Size>1024</Size>
    <LastModified>2025-03-01T10:00:00.000Z</LastModified>
  </Contents>
  <Contents>
    <Key>file2.txt</Key>
    <Size>2048</Size>
    <LastModified>2025-03-02T10:00:00.000Z</LastModified>
  </Contents>
</ListBucketResult>"#;

    Mock::given(method("GET"))
        .and(path("/data"))
        .respond_with(ResponseTemplate::new(200).set_body_string(xml))
        .mount(&server)
        .await;

    Command::cargo_bin("awr")
        .unwrap()
        .args(["--endpoint", &server.uri(), "s3", "ls", "data"])
        .assert()
        .success()
        .stdout(contains("file1.txt"))
        .stdout(contains("1024"))
        .stdout(contains("file2.txt"))
        .stdout(contains("2048"));
}

#[tokio::test]
async fn given_objects_with_prefix_when_ls_with_prefix_then_filters() {
    let server = MockServer::start().await;
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<ListBucketResult xmlns="http://s3.amazonaws.com/doc/2006-03-01/">
  <Name>data</Name>
  <Contents>
    <Key>logs/app.log</Key>
    <Size>512</Size>
    <LastModified>2025-04-01T08:00:00.000Z</LastModified>
  </Contents>
</ListBucketResult>"#;

    Mock::given(method("GET"))
        .and(path("/data"))
        .respond_with(ResponseTemplate::new(200).set_body_string(xml))
        .mount(&server)
        .await;

    Command::cargo_bin("awr")
        .unwrap()
        .args(["--endpoint", &server.uri(), "s3", "ls", "data/logs/"])
        .assert()
        .success()
        .stdout(contains("logs/app.log"));

    let requests = server.received_requests().await.unwrap();
    let uri = requests[0].url.to_string();
    assert!(uri.contains("list-type=2"));
    assert!(uri.contains("prefix=logs/"));
}
