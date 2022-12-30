use crate::State;
use chrono::NaiveDate;
use cucumber::then;

#[then(expr = "service object exists")]
async fn service_object_exists(state: &mut State) {
    assert!(state.service.is_some())
}

#[then(
    expr = "a transaction with id {int}, amount {int}, and datetime {int}-{int}-{int} {int}:{int}:{int} exists"
)]
async fn a_transaction_with_id_amount_and_datetime_exists(
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
    let transactions = state
        .service
        .as_mut()
        .unwrap()
        .get_transactions()
        .await
        .unwrap();

    assert!(transactions
        .iter()
        .any(|t| { t.get_id() == id && t.get_amount() == amount && t.get_datetime() == datetime }));
}

#[then(expr = "a transaction with id {int} does not exist")]
async fn then_a_transaction_with_id_does_not_exist(state: &mut State, id: u32) {
    let transactions = state
        .service
        .as_mut()
        .unwrap()
        .get_transactions()
        .await
        .unwrap();

    assert!(!transactions.iter().any(|t| t.get_id() == id));
}
