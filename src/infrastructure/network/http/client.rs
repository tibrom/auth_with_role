use reqwest::{Method, Response};
use std::time::{Duration, SystemTime};
use uuid::Uuid;

use super::interface::HttpClientInterface;

static MAX_RETRY_DEFAULT: u64 = 5;
static RETRY_DURATION_MS_DEFAULT: u64 = 1000;

#[derive(Debug, Clone)]
pub struct HttpClient {
    max_retry: u64,
    retry_duration_ms: u64,
    finish_retry_count: u64,
    uri: String,
    headers: Vec<(String, String)>,
}

impl HttpClient {
    pub fn new(uri: String) -> Self {
        Self {
            max_retry: MAX_RETRY_DEFAULT,
            retry_duration_ms: RETRY_DURATION_MS_DEFAULT,
            finish_retry_count: 0,
            uri,
            headers: Vec::new(),
        }
    }

    pub fn set_max_retry(&mut self, retry: u64) {
        self.max_retry = retry
    }

    pub fn set_retry_duration_ms(&mut self, duration: u64) {
        self.retry_duration_ms = duration
    }

    pub fn finish_retry_count(&self) -> u64 {
        self.finish_retry_count
    }

    pub fn add_header(mut self, header: (String, String)) -> Self {
        self.headers.push(header);
        self
    }

    pub fn get_header(&self, key: &str) -> Option<String> {
        for (k, v) in &self.headers {
            if k == key {
                return Some(v.clone());
            }
        }
        None
    }

    async fn request_inner(
        &self,
        method: Method,
        uri: &str,
        body: Option<String>,
        trace_id: Option<String>,
    ) -> Result<Response, reqwest::Error> {
        let client = reqwest::Client::builder()
            .danger_accept_invalid_certs(true)
            .timeout(Duration::from_secs(15))
            .build()
            .unwrap();
        let mut rb = client.request(method.clone(), uri);
        if let Some(_body) = body.clone() {
            rb = rb.body(_body);
        }
        if !self.headers.is_empty() {
            for (k, v) in &self.headers {
                rb = rb.header(k, v);
            }
        }
        // Add trace header
        let uuid = trace_id.unwrap_or(Uuid::new_v4().to_string());
        rb = rb.header("X-API-TraceId".to_string(), uuid.clone());

        match rb.send().await {
            Ok(r) => r.error_for_status(),
            Err(e) => {
                tracing::error!(
                    "Request traceId: {} on {} by {} with body: '{:?}'",
                    uuid,
                    uri,
                    method.clone(),
                    body.clone()
                );
                Err(e)
            }
        }
    }

    async fn request(
        &mut self,
        method: Method,
        uri: &str,
        body: Option<String>,
    ) -> Result<Response, reqwest::Error> {
        let max_retry = self.max_retry;
        let retry_timeout = Duration::from_millis(self.retry_duration_ms);
        let mut retry_count = 0;
        let elapsed = SystemTime::now();
        loop {
            retry_count += 1;
            let trace_id = Uuid::new_v4().to_string();
            match self
                .request_inner(method.clone(), uri, body.clone(), Some(trace_id.clone()))
                .await
            {
                Ok(response) => {
                    self.finish_retry_count = retry_count;
                    return Ok(response);
                }
                Err(e) => {
                    if retry_count >= max_retry {
                        self.finish_retry_count = retry_count;
                        return Err(e);
                    }
                }
            }
            tracing::warn!(
                "Retry HTTP traceId: {} request {}/{} Elapsed: {}ms",
                trace_id,
                retry_count,
                max_retry,
                elapsed.elapsed().unwrap().as_millis()
            );
            tokio::time::sleep(retry_timeout).await;
        }
    }
}

impl HttpClientInterface for HttpClient {
    type Error = reqwest::Error;
    async fn post(&mut self, body: String) -> Result<String, Self::Error> {
        let resp = self
            .request(Method::POST, self.uri.clone().as_str(), Some(body.clone()))
            .await?;
        let body_bytes = resp.bytes().await?;
        let body = String::from_utf8(body_bytes.to_vec()).unwrap();
        Ok(body)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::{Mock, MockServer, ResponseTemplate};
    use wiremock::matchers::{method, path};
       

    #[tokio::test]
    async fn test_post_retry_on_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
        .and(path("/test"))
        .respond_with(ResponseTemplate::new(500))
        .up_to_n_times(4)
        .mount(&mock_server)
        .await;

        Mock::given(method("POST"))
            .and(path("/test"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        let mut client = HttpClient::new(format!("{}/test", &mock_server.uri()));
        client.set_retry_duration_ms(10);
        let result = client.post("will-fail".to_string()).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_post_retry_on_error() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
        .and(path("/test"))
        .respond_with(ResponseTemplate::new(500))
        .up_to_n_times(10)
        .mount(&mock_server)
        .await;

        let mut client = HttpClient::new(format!("{}/test", &mock_server.uri()));
        client.set_retry_duration_ms(10);
        let result = client.post("will-fail".to_string()).await;

        assert!(result.is_err());
        assert_eq!(MAX_RETRY_DEFAULT, client.finish_retry_count())
    }

}
