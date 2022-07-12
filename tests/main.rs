use cucumber::WorldInit;
use tokio;

mod common;
use common::state::State;

#[tokio::main]
async fn main() {
    State::run("tests/features").await;
}
