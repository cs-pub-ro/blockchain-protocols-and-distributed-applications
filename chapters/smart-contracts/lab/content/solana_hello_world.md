# Your First Solana Program

In this lab, we'll create the simplest possible Solana program - a "Hello World" program that does nothing but can be deployed to the blockchain. This is similar to the empty contract in MultiversX, serving as a foundation for more complex programs.

## Prerequisites

Before starting, make sure you have completed the [Solana Prerequisites](./solana_prerequisites.md) guide:
- Solana CLI installed
- Anchor framework installed
- Rust toolchain installed
- Development keypair created
- Test SOL on devnet

## Creating a New Anchor Project

Anchor provides a convenient way to scaffold a new Solana program project.

### Initialize Project

Create a new Anchor project:

```bash
anchor init hello-world
cd hello-world
```

This command creates a new directory with the following structure:

```
hello-world/
├── Anchor.toml          # Anchor configuration file
├── Cargo.toml           # Rust workspace configuration
├── package.json         # Node.js dependencies
├── programs/
│   └── hello-world/
│       ├── Cargo.toml   # Program dependencies
│       └── src/
│           └── lib.rs   # Main program file
├── tests/
│   └── hello-world.ts   # TypeScript test file
└── migrations/
    └── deploy.ts        # Deployment script
```

### Understanding the Project Structure

- **`programs/hello-world/src/lib.rs`**: The main Solana program code (Rust)
- **`tests/hello-world.ts`**: Tests for the program (TypeScript)
- **`Anchor.toml`**: Configuration file specifying program IDs, clusters, etc.
- **`migrations/deploy.ts`**: Script to deploy the program

## The Default Program

Let's examine the default program that Anchor generates. Open `programs/hello-world/src/lib.rs`:

```rust
use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod hello_world {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Hello, World!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
```

### Understanding the Code

Let's break down each part:

1. **`use anchor_lang::prelude::*;`**
   - Imports Anchor's prelude, which includes commonly used types and macros

2. **`declare_id!(...)`**
   - Declares the program ID (public key) for this program
   - Anchor generates this automatically
   - This ID uniquely identifies your program on the blockchain

3. **`#[program]`**
   - Marks the module as the main program module
   - All public functions in this module become callable program instructions

4. **`pub fn initialize(ctx: Context<Initialize>) -> Result<()>`**
   - A public function that can be called from outside the program
   - `Context<Initialize>` provides access to accounts and program info
   - Returns `Result<()>` - success or error

5. **`msg!("Hello, World!");`**
   - Logs a message that appears in program logs
   - Useful for debugging

6. **`#[derive(Accounts)]`**
   - Defines the account structure for the instruction
   - `Initialize {}` is empty, meaning no accounts are required

## Building the Program

Before deploying, we need to build the program:

```bash
anchor build
```

This command:
- Compiles the Rust program to BPF bytecode
- Generates the Program Derived Address (PDA)
- Creates the program binary
- Generates the IDL (Interface Definition Language) file

**Expected Output**:
```
Building...
   Compiling hello-world v0.1.0
   Finished release [optimized] target(s) in X.XXs
```

The compiled program will be in `target/deploy/hello_world.so`.

## Deploying to Devnet

### Option 1: Using Anchor Deploy

The simplest way to deploy:

```bash
anchor deploy
```

This will:
1. Build the program (if not already built)
2. Deploy to the cluster specified in `Anchor.toml`
3. Display the program ID

### Option 2: Manual Deployment

You can also deploy manually:

```bash
# Build first
anchor build

# Deploy using Solana CLI
solana program deploy target/deploy/hello_world.so
```

### Verify Deployment

Check that your program is deployed:

```bash
solana program show <your-program-id>
```

Or view it in the explorer:
- [Solana Explorer](https://explorer.solana.com/?cluster=devnet)
- Search for your program ID

## Testing the Program

Anchor generates a test file for you. Let's examine `tests/hello-world.ts`:

```typescript
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { HelloWorld } from "../target/types/hello_world";
import { assert } from "chai";

describe("hello-world", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.HelloWorld as Program<HelloWorld>;

  it("Initializes", async () => {
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
```

### Run Tests

Make sure you have a local validator running or are connected to devnet:

```bash
# If using local validator, start it first:
solana-test-validator

# In another terminal, run tests:
anchor test
```

**Expected Output**:
```
  hello-world
    ✓ Initializes (XXXms)

  1 passing (XXs)
```

## Simplifying to a True "Hello World"

The default program includes an `initialize` function. For a true minimal program (like the empty contract), we can simplify it further:

```rust
use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod hello_world {
    use super::*;

    // This is the minimal program - it does nothing
    // but can still be deployed to the blockchain
}
```

However, Solana programs require at least one instruction, so we'll keep a minimal one:

```rust
use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod hello_world {
    use super::*;

    pub fn say_hello(_ctx: Context<SayHello>) -> Result<()> {
        msg!("Hello, World from Solana!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SayHello {}
```

## Understanding Program Size

After building, check the program size:

```bash
ls -lh target/deploy/hello_world.so
```

Solana programs have a size limit:
- **Maximum Program Size**: 10 KB (10,240 bytes) for on-chain programs
- **Upgradeable Programs**: Can be larger but must fit in the upgrade buffer

Our minimal program should be well under this limit.

## Program ID and Upgradability

### Program ID

The program ID is defined in:
- `declare_id!()` macro in `lib.rs`
- `Anchor.toml` under `[programs.devnet]`

These must match for the program to work correctly.

### Making Programs Upgradeable

By default, Anchor programs are upgradeable. This means:
- You can update the program code
- The program address stays the same
- You need the upgrade authority keypair

To deploy as non-upgradeable (final):
```bash
solana program deploy --program-id <keypair-file> target/deploy/hello_world.so --final
```

## Practice Exercises

### Exercise 1: Create and Deploy
1. Create a new Anchor project called `my-first-program`
2. Build the program
3. Deploy it to devnet
4. Verify the deployment in Solana Explorer

### Exercise 2: Modify the Program
1. Add a new instruction called `greet` that takes a name parameter
2. Log a personalized greeting
3. Rebuild and redeploy
4. Test the new instruction

### Exercise 3: Compare with MultiversX
1. Compare the Solana program structure with the MultiversX empty contract
2. Note the differences in:
   - Language and framework
   - Deployment process
   - Program size
   - Upgrade mechanism

## Common Issues and Solutions

### Issue: Build Fails
**Error**: `error: linker 'cc' not found`
- **Solution**: Install a C compiler (build-essential on Linux, Xcode on macOS)

### Issue: Deployment Fails
**Error**: `Error: Account not found`
- **Solution**: Make sure you have SOL in your account: `solana airdrop 2`

### Issue: Program ID Mismatch
**Error**: `Error: Invalid program id`
- **Solution**: Make sure the program ID in `lib.rs` matches `Anchor.toml`

### Issue: Tests Fail
**Error**: `Error: Connection refused`
- **Solution**: Start local validator: `solana-test-validator`

## Next Steps

Now that you've created your first Solana program:

1. ✅ Understand the basic program structure
2. ✅ Know how to build and deploy
3. ✅ Understand how to test programs
4. ➡️ Proceed to [Solana Program with State](./solana_with_state.md) (when available)
5. ➡️ Learn about [Solana Accounts](./solana_accounts.md) (when available)

## Summary

In this lab, you've learned:
- How to create a new Anchor project
- The structure of a Solana program
- How to build and deploy Solana programs
- How to test Solana programs
- The difference between upgradeable and non-upgradeable programs

This minimal program serves as the foundation for all Solana program development.

