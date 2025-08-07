use crate::infrastructure::network::http::interface::HttpClientInterface;
use include_dir::Dir;

use std::collections::HashMap;
use std::str;
use tokio::sync::RwLock;
use std::sync::Arc;

#[derive(Clone)]
pub struct MockHttpClientResponse {
    data: Arc<RwLock<Option<String>>>,
}

impl MockHttpClientResponse {
    pub fn new() -> Self {
        Self {
            data: Arc::new(RwLock::new(None)),
        }
    }

    pub async fn set_data(&self, value: String) {
        let mut lock = self.data.write().await;
        *lock = Some(value);
    }

    pub async fn read_data(&self) -> Option<String> {
        self.data.read().await.clone()
    }
}

#[derive(Clone)]
pub enum ResponseMode {
    File,
    Memory,
}

#[derive(Clone)]
pub struct ResponseFile {
    dir: Dir<'static>,
    filename: &'static str,
}

impl ResponseFile {
    pub fn new(dir: Dir<'static>, filename: &'static str) -> Self {
        Self { dir, filename }
    }
}

#[derive(Clone)]
pub struct MockHttpClient {
    response_mode: ResponseMode,
    files_map: HashMap<String, ResponseFile>,
    memory: HashMap<String, serde_json::Value>,
    recorder: MockHttpClientResponse,
}

impl MockHttpClient {
    pub fn new(response_mode: ResponseMode) -> Self {
        Self {
            response_mode,
            files_map: HashMap::new(),
            memory: HashMap::new(),
            recorder: MockHttpClientResponse::new(),
        }
    }

    pub fn set_memory_response(&mut self, query_name: String, response: serde_json::Value) -> &mut Self {
        self.memory.insert(query_name, response);
        self
    }

    pub fn set_file_response(&mut self, query_name: String, response: ResponseFile) -> &mut Self {
        self.files_map.insert(query_name, response);
        self
    }

    pub fn recorder(&self) -> MockHttpClientResponse {
        self.recorder.clone()
    }

    fn generate_response(&self, body: String) -> Result<String, String> {
        let req_body: serde_json::Value =
            serde_json::from_str(&body).map_err(|_| "Invalid JSON format".to_owned())?;

        match self.response_mode {
            ResponseMode::File => self.get_response_from_file(req_body),
            ResponseMode::Memory => self.get_response_from_memory(req_body),
        }
    }

    fn get_response_from_memory(&self, body: serde_json::Value) -> Result<String, String> {
        let Some(name) = body.get("operationName").and_then(|v| v.as_str()) else {
            return Err("Missing or invalid 'operationName' field".to_owned());
        };

        let Some(response) = self.memory.get(name) else {
            return Err(format!("No mocked memory response found for operation '{}'", name));
        };

        Ok(response.to_string())
    }

    fn get_response_from_file(&self, body: serde_json::Value) -> Result<String, String> {
        let Some(name) = body.get("operationName").and_then(|v| v.as_str()) else {
            return Err("Missing or invalid 'operationName' field".to_owned());
        };

        let Some(response_file) = self.files_map.get(name) else {
            return Err(format!("No mocked file response found for operation '{}'", name));
        };

        let file = response_file
            .dir
            .get_file(response_file.filename)
            .ok_or_else(|| {
                format!(
                    "MockHttpClient error: file '{}' not found in the provided directory.",
                    response_file.filename
                )
            })?;

        let contents = file.contents_utf8().ok_or_else(|| {
            format!(
                "MockHttpClient error: file '{}' is not valid UTF-8.",
                response_file.filename
            )
        })?;

        Ok(contents.to_string())
    }
}

impl HttpClientInterface for MockHttpClient {
    type Error = String;

    async fn post(&mut self, body: String) -> Result<String, Self::Error> {
        self.recorder.set_data(body.clone()).await;
        self.generate_response(body)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use include_dir::{include_dir, Dir};

    static RESPONSE_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/tests/mock_gql/response/");

    #[tokio::test]
    async fn test_file_mode() {
        let query_name = "TEST".to_string();

        let response = ResponseFile::new(RESPONSE_DIR.clone(), "test.json");
        let mut client = MockHttpClient::new(ResponseMode::File);
            client.set_file_response(query_name.clone(), response);

        let body = serde_json::json!({
            "operationName": query_name,
            "query": "TEST",
            "variables": serde_json::Value::Null
        });

        let result = client.post(body.to_string()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_memory_mode() {
        let query_name = "TEST".to_string();
        let response = serde_json::json!({ "result": "success" });

        let mut client = MockHttpClient::new(ResponseMode::Memory);
        client.set_memory_response(query_name.clone(), response);

        let body = serde_json::json!({
            "operationName": query_name,
            "query": "TEST",
            "variables": serde_json::Value::Null
        });

        let result = client.post(body.to_string()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_invalid_json() {
        let mut client = MockHttpClient::new(ResponseMode::Memory);
        let result = client.post("not a json".to_string()).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid JSON format");
    }

    #[tokio::test]
    async fn test_missing_operation_name() {
        let mut client = MockHttpClient::new(ResponseMode::Memory);

        let body = serde_json::json!({ "query": "TEST" });

        let result = client.post(body.to_string()).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Missing or invalid 'operationName' field");
    }

    #[tokio::test]
    async fn test_operation_name_not_found_in_memory() {
        let mut client = MockHttpClient::new(ResponseMode::Memory);

        let body = serde_json::json!({
            "operationName": "UNKNOWN",
            "query": "TEST"
        });

        let result = client.post(body.to_string()).await;
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "No mocked memory response found for operation 'UNKNOWN'"
        );
    }

    #[tokio::test]
    async fn test_file_not_found() {
        let query_name = "NOT_FOUND".to_string();

        let response = ResponseFile::new(RESPONSE_DIR.clone(), "not_exist.json");

        let mut client = MockHttpClient::new(ResponseMode::File);
        client.set_file_response(query_name.clone(), response);

        let body = serde_json::json!({
            "operationName": query_name,
            "query": "TEST"
        });

        let result = client.post(body.to_string()).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("file 'not_exist.json' not found"));
    }

    #[tokio::test]
    async fn test_recorder_stores_request() {
        let query_name = "TEST".to_string();

        let response = serde_json::json!({ "result": "success" });

        let mut client = MockHttpClient::new(ResponseMode::Memory);
        client.set_memory_response(query_name.clone(), response.clone());

        let body = serde_json::json!({
            "operationName": query_name,
            "query": "TEST"
        });

        let expected = body.to_string();
        let recorder = client.recorder();

        let _ = client.post(expected.clone()).await;

        let recorded = recorder.read_data().await;
        assert_eq!(recorded, Some(expected));
    }
}
