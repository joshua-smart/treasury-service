use crate::State;
use cucumber::then;

#[then(expr = "service object exists")]
async fn service_object_exists(state: &mut State) {
    assert!(state.service.is_some())
}

#[then(expr = "a transaction with id {int}, amount {int}, and date {string} exists")]
async fn a_transaction_with_id_amount_and_datetime_exists(
    state: &mut State,
    id: u32,
    amount: u32,
    date_string: String,
) {
    let transactions = state
        .service
        .as_mut()
        .unwrap()
        .get_transactions()
        .await
        .unwrap();

    assert!(transactions.iter().any(|t| {
        t.get_id() == id && t.get_amount() == amount && t.get_date().to_string() == date_string
    }));
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
