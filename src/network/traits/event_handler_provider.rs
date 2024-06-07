use crate::common::event::event_handler::EventHandler;
use async_trait::async_trait;
use reqwest::Error;
#[async_trait]
pub trait EventHandlerProvider {
    async fn get_event_handler(&self, name: &str) -> Result<Option<EventHandler>, Error>;
    async fn get_all_event_handlers(&self) -> Result<Vec<EventHandler>, Error>;
}
