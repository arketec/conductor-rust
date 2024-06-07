use crate::common::event::event_handler::EventHandler;
use async_trait::async_trait;
use reqwest::Error;

#[async_trait]
pub trait EventHandlerMutator {
    async fn update_event_handler(&self, handler: EventHandler) -> Result<bool, Error>;
    async fn save_event_handler(&self, handler: EventHandler) -> Result<bool, Error>;
}
