use crate::error::{Error, Result};

pub struct Client {
    http: reqwest::Client,
    endpoint: String,
}

impl Client {
    pub fn new(endpoint: &str) -> Self {
        Self {
            http: reqwest::Client::new(),
            endpoint: endpoint.trim_end_matches('/').to_string(),
        }
    }

    fn url(&self, path: &str) -> String {
        format!("{}{}", self.endpoint, path)
    }

    pub async fn get(&self, path: &str) -> Result<reqwest::Response> {
        let resp = self.http.get(self.url(path)).send().await?;
        Self::check(resp).await
    }

    pub async fn put(&self, path: &str, body: Vec<u8>) -> Result<reqwest::Response> {
        let resp = self.http.put(self.url(path)).body(body).send().await?;
        Self::check(resp).await
    }

    pub async fn delete(&self, path: &str) -> Result<reqwest::Response> {
        let resp = self.http.delete(self.url(path)).send().await?;
        Self::check(resp).await
    }

    async fn check(resp: reqwest::Response) -> Result<reqwest::Response> {
        if resp.status().is_success() {
            return Ok(resp);
        }
        let status = resp.status().as_u16();
        let body = resp.text().await.unwrap_or_default();
        Err(Error::Api(status, body))
    }
}
