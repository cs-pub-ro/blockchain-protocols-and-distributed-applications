# Practical Exam BPDA

**Date:** 5 February 2024  
**Duration:** 60 minutes  

---

## 1. Instructions

- You have **60 minutes** for this practical exam.
- You can access any non-collaborative resource including:
  - [Course Website](https://cs-pub-ro.github.io/blockchain-protocols-and-distributed-applications/)
  - [MultiversX Documentation](https://docs.multiversx.com/developers/smart-contracts)
  - [MultiversX Contract Examples](https://github.com/multiversx/mx-contracts-rs/)
- You can solve the tasks on any blockchain you want (e.g., MultiversX, Ethereum, Solana).
- You can solve the tasks in any order.
- **Total Points:** 110p (100p will get you the maximum grade).

### Teacher Addresses
- **MultiversX:** `erd1ld6er5zpdze3cynzkapur9qhzh826jje6n87g7tvdftszs8jn2qv44nqd`
- **Ethereum:** `0x8270685220fAd7EDb232E649EeE9D8810dac1d58`

---

## 2. Wallet Setup

Create a wallet on testnet/devnet and use it to solve the following tasks. Call the Faucet to get tokens.

---

## 3. Issue an NFT Collection (25p)

1. **Issue the collection (15p):**
   - **Token Name:** `BPDAExamNFT`
   - **Token Ticker:** `BPDANFT`
2. **Create an NFT (10p):**
   - Based on the issued collection, create an NFT.
   - Set the royalties to **7%**.
   - Send the NFT to the `$TEACHER_ADDR`.

---

## 4. Issue a Fungible Token (35p)

1. **Issue the token (20p):**
   - **Token Name:** `BPDAExamFT`
   - **Token Ticker:** `BPDA`
   - **Initial supply:** 1991 tokens
   - **Number of decimals:** 18
2. **Mint additional tokens (5p):**
   - Mint an additional **9 tokens**.
3. **Transfer (10p):**
   - Send **500 tokens** to the `$TEACHER_ADDR`.

---

## 5. Token Marketplace Smart Contract (70p)

Write a smart contract for a Token Marketplace. The smart contract should allow users to list, buy, and sell fungible or non-fungible tokens.

### a. Token Listing (25p)

Users can list their tokens for sale by specifying:
- Token type (Fungible or NFT).
- Token identifier.
- Quantity (for fungible tokens).
- Price per token.

**Endpoints:**
1. `listToken`: Allows a user to list their token for sale.
   - **Inputs:** `tokenId`, `quantity`, `price`.
2. `removeListing`: Allows the seller to remove their listing.
   - **Inputs:** `listingId`.
3. `getListings`: Returns all active token listings.
4. `getMyListings`: Returns all token listings created by the current user.

### b. Token Purchase (25p)

Buyers can purchase tokens by sending the specified amount in the blockchain's native currency (e.g., EGLD, ETH). Ensure proper fund transfer and ownership update.

**Endpoints:**
1. `buyToken`: Allows a user to buy a token by specifying the listing ID and quantity (if applicable).
   - **Inputs:** `listingId`, `quantity`.
2. `getPurchasedTokens`: Returns the list of tokens purchased by the user.

### c. Events (20p)

Define corresponding events for transparency and notifications:
1. `TokenListed` – Emitted when a token is listed for sale.
2. `TokenPurchased` – Emitted when a token is successfully purchased.
3. `ListingRemoved` – Emitted when a token listing is removed.
4. `AuctionCreated` – (Optional) Emitted when an auction is created.
5. `BidPlaced` – (Optional) Emitted when a bid is placed.
6. `AuctionEnded` – (Optional) Emitted when an auction ends successfully.
