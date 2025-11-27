# Subscription Management SC

This lab walks through a real-world subscription smart contract. You will practice cloning a contract that lives as a git submodule, compiling it with the `sc-meta` CLI, generating a Rust interactor, and tightening both the contract logic and its scenario tests.

Key takeaways:
- `sc-meta` orchestrates compilation, snippets, and tests for MultiversX smart contracts.
- Payable endpoints must enforce economic rules explicitly; tests highlight the missing checks.
- Interactors are code-generated and let you script deployments and endpoint calls from Rust.

## 1. Clone the submodule

Clone and checkout the correct branch:

```bash
# Either HTTPS or SSH
git clone https://github.com/andreiblt1304/subscription-sc.git 
git clone git@github.com:andreiblt1304/subscription-sc.git 

# Checkout blackbox branch
git checkout blackbox-lab
```

Inspect the contract sources (`src/lib.rs`) and the black-box tests under `tests/`.

## 2. Build with `sc-meta`

From the root of the crate, compile the contract:

```bash
sc-meta all build
```

You should see the resulting artifacts in `output/`.

## 3. Generate the interactor

Produce a Rust interactor so you can script deployments and calls:

```bash
sc-meta all snippets
```

This creates an `interactor/` project. Open it to understand the generated proxy code and how endpoint calls are exercised from Rust.

## 4. Run the scenario tests

Trigger the test harness:

```bash
sc-meta test
```

The `add_new_subscription_should_require_payment` test is expected to fail initially. Use the failure to trace how the contract handles the payment checks.

## 5. Fix the contract and the tests

- Enforce that `add_new_subscription` only records a subscription when the caller pays exactly the plan price. Hint: read the `call_value()` before storing the subscription.
- Augment the test so it also covers the happy path (send the correct payment and assert the on-chain subscription mirrors it). The helper utilities in `tests/subscription_blackbox_setup.rs` already handle caller addresses; extend them to accept an EGLD amount.

Re-run `sc-meta test` until the suite is green.

## Practice

* Finish the contract fix and extend the test so it validates both failure (no payment) and success (exact price) scenarios.
