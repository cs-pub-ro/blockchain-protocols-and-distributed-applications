use multiversx_sc::{imports::OptionalValue, types::{BigUint, Egld, EgldPayment}};

use crate::subscription_blackbox_setup::{SubscriptionContractTestState, STUDENT_ADDRESS};

const MONTHLY_PLAN_PRICE: u64 = 1_000_000_000_000_000;
const MONTHLY_PLAN_DURATION_DAYS: u64 = 30;
pub mod subscription_blackbox_setup;

/// Demonstrates how the blackbox harness can surface issues in `add_new_subscription`.
/// Students can use the failing expectation below to trace the logic gap and implement a fix.
#[test]
fn add_new_subscription_should_require_payment() {
    let mut state = SubscriptionContractTestState::new();

    state.deploy();

    let plan_id = state.create_plan(
        "Monthly Plan",
        MONTHLY_PLAN_DURATION_DAYS,
        MONTHLY_PLAN_PRICE,
    );

    let response = state.subscribe_without_payment(plan_id);

    assert!(
        response.is_err(),
        "Expected subscription to fail when no payment is sent"
    );

    let subscription = state.get_subscription(STUDENT_ADDRESS);

    assert!(
        matches!(subscription, OptionalValue::None),
        "Subscription should not be recorded when payment is missing"
    );


    let b = BigUint::from(MONTHLY_PLAN_PRICE + 1);
    let egld = EgldPayment::from(Egld(b));
    let response = state.subscribe(plan_id, egld);

    assert!(
        response.is_err(),
        "expected the subscription to fail: too much money"
    );

    let subscription = state.get_subscription(STUDENT_ADDRESS);

    assert!(
        subscription.is_none(),
        "Subscription should NOT be recorded when too much payment was sent"
    );


    let b = BigUint::from(MONTHLY_PLAN_PRICE);
    let egld = EgldPayment::from(Egld(b));
    let response = state.subscribe(plan_id, egld);

    assert!(
        response.is_ok(),
        "expected the subscription to succeed"
    );

    let subscription = state.get_subscription(STUDENT_ADDRESS);

    assert!(
        subscription.is_some(),
        "Subscription should be recorded when payment was sent"
    );



}


#[test]
fn subscription_upgradable() {
    let mut state = SubscriptionContractTestState::new();

    state.deploy();

    let plan_id = state.create_plan(
        "Monthly Plan",
        MONTHLY_PLAN_DURATION_DAYS,
        MONTHLY_PLAN_PRICE,
    );

    let premium_plan_id = state.create_plan(
        "PREMIUM Monthly Plan Plus Pro Super Extra Mega Double Deluxe",
        MONTHLY_PLAN_DURATION_DAYS,
        MONTHLY_PLAN_PRICE + 1,
    );

    let b = BigUint::from(MONTHLY_PLAN_PRICE);
    let egld = EgldPayment::from(Egld(b));
    let response = state.subscribe(plan_id, egld);

    assert!(
        response.is_ok(),
        "expected the subscription to succeed"
    );

    let subscription = state.get_subscription(STUDENT_ADDRESS);

    assert!(
        subscription.is_some(),
        "Subscription should be recorded when payment was sent"
    );


    let b = BigUint::from(1_u64);
    let egld = EgldPayment::from(Egld(b));
    let response = state.upgrade_subscription(premium_plan_id, egld);

    assert!(
        response.is_ok(),
        "expected the subscription to succeed"
    );

    let subscription = state.get_subscription(STUDENT_ADDRESS);

    assert!(
        subscription.is_some(),
        "Subscription should be recorded when payment was sent"
    );



}
