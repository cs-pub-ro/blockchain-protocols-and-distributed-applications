#![allow(non_snake_case)]

pub mod config;

use config::Config;
use multiversx_sc_snippets::imports::*;
use serde::{Deserialize, Serialize};
use std::{
    io::{Read, Write},
    path::Path,
};
use subscription::subscription_proxy::SubscriptionContractProxy;

const STATE_FILE: &str = "state.toml";
const CONTRACT_ABI_PATH: &str = "mxsc:../output/subscription.mxsc.json";

pub async fn subscription_cli() {
    env_logger::init();

    let mut args = std::env::args();
    let _ = args.next();
    let cmd = args.next().expect("at least one argument required");
    let config = Config::new_config();
    let mut interact = ContractInteract::new(config).await;
    match cmd.as_str() {
        "deploy" => interact.deploy().await,
        "upgrade" => interact.upgrade().await,
        "addSubscriptionPlan" => interact.add_subscription_plan().await,
        "upgradeSubscription" => interact.upgrade_subscription().await,
        "addNewSubscription" => interact.add_new_subscription().await,
        "getPlan" => interact.get_plan().await,
        "getSubscription" => interact.get_subscription().await,
        "getAllPlanIds" => interact.get_all_plan_ids().await,
        _ => panic!("unknown command: {}", &cmd),
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct State {
    contract_address: Option<Bech32Address>,
}

impl State {
    // Deserializes state from file
    pub fn load_state() -> Self {
        if Path::new(STATE_FILE).exists() {
            let mut file = std::fs::File::open(STATE_FILE).unwrap();
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();
            toml::from_str(&content).unwrap()
        } else {
            Self::default()
        }
    }

    /// Sets the contract address
    pub fn set_address(&mut self, address: Bech32Address) {
        self.contract_address = Some(address);
    }

    /// Returns the contract address
    pub fn current_address(&self) -> &Bech32Address {
        self.contract_address
            .as_ref()
            .expect("no known contract, deploy first")
    }
}

impl Drop for State {
    // Serializes state to file
    fn drop(&mut self) {
        let mut file = std::fs::File::create(STATE_FILE).unwrap();
        file.write_all(toml::to_string(self).unwrap().as_bytes())
            .unwrap();
    }
}

pub struct ContractInteract {
    interactor: Interactor,
    wallet_address: Address,
    contract_code: BytesValue,
    state: State,
}

impl ContractInteract {
    pub async fn new(config: Config) -> Self {
        let mut interactor = Interactor::new(config.gateway_uri())
            .await
            .use_chain_simulator(config.use_chain_simulator());

        interactor.set_current_dir_from_workspace("subscription");
        let wallet_address = interactor.register_wallet(test_wallets::alice()).await;

        // Useful in the chain simulator setting
        // generate blocks until ESDTSystemSCAddress is enabled
        interactor.generate_blocks_until_all_activations().await;

        let contract_code =
            BytesValue::interpret_from(CONTRACT_ABI_PATH, &InterpreterContext::default());

        ContractInteract {
            interactor,
            wallet_address,
            contract_code,
            state: State::load_state(),
        }
    }

    pub async fn deploy(&mut self) {
        let new_address = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .gas(30_000_000u64)
            .typed(SubscriptionContractProxy)
            .init()
            .code(&self.contract_code)
            .returns(ReturnsNewAddress)
            .run()
            .await;
        let new_address_bech32 = new_address.to_bech32_default();
        println!("new address: {new_address_bech32}");
        self.state.set_address(new_address_bech32);
    }

    pub async fn upgrade(&mut self) {
        let response = self
            .interactor
            .tx()
            .to(self.state.current_address())
            .from(&self.wallet_address)
            .gas(30_000_000u64)
            .typed(SubscriptionContractProxy)
            .upgrade()
            .code(&self.contract_code)
            .code_metadata(CodeMetadata::UPGRADEABLE)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn add_subscription_plan(&mut self) {
        let title = ManagedBuffer::new_from_bytes(&b""[..]);
        let duration_days = 0u64;
        let price = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(SubscriptionContractProxy)
            .add_subscription_plan(title, duration_days, price)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn upgrade_subscription(&mut self) {
        let egld_amount = BigUint::<StaticApi>::from(0u128);

        let new_plan_id = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(SubscriptionContractProxy)
            .upgrade_subscription(new_plan_id)
            .egld(egld_amount)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn add_new_subscription(&mut self) {
        let egld_amount = BigUint::<StaticApi>::from(0u128);

        let plan_id = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(SubscriptionContractProxy)
            .add_new_subscription(plan_id)
            .egld(egld_amount)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn get_plan(&mut self) {
        let plan_id = 0u32;

        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(SubscriptionContractProxy)
            .get_plan(plan_id)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn get_subscription(&mut self) {
        let user = ManagedAddress::<StaticApi>::zero();

        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(SubscriptionContractProxy)
            .get_subscription(user)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn get_all_plan_ids(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(SubscriptionContractProxy)
            .get_all_plan_ids()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }
}
