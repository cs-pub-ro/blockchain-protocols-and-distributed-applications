#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();
pub mod subscription_proxy;

pub const SECONDS_PER_DAY: u64 = 24 * 60 * 60;

#[type_abi]
#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, Clone)]
pub struct SubscriptionPlan<M: ManagedTypeApi> {
    pub title: ManagedBuffer<M>,
    pub price: BigUint<M>,
    pub duration_days: u64,
}

#[type_abi]
#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, Clone)]
pub struct Subscription<M: ManagedTypeApi> {
    pub plan_id: u32,
    pub started_at: u64,
    pub expires_at: u64,
    pub paid_amount: BigUint<M>,
}

#[multiversx_sc::contract]
pub trait SubscriptionContract {
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}

    #[only_owner]
    #[endpoint(addSubscriptionPlan)]
    fn add_subscription_plan(
        &self,
        title: ManagedBuffer,
        duration_days: u64,
        price: BigUint,
    ) -> u32 {
        require!(!title.is_empty(), "subscription title required");
        require!(duration_days > 0, "duration must be greater than zero");
        require!(price > 0, "price must be greater than zero");

        let plan_id = self.next_plan_id();

        let plan = SubscriptionPlan {
            title,
            price,
            duration_days,
        };

        self.subscription_plans(plan_id).set(&plan);
        plan_id
    }

    #[payable("EGLD")]
    #[endpoint(addNewSubscription)]
    fn add_new_subscription(&self, plan_id: u32) {
        let caller = self.blockchain().get_caller();

        require!(
            !self.subscription_plans(plan_id).is_empty(),
            "subscription plan not found"
        );
        require!(
            self.subscriptions(&caller).is_empty(),
            "caller already subscribed"
        );

        let plan = self.subscription_plans(plan_id).get();

        let start_timestamp = self.blockchain().get_block_timestamp();
        let duration_in_seconds = plan.duration_days.saturating_mul(SECONDS_PER_DAY);
        let expires_at = start_timestamp.saturating_add(duration_in_seconds);
        let paid_amount = self.call_value().egld().clone_value();

        let subscription = Subscription {
            plan_id,
            started_at: start_timestamp,
            expires_at,
            paid_amount: paid_amount.clone(),
        };

        self.subscriptions(&caller).set(&subscription);
        self.subscription_created(&caller, plan_id, &paid_amount);
    }

    #[view(getPlan)]
    fn get_plan(&self, plan_id: u32) -> OptionalValue<SubscriptionPlan<Self::Api>> {
        if self.subscription_plans(plan_id).is_empty() {
            OptionalValue::None
        } else {
            OptionalValue::Some(self.subscription_plans(plan_id).get())
        }
    }

    #[view(getSubscription)]
    fn get_subscription(&self, user: ManagedAddress) -> OptionalValue<Subscription<Self::Api>> {
        if self.subscriptions(&user).is_empty() {
            OptionalValue::None
        } else {
            OptionalValue::Some(self.subscriptions(&user).get())
        }
    }

    #[view(getAllPlanIds)]
    fn get_all_plan_ids(&self) -> MultiValueEncoded<u32> {
        let mut ids = MultiValueEncoded::new();
        let max_id = self.plan_counter().get();

        for plan_id in 1..=max_id {
            if !self.subscription_plans(plan_id).is_empty() {
                ids.push(plan_id);
            }
        }

        ids
    }

    #[event("subscriptionCreated")]
    fn subscription_created(
        &self,
        #[indexed] subscriber: &ManagedAddress,
        #[indexed] plan_id: u32,
        paid_amount: &BigUint,
    );

    fn next_plan_id(&self) -> u32 {
        let current = self.plan_counter().get();
        let next = current.checked_add(1).expect("plan id overflow");
        self.plan_counter().set(next);
        next
    }

    #[storage_mapper("planCounter")]
    fn plan_counter(&self) -> SingleValueMapper<u32>;

    #[storage_mapper("subscriptionPlans")]
    fn subscription_plans(&self, plan_id: u32) -> SingleValueMapper<SubscriptionPlan<Self::Api>>;

    #[storage_mapper("subscriptions")]
    fn subscriptions(&self, user: &ManagedAddress) -> SingleValueMapper<Subscription<Self::Api>>;
}
