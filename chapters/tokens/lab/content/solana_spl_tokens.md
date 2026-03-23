# Solana SPL Tokens

## Introduction
On older blockchains like Ethereum, every new token requires deploying an entirely new smart contract following the ERC-20 standard. Solana uses a starkly different approach. 

The Solana Foundation has already deployed the **Solana Program Library (SPL) Token Program**. To create a new token, you do not deploy your own contract. Instead, you send instructions to the official SPL Token Program, asking it to create a "Mint" account for you.

## 1. The SPL Token Architecture

### The Mint Account
The Mint account represents the token itself. It stores global metadata about the token, such as:
- Total supply
- Number of decimals (e.g., 9 decimals means a `1` on chain is actually `0.000000001` tokens)
- Mint Authority (who is allowed to create more tokens)
- Freeze Authority (who is allowed to freeze token transfers)

### The Token Account
While the Mint account tracks global supply, a **Token Account** tracks how many of those tokens a specific user holds. Your classic Solana wallet (like Phantom) doesn't directly hold your USDC. Instead, your wallet owns a specific "Token Account", and that Token Account points to the USDC Mint Account.

## 2. Creating a Fungible Token via the CLI
We will use the `@solana/spl-token` CLI tool to create our first token on the devnet.

```bash
# 1. Ensure you are on Devnet
solana config set --url devnet

# 2. Check your balance
solana balance

# 3. Create a new Token Mint
spl-token create-token
```
Expected output:
> Creating token `5J4c...` under program `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA`

Save the token address. This is your `Mint Address`.

```bash
# 4. Create an Associated Token Account (ATA) for your wallet
spl-token create-account <YOUR_MINT_ADDRESS>
```
An Associated Token Account (ATA) is a PDA derived from your wallet address and the Mint address. It's the standard way wallets track balances.

```bash
# 5. Mint some tokens to yourself
spl-token mint <YOUR_MINT_ADDRESS> 1000
```
Congratulations! You've just created a fungible token on Solana. Check your Phantom wallet (configured to Devnet), and you will see an "Unknown Token" with a balance of 1000.

## 3. Creating a Mint from Anchor (Bonus)
You can also invoke the Token Program directly from your Rust Anchor contract via Cross Program Invocations (CPI).

Using the `anchor-spl` crate:
```rust
use anchor_spl::token::{self, MintTo, Token, TokenAccount};

#[program]
pub mod token_minter {
    use super::*;

    pub fn mint_reward(ctx: Context<MintReward>, amount: u64) -> Result<()> {
        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.user_token_account.to_account_info(),
            authority: ctx.accounts.mint_authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        
        token::mint_to(cpi_ctx, amount)?;
        Ok(())
    }
}
```

## 4. Assignment
Create an Anchor smart contract that dispenses 10 custom "Class Tokens" to any student who calls it. 
*Hint:* Your contract will need a PDA that acts as the `mint_authority` so the contract can autonomously sign for the mint.
