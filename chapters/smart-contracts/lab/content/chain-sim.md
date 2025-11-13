# Interactor and System Testing lab

This lab walks through building integration tests and running a local blockchain using the `Interactor` and `Chain-Simulator`. We will use the same `subscription-sc` smart contract from the previous lab.

Key takeaways:
- The `interactor` is a module that facilitates interaction with smart contracts in a simulated blockchain environment or directly on the live environments. You can find more info in the official [documentation](https://docs.multiversx.com/developers/meta/interactor/interactors-overview/).
- The `chain-simulator` mimics the behavior of a local testnet. It helps developers test smart contracts and dApps in a controlled environment. More details can be found in the official [documentation](https://docs.multiversx.com/sdk-and-tools/chain-simulator/)

# Prerequisites
## 1. Clone the SC

Clone and pull it in before you start:

```bash
git clone https://github.com/andreiblt1304/subscription-sc.git # HTTPS
git clone git@github.com:andreiblt1304/subscription-sc.git # SSH
```

There have been some changes made to the original repository to accommodate this lab. The most important ones are:
- Added `interactor` folder that contains the interactor code.
- Added the `sc-config.toml` file that tells the sc-meta tool where to generate the proxy file.
```toml
    [[proxy]]
    path = "src/subscription_proxy.rs"
```

## 2. Chain-Simulator setup

If you are using the provided VM, the chain-simulator is already installed. If you are using your local machine all you have to do is have Docker installed and run the following command:

```bash
# This will also install other dependencies but if you already have them installed it will skip them
sc-meta all install
```

To start the chain-simulator run the following command:

```bash
sc-meta cs-start
```

## Understanding the interactor

The interactor has a straight forward structure:
- `src` - contains the main interactor code.
- `tests` - contains the integration tests that will be run against the chain-simulator.

> The interactor uses *vanilla async rust*, so make sure to familiarize yourself with it if you haven't already. Remember to use errors from the compiler as your friend! For further information regarding rust or async rust feel free to use any other source of your preference.

## Deep dive into the interactor tests

```rust
    #[tokio::test]
    #[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
    async fn test() {
    // let mut interactor = ContractInteract::new(Config::new_config()).await;
    let mut interactor = ContractInteract::new(Config::chain_simulator_config()).await;
```

- `#[tokio::test]` - this macro allows us to write async tests using the tokio runtime.
- `#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]` - this attribute ensures that the test is only run when the `chain-simulator-tests` feature is enabled.
- `async fn test() {}` - the actual test function that will be executed.
- `ContractInteract::new(Config::chain_simulator_config());` - this line initializes a new instance of the interactor with the chain-simulator configuration.
- `ContractInteract::new(Config::new_config());` - this line initializes a new instance of the interactor with a custom configuration. Take a look at the `Config` struct to see how to create your own configuration.

To run the tests you have to run this command:

```bash
    sc-meta test -c
    # OR
    sc-meta test --chain-simulator-tests
```

> Remember to have the chain-simulator running in the background before running the tests.

## Do it yourself

> Quick tip: the interactor uses the same syntax for writing tests, *the unified syntax*. Since the last lab you worked on blackbox tests, this should be easy work. Here is the [overview](https://docs.multiversx.com/developers/testing/rust/sc-test-overview/#calling-contract-code).

The task for this lab is to add more tests to the interactor. Here are some ideas:
- Create a test that deploys the contract and checks if the deployment was successful.
- Create a test that tries to subscribe with insufficient funds and checks if the correct error is returned.
- Create a test that checks if the subscription expiration date is correctly set after a successful subscription.
- Create a test that checks if the unsubscribe functionality works as expected.

Feel free to come up with your own ideas as well! The goal is to get comfortable with writing integration tests using the interactor and chain-simulator.
For easier attendance please also create a test that run directly on the MultiversX devnet environment. After that send the transaction hash.
