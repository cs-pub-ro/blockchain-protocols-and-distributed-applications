# CTF Bump

Let's see who can bump the most!

Participants are asked to call `bump` as many times as possible. Each `bump` increases the score by one.

It is also possible to donate your bumps, if you're feeling generous (or if you used a smart contract to harvest more bumps).

```Rust
#[multiversx_sc::contract]
pub trait CtfBump: bump_common::BumpCommon {
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}

    #[endpoint]
    fn bump(&self) {
        let caller = self.blockchain().get_caller();
        self.perform_bump(&caller);
    }

    fn perform_bump(&self, bumper: &ManagedAddress) {
        self.bumps(bumper).update(|bumps| *bumps += 1u32);
    }

    #[endpoint(donateBumps)]
    fn donate_bumps(&self, receiver: ManagedAddress) {
        let caller = self.blockchain().get_caller();
        let caller_bumps = self.bumps(&caller).take();
        self.bumps(&receiver).update(|bumps| *bumps += caller_bumps);
    }
    
    #[view]
    #[storage_mapper("bumps")]
    fn bumps(&self, bumper: &ManagedAddress) -> SingleValueMapper<BigUint>;
}
```