use cucumber::then;
use crate::State;

#[then(expr = "service object exists")]
async fn service_object_exists(state: &mut State) {
    assert!(state.service.is_some())
}
