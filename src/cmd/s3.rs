use clap::Subcommand;
use quick_xml::de::from_str;
use quick_xml::se::to_string;
use serde::{Deserialize, Serialize};

use crate::client::Client;
use crate::error::{Error, Result};

#[derive(Subcommand)]
pub enum S3Command {
    Mb {
        bucket: String,
        #[arg(long, default_value = "us-east-1")]
        region: String,
    },
    Rb {
        bucket: String,
        #[arg(long)]
        force: bool,
    },
    Ls {
        path: Option<String>,
    },
    Cp {
        source: String,
        dest: String,
    },
    Rm {
        path: String,
        #[arg(long)]
        recursive: bool,
    },
}

pub async fn execute(client: &Client, cmd: S3Command) -> Result<()> {
    match cmd {
        S3Command::Mb { bucket, region } => make_bucket(client, &bucket, &region).await,
        S3Command::Rb { bucket, force } if force => force_remove_bucket(client, &bucket).await,
        S3Command::Rb { bucket, .. } => remove_bucket(client, &bucket).await,
        S3Command::Ls { path } => list(client, path.as_deref()).await,
        S3Command::Cp { source, dest } => copy(client, &source, &dest).await,
        S3Command::Rm { path, recursive } if recursive => {
            remove_objects_recursive(client, &path).await
        }
        S3Command::Rm { path, .. } => remove_object(client, &path).await,
    }
}

fn split_path(path: &str) -> (&str, &str) {
    path.split_once('/').unwrap_or((path, ""))
}

async fn make_bucket(client: &Client, bucket: &str, region: &str) -> Result<()> {
    let body = if region == "us-east-1" {
        Vec::new()
    } else {
        to_string(&CreateBucketConfig {
            location_constraint: region.to_string(),
        })
        .map_err(|e| Error::Xml(e.to_string()))?
        .into_bytes()
    };
    client.put(&format!("/{bucket}"), body).await?;
    println!("Bucket created: {bucket}");
    Ok(())
}

async fn remove_bucket(client: &Client, bucket: &str) -> Result<()> {
    client.delete(&format!("/{bucket}")).await?;
    println!("Bucket removed: {bucket}");
    Ok(())
}

async fn list(client: &Client, path: Option<&str>) -> Result<()> {
    match path {
        None => list_buckets(client).await,
        Some(p) => {
            let (bucket, prefix) = split_path(p);
            list_objects(client, bucket, prefix).await
        }
    }
}

async fn list_buckets(client: &Client) -> Result<()> {
    let resp = client.get("/").await?;
    let body = resp.text().await?;
    let result: ListAllMyBucketsResult = from_str(&body).map_err(|e| Error::Xml(e.to_string()))?;
    for b in &result.buckets.items {
        println!("{}\t{}", b.creation_date, b.name);
    }
    Ok(())
}

async fn list_objects(client: &Client, bucket: &str, prefix: &str) -> Result<()> {
    let path = if prefix.is_empty() {
        format!("/{bucket}?list-type=2")
    } else {
        format!("/{bucket}?list-type=2&prefix={prefix}")
    };
    let resp = client.get(&path).await?;
    let body = resp.text().await?;
    let result: ListBucketResult = from_str(&body).map_err(|e| Error::Xml(e.to_string()))?;
    for obj in &result.contents {
        println!("{}\t{}\t{}", obj.last_modified, obj.size, obj.key);
    }
    Ok(())
}

async fn copy(client: &Client, source: &str, dest: &str) -> Result<()> {
    if std::path::Path::new(source).exists() {
        upload(client, source, dest).await
    } else {
        download(client, source, dest).await
    }
}

async fn upload(client: &Client, local: &str, remote: &str) -> Result<()> {
    let data = tokio::fs::read(local).await?;
    let (bucket, key) = split_path(remote);
    client.put(&format!("/{bucket}/{key}"), data).await?;
    println!("Uploaded: {local} -> {bucket}/{key}");
    Ok(())
}

async fn download(client: &Client, remote: &str, local: &str) -> Result<()> {
    let (bucket, key) = split_path(remote);
    let resp = client.get(&format!("/{bucket}/{key}")).await?;
    let bytes = resp.bytes().await?;
    tokio::fs::write(local, bytes).await?;
    println!("Downloaded: {bucket}/{key} -> {local}");
    Ok(())
}

async fn fetch_object_keys(client: &Client, bucket: &str) -> Result<Vec<String>> {
    let resp = client.get(&format!("/{bucket}?list-type=2")).await?;
    let body = resp.text().await?;
    let result: ListBucketResult = from_str(&body).map_err(|e| Error::Xml(e.to_string()))?;
    Ok(result.contents.into_iter().map(|o| o.key).collect())
}

async fn remove_object(client: &Client, path: &str) -> Result<()> {
    let (bucket, key) = split_path(path);
    client.delete(&format!("/{bucket}/{key}")).await?;
    println!("Deleted: {bucket}/{key}");
    Ok(())
}

async fn remove_objects_recursive(client: &Client, path: &str) -> Result<()> {
    let (bucket, _) = split_path(path);
    for key in fetch_object_keys(client, bucket).await? {
        client.delete(&format!("/{bucket}/{key}")).await?;
        println!("Deleted: {bucket}/{key}");
    }
    Ok(())
}

async fn force_remove_bucket(client: &Client, bucket: &str) -> Result<()> {
    for key in fetch_object_keys(client, bucket).await? {
        client.delete(&format!("/{bucket}/{key}")).await?;
    }
    remove_bucket(client, bucket).await
}

#[derive(Serialize)]
#[serde(rename = "CreateBucketConfiguration")]
struct CreateBucketConfig {
    #[serde(rename = "LocationConstraint")]
    location_constraint: String,
}

#[derive(Deserialize, Default)]
#[serde(rename = "ListAllMyBucketsResult")]
struct ListAllMyBucketsResult {
    #[serde(rename = "Buckets", default)]
    buckets: BucketList,
}

#[derive(Deserialize, Default)]
struct BucketList {
    #[serde(rename = "Bucket", default)]
    items: Vec<BucketInfo>,
}

#[derive(Deserialize)]
struct BucketInfo {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "CreationDate")]
    creation_date: String,
}

#[derive(Deserialize, Default)]
#[serde(rename = "ListBucketResult")]
struct ListBucketResult {
    #[serde(rename = "Contents", default)]
    contents: Vec<ObjectInfo>,
}

#[derive(Deserialize)]
struct ObjectInfo {
    #[serde(rename = "Key")]
    key: String,
    #[serde(rename = "Size")]
    size: u64,
    #[serde(rename = "LastModified")]
    last_modified: String,
}
