use cucumber::then;
use crate::State;

#[then(expr = "the value in state is {int}")]
async fn a(state: &mut State, n: isize) {
    assert_eq!(state.number, n);
}
