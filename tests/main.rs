use std::fs;

use cucumber::{writer, WorldInit, WriterExt};

mod common;
use common::state::State;

#[tokio::main]
async fn main() {
    let file = fs::File::create("junit.xml").unwrap();
    let writer = writer::Basic::stdout()
        .summarized()
        .tee::<State, _>(writer::JUnit::for_tee(file, 0))
        .normalized();

    State::cucumber()
        .with_writer(writer)
        .run("tests/features")
        .await;
}
