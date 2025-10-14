# CTF Coinflip

Let's see who can get the highest score!

The rules are as follows:
- You call coinflip.
- The contract flips a coin, your odds of being lucky are 50/50.
- If you are lucky, you get 1 point.
- If you are unlucky, your score is reset to zero.

You can rely on sheer brute-force, but some of you will notice that the "randomness" function is not exactly bulletproof.

Best of _"luck"_!

Here is the code that is executed:
```Rust
    #[endpoint]
    fn coinflip(&self) -> bool {
        let lucky = self.flip_coin();
        let caller = self.blockchain().get_caller();
        if lucky {
            self.perform_bump(&caller);
        } else {
            let caller = self.blockchain().get_caller();
            self.bumps(&caller).clear();
        };
        self.coinflip_event(&caller, lucky);
        lucky
    }

    fn flip_coin(&self) -> bool {
        let block_nonce = self.blockchain().get_block_nonce();
        block_nonce & 1 == block_nonce & 2
    }

    fn perform_bump(&self, bumper: &ManagedAddress) {
        self.bumps(bumper).update(|bumps| *bumps += 1u32);
    }

    #[view]
    #[storage_mapper("bumps")]
    fn bumps(&self, bumper: &ManagedAddress) -> SingleValueMapper<BigUint>;
```