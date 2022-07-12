use cucumber::when;
use crate::State;

#[when(expr = "the addTwo method is called")]
async fn add_two(state: &mut State) {
    state.number += 2;
}
