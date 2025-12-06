# Solana Explorer

Solana has several blockchain explorers that allow you to inspect transactions, blocks, accounts, and programs. In this lab, we'll explore the three most popular Solana explorers: Solscan, Solana Explorer, and Solana Beach.

## Summary

In this lab, you'll learned to:
- Navigate three major Solana explorers (Solscan, Solana Explorer, Solana Beach)
- Inspect blocks and understand Solana's slot-based system
- Analyze transactions and their multiple instructions
- Explore accounts and understand Solana's account model
- Examine programs (smart contracts) and PDAs
- Compare Solana explorers with MultiversX explorers


## Solana Explorers Overview

### 1. Solscan
**URL**: [solscan.io](https://solscan.io/)

Solscan is one of the most feature-rich Solana explorers, offering detailed transaction analysis, token tracking, and DeFi protocol insights.

**Key Features**:
- Transaction details and status
- Token balances and transfers
- NFT collections
- DeFi protocol interactions
- Program (smart contract) inspection
- Validator information

### 2. Solana Explorer (Official)
**URL**: [explorer.solana.com](https://explorer.solana.com/)

The official Solana explorer maintained by the Solana Foundation, providing core blockchain data.

**Key Features**:
- Clean, official interface
- Transaction and block details
- Account information
- Program inspection
- Cluster selection (Mainnet, Testnet, Devnet)

### 3. Solana Beach
**URL**: [solanabeach.io](https://solanabeach.io/)

Solana Beach offers comprehensive analytics and monitoring tools for the Solana network.

**Key Features**:
- Network statistics
- Validator rankings
- Transaction history
- Account monitoring
- Token analytics

---

## Exploring Blocks

Let's start by examining a recent block on Solana. Unlike MultiversX which uses shards, Solana processes transactions in a single global state.

### Block Structure

Navigate to [Solscan](https://solscan.io/) and look at the latest blocks. You'll notice:

- **Slot**: Solana uses slots instead of block heights. Each slot represents a time period (approximately 400ms)
- **Block Height**: Sequential block number
- **Transactions**: Number of transactions in the block
- **Success Rate**: Percentage of successful transactions
- **Leader**: The validator that produced this block

### Practice: Block Inspection

1. Go to [Solscan Blocks](https://solscan.io/blocks) and open a recent block
2. Observe the following fields:
   - Slot number
   - Block height
   - Timestamp
   - Number of transactions
   - Success rate
   - Leader (validator) address
3. Compare with a block on [Solana Explorer](https://explorer.solana.com/) - notice the different presentation styles
4. Open the same block on [Solana Beach](https://solanabeach.io/) and compare the information displayed

---

## Exploring Transactions

Solana transactions are different from Ethereum-style transactions. They can include multiple instructions and require multiple signatures.

### Transaction Structure

Let's examine a transaction. Here's an example transaction on Solana:
[Example Transaction on Solscan](https://solscan.io/tx/example)

**Key Fields**:
- **Signature**: Unique transaction identifier
- **Status**: Success or Failed
- **Slot**: The slot in which the transaction was included
- **Timestamp**: When the transaction was processed
- **Fee**: Transaction fee paid (in SOL)
- **Signer(s)**: Account(s) that signed the transaction
- **Instructions**: The operations performed in this transaction
- **Accounts Involved**: All accounts read or written to

### Transaction Types

Solana transactions can contain multiple instructions:
- **Transfer**: SOL or token transfers
- **Program Invocation**: Calls to smart contracts (programs)
- **Account Creation**: Creating new accounts
- **Staking**: Staking SOL to validators

### Practice: Transaction Exploration

1. Go to [Solscan Transactions](https://solscan.io/transactions) and open 3 different transactions
2. For each transaction, identify:
   - The type of transaction (transfer, program call, etc.)
   - Number of instructions
   - Number of signers
   - Fee paid
   - Status (success/failure)
3. Click on one of the signer addresses to view their account details
4. If the transaction involves a program, click on the program address to see program information
5. Compare the same transaction across Solscan, Solana Explorer, and Solana Beach

---

## Exploring Accounts

Solana uses an account-based model where accounts store data and code. There are different types of accounts:

### Account Types

1. **System Accounts**: Regular user accounts owned by the System Program
2. **Program Accounts**: Accounts owned by programs (smart contracts)
3. **Token Accounts**: Accounts holding SPL tokens
4. **Program Derived Addresses (PDAs)**: Deterministic addresses derived from programs

### Account Information

When viewing an account, you'll see:
- **Address**: The public key of the account
- **Balance**: SOL balance
- **Owner**: The program that owns this account
- **Executable**: Whether this account is a program
- **Rent**: SOL locked for account storage
- **Data**: Account data (if any)

### Practice: Account Analysis

1. Find a transaction and click on one of the account addresses
2. Examine the account details:
   - Balance in SOL
   - Owner program
   - Whether it's executable (a program)
   - Rent-exempt status
3. View the transaction history for this account
4. If it's a token account, identify:
   - Token mint address
   - Token balance
   - Token owner
5. Compare account views across different explorers

---

## Exploring Programs (Smart Contracts)

Programs in Solana are executable accounts that contain smart contract code. Let's explore how to inspect programs.

### Program Accounts

Program accounts are marked as "Executable" and have:
- **Program ID**: The address of the program
- **Owner**: Usually the BPF Loader or Native Loader
- **Data**: Program bytecode (not visible in explorers)
- **Upgrade Authority**: Who can upgrade the program

### Program-Derived Accounts (PDAs)

Programs can create PDAs - deterministic addresses that the program controls. These are used for:
- Storing program state
- Creating unique addresses for each user/program interaction
- Managing token accounts

### Practice: Program Inspection

1. Find a transaction that calls a program (look for transactions with program invocations)
2. Click on the program address to view program details:
   - Program ID
   - Owner
   - Upgrade authority (if any)
   - Transaction history
3. Look for Program Derived Addresses (PDAs) associated with this program
4. Examine the program's transaction history to understand what operations it performs
5. Check if the program has a verified source code (some explorers show this)

---

## Comparative Analysis: Solana vs MultiversX Explorers

### Key Differences

| Feature | Solana | MultiversX |
|---------|--------|------------|
| **Block Structure** | Slots (time-based) | Blocks with shards |
| **Transaction Model** | Multiple instructions per transaction | Single transaction per entry |
| **Account Model** | Account-based with PDAs | Account-based with shards |
| **Program Storage** | Programs are executable accounts | Smart contracts stored separately |
| **Fee Structure** | Per signature + compute units | Dynamic fees based on gas |


## Practice Exercises

### Exercise 1: Transaction Deep Dive
1. Find a transaction that transfers SOL between two accounts
2. Identify all accounts involved in the transaction
3. Determine the transaction fee
4. Check if the transaction was successful
5. View the same transaction on all three explorers and note differences

### Exercise 2: Program Analysis
1. Find a transaction that invokes a program (not a simple transfer)
2. Identify the program being called
3. Examine the program's account details
4. Look for any PDAs created or used by this program
5. Review the program's recent transaction history

### Exercise 3: Account Investigation
1. Pick a random account address from a transaction
2. View the account's current balance
3. Check the account's transaction history
4. Identify what type of account it is (system account, token account, program, etc.)
5. If it's a token account, identify the token mint and owner

### Exercise 4: Block Comparison
1. Find a recent block on Solana
2. Count the number of transactions
3. Calculate the success rate
4. Identify the validator (leader) that produced the block
5. Compare this with a MultiversX block - note the structural differences

### Exercise 5: Explorer Comparison
1. Pick one transaction signature
2. View it on Solscan, Solana Explorer, and Solana Beach
3. Create a comparison table noting:
   - Information displayed
   - Ease of navigation
   - Additional features available
   - Which explorer you prefer and why

---

## Additional Resources

- [Solana Explorer Documentation](https://docs.solana.com/developing/clients/jsonrpc-api)
- [Solscan API Documentation](https://public-api.solscan.io/docs/)
- [Solana Account Model](https://docs.solana.com/developing/programming-model/accounts)
- [Solana Transaction Format](https://docs.solana.com/developing/programming-model/transactions)
- [Understanding Solana Slots](https://docs.solana.com/terminology#slot)

---

