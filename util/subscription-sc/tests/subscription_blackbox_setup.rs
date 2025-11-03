use multiversx_sc::imports::OptionalValue;
use multiversx_sc::proxy_imports::*;
use multiversx_sc::types::{BigUint, ManagedBuffer, ReturnsHandledOrError, ReturnsResult};
use multiversx_sc_scenario::api::StaticApi;
use multiversx_sc_scenario::imports::*;
use multiversx_sc_scenario::scenario_model::TxResponseStatus;
use subscription::subscription_proxy::{Subscription, SubscriptionContractProxy};
use subscription::ContractBuilder;

pub const SUBSCRIPTION_CODE_PATH: MxscPath<'static> =
    MxscPath::new("output/subscription.mxsc.json");
pub const SUBSCRIPTION_ADDRESS: TestSCAddress = TestSCAddress::new("subscription-contract");
pub const OWNER_ADDRESS: TestAddress = TestAddress::new("owner");
pub const STUDENT_ADDRESS: TestAddress = TestAddress::new("student");

const INITIAL_BALANCE: u64 = 5_000_000_000_000_000_000;

pub struct SubscriptionContractTestState {
    pub world: ScenarioWorld,
}

impl SubscriptionContractTestState {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let mut world = ScenarioWorld::new();

        world.register_contract(SUBSCRIPTION_CODE_PATH, ContractBuilder);

        world
            .account(OWNER_ADDRESS)
            .nonce(1)
            .balance(BigUint::from(INITIAL_BALANCE));

        world
            .account(STUDENT_ADDRESS)
            .nonce(1)
            .balance(BigUint::from(INITIAL_BALANCE));

        Self { world }
    }

    pub fn deploy(&mut self) {
        self.world
            .tx()
            .from(OWNER_ADDRESS)
            .typed(SubscriptionContractProxy)
            .init()
            .code(SUBSCRIPTION_CODE_PATH)
            .new_address(SUBSCRIPTION_ADDRESS)
            .run();
    }

    pub fn create_plan(&mut self, title: &str, duration_days: u64, price: u64) -> u32 {
        self.world
            .tx()
            .from(OWNER_ADDRESS)
            .to(SUBSCRIPTION_ADDRESS)
            .typed(SubscriptionContractProxy)
            .add_subscription_plan(
                ManagedBuffer::from(title),
                duration_days,
                BigUint::from(price),
            )
            .returns(ReturnsResult)
            .run()
    }

    pub fn subscribe_without_payment(&mut self, plan_id: u32) -> Result<(), TxResponseStatus> {
        self.world
            .tx()
            .from(STUDENT_ADDRESS)
            .to(SUBSCRIPTION_ADDRESS)
            .typed(SubscriptionContractProxy)
            .add_new_subscription(plan_id)
            .returns(ReturnsHandledOrError::new())
            .run()
    }

    pub fn get_subscription(
        &mut self,
        user: TestAddress,
    ) -> OptionalValue<Subscription<StaticApi>> {
        self.world
            .query()
            .to(SUBSCRIPTION_ADDRESS)
            .typed(SubscriptionContractProxy)
            .get_subscription(user.to_managed_address())
            .returns(ReturnsResult)
            .run()
    }
}
