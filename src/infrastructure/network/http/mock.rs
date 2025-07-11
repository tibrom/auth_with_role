use reqwest::{Method, Response};
use std::time::{Duration, SystemTime};
use tokio::sync::RwLock;
use super::interface::HttpClientInterface;
use std::sync::atomic::{AtomicU8, AtomicUsize, Ordering};

use std::sync::Arc;


#[derive(Clone)]
pub struct MockHttpClientResponse {
    act_count: Arc<RwLock<u16>>,
    data: Arc<RwLock<Option<String>>>,
}

impl MockHttpClientResponse {
    pub fn new() -> Self {
        Self {
            act_count: Arc::new(RwLock::new(0)),
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

    async fn get_act_counter(&self) -> u16 {
        self.act_count.read().await.clone()
    }
}

#[derive(Clone)]
pub struct MockHttpClient {
    
    response: serde_json::Value,
    recorder: MockHttpClientResponse,
}

impl MockHttpClient {
    pub fn new(response: serde_json::Value) -> Self {
        Self {
            
            response,
            recorder: MockHttpClientResponse::new(),
        }
    }

    pub fn recorder(&self) -> MockHttpClientResponse {
        self.recorder.clone()
    }

    
}


impl HttpClientInterface for MockHttpClient {
    type Error = ();
    async fn post(&self, body: String) -> Result<String, Self::Error> {
        self.recorder.set_data(body).await;
        Ok(self.response.to_string())
    }
    
    
}
