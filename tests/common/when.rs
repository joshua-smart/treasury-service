use cucumber::when;
use crate::State;
use treasury_service::TreasuryService;

#[when(expr = "service is started")]
async fn service_is_started(state: &mut State) {
    state.service = Some(TreasuryService::new());
}
