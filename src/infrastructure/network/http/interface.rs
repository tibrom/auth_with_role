pub trait HttpClientInterface: Send + Sync {
    type Error: ToString + Send + Sync;
    async fn post(&mut self, body: String) -> Result<String, Self::Error>;
}
