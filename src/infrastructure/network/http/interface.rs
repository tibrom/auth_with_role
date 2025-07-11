pub trait HttpClientInterface: Send + Sync {
    type Error;
    async fn post(&self, body: String) -> Result<String, Self::Error>;
}
