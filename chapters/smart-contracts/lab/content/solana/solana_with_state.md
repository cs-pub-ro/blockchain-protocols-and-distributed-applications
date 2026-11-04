# Understanding Program Derived Addresses (PDAs)

## The Core Concept
Program Derived Addresses (PDAs) are one of the most powerful and critical concepts in Solana development. 

A traditional Solana Keypair consists of a Public Key and a Secret Key. The Secret Key is used to sign transactions, proving ownership over the Public Key (and any assets or data it holds).

**A PDA, however, is a Public Key that does NOT have a corresponding Secret Key.**

Because there is no Private Key, no human or external wallet can sign for a PDA. Instead, **PDAs are cryptographically designed so that only a specific Solana Program can sign for them.**

### Why do we need PDAs?
1. **Deterministic Account Discovery:** Instead of saving the randomized public keys of users' data accounts inside your frontend database, you can deterministically compute the address of a user's data on the fly.
2. **Program Authority:** Programs can own and manage SOL, tokens, and other assets without needing a backend server to hold a private key.

## 1. Computing a PDA
To find a PDA, we use a concept called "seeds". A seed can be any byte slice: a string, a public key, or an integer.

```rust
// On-chain Rust
let (pda, bump) = Pubkey::find_program_address(
    &[b"vault", user.key().as_ref()], 
    program_id
);
```
```javascript
// Off-chain TypeScript (Frontend)
const [pda, bump] = await PublicKey.findProgramAddress(
    [Buffer.from("vault"), user.publicKey.toBuffer()],
    programId
);
```

### The "Bump" Seed
Because PDAs must not sit on the `ed25519` elliptic curve (to ensure there is no private key), `find_program_address` iterates through a "bump" value (from 255 down to 0) appending it to your seeds until it finds a valid address that is off the curve.


## 2. Using PDAs with Anchor
Anchor makes interacting with PDAs incredibly simple through the `#[account(seeds = [...], bump)]` macro.

Let's build a "User Profile" system where each user can only have exactly ONE profile document.

```rust
use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod profile_system {
    use super::*;

    pub fn create_profile(ctx: Context<CreateProfile>, username: String) -> Result<()> {
        let profile = &mut ctx.accounts.profile;
        profile.username = username;
        profile.authority = *ctx.accounts.user.key;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateProfile<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + 32 + 50,
        seeds = [b"profile_seed", user.key().as_ref()], // Deterministic seeds
        bump // Anchor automatically calculates and saves this
    )]
    pub profile: Account<'info, UserProfile>,
    
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct UserProfile {
    pub authority: Pubkey,
    pub username: String,
}
```

### Benefits of this design:
If Alice interacts with this contract, her profile address will be `hash("profile_seed" + Alice's Pubkey)`. 
1. The frontend never needs to store her profile's address. It can just recalculate it instantly.
2. Alice **cannot** create two profiles. If she calls `create_profile` again, Anchor will try to `init` the exact same PDA address, and the transaction will fail because the account already exists.

## 3. Assignment
1. Extend the previously created To-Do list contract.
2. Instead of generating a random Keypair for each new task, use a PDA. 
3. The seeds should be `[b"task", user.key().as_ref(), task_id.to_le_bytes().as_ref()]`.
4. The `task_id` should be a numerical index so the user can have multiple tasks.
