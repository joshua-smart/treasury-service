use crate::State;
use chrono::NaiveDate;
use cucumber::when;
use treasury_service::TreasuryService;

#[when(expr = "service is started")]
async fn service_is_started(state: &mut State) {
    state.service = Some(TreasuryService::new(":memory:").await.unwrap());
}

#[when(
    expr = "a transaction with amount {int}, and datetime {int}-{int}-{int} {int}:{int}:{int} is added"
)]
async fn when_a_transaction_is_added(
    state: &mut State,
    amount: u32,
    day: u32,
    month: u32,
    year: i32,
    hour: u32,
    minute: u32,
    second: u32,
) {
    let datetime = NaiveDate::from_ymd_opt(year, month, day)
        .unwrap()
        .and_hms_opt(hour, minute, second)
        .unwrap();

    state
        .service
        .as_mut()
        .unwrap()
        .add_transaction(amount, datetime)
        .await
        .unwrap();
}

#[when(
    expr = "a transaction is updated with values id {int}, amount {int} and datetime {int}-{int}-{int} {int}:{int}:{int}"
)]
async fn a_transaction_is_updated_with_values_id_amount_and_datetime(
    state: &mut State,
    id: u32,
    amount: u32,
    day: u32,
    month: u32,
    year: i32,
    hour: u32,
    minute: u32,
    second: u32,
) {
    let datetime = NaiveDate::from_ymd_opt(year, month, day)
        .unwrap()
        .and_hms_opt(hour, minute, second)
        .unwrap();

    state
        .service
        .as_mut()
        .unwrap()
        .set_transaction(id, amount, datetime)
        .await
        .unwrap();
}

#[when(expr = "a transaction with id {int} is removed")]
async fn a_transaction_with_id_is_removed(state: &mut State, id: u32) {
    state
        .service
        .as_mut()
        .unwrap()
        .remove_transaction(id)
        .await
        .unwrap();
}
