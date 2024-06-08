use async_trait::async_trait;

#[async_trait]
pub trait TokenProvider {
    async fn get_token(&self) -> Result<Option<String>, reqwest::Error>;
}
