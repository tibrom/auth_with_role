use reqwest::{Method, Response};
use std::time::{Duration, SystemTime};
use uuid::Uuid;


use super::interface::HttpClientInterface;

#[derive(Debug, Clone)]
pub struct HttpClient {
    uri: String,
    headers: Vec<(String, String)>,
}

impl HttpClient {
    pub fn new(uri: String) -> Self {
        Self { uri, headers: Vec::new() }
    }

    pub fn add_header(mut self, header: (String, String)) -> Self {
        self.headers.push(header);
        self
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
        &self,
        method: Method,
        uri: &str,
        body: Option<String>,
    ) -> Result<Response, reqwest::Error> {
        let max_retry = 5;
        let retry_timeout = Duration::from_millis(1000);
        let mut retry_count = 0;
        let elapsed = SystemTime::now();
        loop {
            let trace_id = Uuid::new_v4().to_string();
            match self.request_inner(
                method.clone(),
                uri,
                body.clone(),
                Some(trace_id.clone()),
            )
            .await
            {
                Ok(response) => {
                    //info!("Resolved traceId: {} duration: {}ms",trace_id,elapsed.elapsed().unwrap().as_millis());
                    break Ok(response);
                }
                Err(e) => {
                    retry_count += 1;
                    if retry_count > max_retry {
                        break Err(e);
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


impl HttpClientInterface for HttpClient  {
    type Error = reqwest::Error;
    async fn post(&self, body: String) -> Result<String, Self::Error> {
        let resp = self.request(
            Method::POST,
            self.uri.clone().as_str(),
            Some(body.clone()),
        )
        .await?;
        let body_bytes = resp.bytes().await?;
        let body = String::from_utf8(body_bytes.to_vec()).unwrap();
        Ok(body)
    }
}

#[cfg(test)]
mod tests {
    
}