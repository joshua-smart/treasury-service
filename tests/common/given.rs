use crate::State;
use cucumber::given;

#[given(expr = "the number {int} is in the state")]
async fn given_num_in_state(state: &mut State, n: isize) {
    state.number = n;
}
