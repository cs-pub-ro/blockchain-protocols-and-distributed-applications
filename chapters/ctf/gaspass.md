# CTF GasPass

Let's see who can get at least one point!

There is one obstacle, though: you must provide the exact value for gas to get even one single point. What should this value be?

Brute force is always an option, but maybe some of you will find a quicker way?

Once you get the correct value for gas, the problem becomes identical to `bump`, so 1 point in this round is enough to prove your worth!

For this challenge you will get the full code:
```Rust
#[multiversx_sc::contract]
pub trait CtfGaspass: bump_common::BumpCommon {
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}

    #[endpoint]
    fn gaspass(&self) -> bool {
        let gas_left = self.blockchain().get_gas_left();
        let caller = self.blockchain().get_caller();
        let the_key = KEY_BASELINE + self.personal_key(&caller);
        let passed = gas_left == the_key;

        if passed {
            self.perform_bump(&caller);
        } else {
            self.bumps(&caller).clear();
        };
        self.gaspass_event(&caller, passed);
        passed
    }

    fn personal_key(&self, caller: &ManagedAddress) -> u64 {
        let bytes = caller.to_byte_array();
        bytes.iter().map(|&b| b as u64).sum()
    }

    #[event("gaspass")]
    fn gaspass_event(&self, #[indexed] caller: &ManagedAddress, #[indexed] passed: bool);

    fn perform_bump(&self, bumper: &ManagedAddress) {
        self.bumps(bumper).update(|bumps| *bumps += 1u32);
    }

    #[view]
    #[storage_mapper("bumps")]
    fn bumps(&self, bumper: &ManagedAddress) -> SingleValueMapper<BigUint>;
}
```

Enjoy your hacking!
