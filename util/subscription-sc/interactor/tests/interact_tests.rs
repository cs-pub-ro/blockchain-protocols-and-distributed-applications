use multiversx_sc_snippets::imports::*;
use rust_interact::{config::Config, ContractInteract};

// Simple deploy test
#[tokio::test]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn deploy_test_subscription() {
    // let mut interactor = ContractInteract::new(Config::new_config()).await;
    let mut interactor = ContractInteract::new(Config::chain_simulator_config()).await;

    interactor.deploy().await;
}
