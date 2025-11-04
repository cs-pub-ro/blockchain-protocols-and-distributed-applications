# Subscription Smart Contract

This crate contains a teaching-oriented MultiversX smart contract that models a simple subscription service. The contract is written in Rust with the `multiversx-sc` framework and lives in `src/lib.rs`.

## Features
- Owner-managed subscription plans with a title, price, and duration in days.
- Payable endpoint that lets an address enroll in one of the published plans.
- Views for retrieving a single plan, a subscriber’s status, or the list of available plan IDs.
- `subscriptionCreated` event emitted whenever a new subscription is recorded.

## Contract API
- `addSubscriptionPlan(title, duration_days, price)` (owner only) creates a new plan and returns its ID.
- `addNewSubscription(plan_id)` (payable) records a subscription for the caller. The intention is that callers pay exactly the plan price.
- `getPlan(plan_id)` returns an `OptionalValue<SubscriptionPlan>` describing the plan details.
- `getSubscription(address)` returns the stored subscription for a user, if any.
- `getAllPlanIds()` enumerates every plan ID that currently exists.

### Storage Layout
- `planCounter` – monotonically increasing counter used to allocate the next plan ID.
- `subscriptionPlans[plan_id]` – single-value mapper holding the metadata for a given plan.
- `subscriptions[address]` – single-value mapper storing the latest subscription for each subscriber.

## Tests & Known Gaps
The integration tests under `tests/` showcase the contract with the MultiversX blackbox harness.
At the moment `tests/subscription_blackbox_tests.rs` contains a deliberately failing test (`add_new_subscription_should_require_payment`).

## Build & Deployment
- Build and compile the contract: `sc-meta all build`.
- The compiled artifacts land under `output/` and `wasm/`, matching the paths referenced by the scenario tests.
- Update the generated proxy in `src/subscription_proxy.rs` via `sc-meta all proxy` when you change exposed endpoints.
