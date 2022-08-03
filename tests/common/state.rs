use cucumber::{World, WorldInit};
use async_trait::async_trait;
use treasury_service::TreasuryService;
use std::convert::Infallible;

#[derive(Debug, WorldInit)]
pub struct State {
    pub service: Option<TreasuryService>
}

#[async_trait(?Send)]
impl World for State {
    type Error = Infallible;

    async fn new() -> Result<Self, Self::Error> {
        Ok(State { service: None })
    }
}
