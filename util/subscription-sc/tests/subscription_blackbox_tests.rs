use multiversx_sc::imports::OptionalValue;

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
}
