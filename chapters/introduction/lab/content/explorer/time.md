# Understanding Time in Blockchains

In traditional computer systems, time is relatively straightforward, with a reliance on the system clock.
However, in the world of blockchains, time is a unique and critical concept.
Blockchains need to maintain a shared understanding of time across all participants, and this introduces some complexities.
Here's how time is managed in blockchains:

## Rounds

`Rounds` refer to the cyclic or sequential stages of consensus protocols, particularly in proof-of-stake (PoS) and delegated proof-of-stake (DPoS) blockchains.
Each blockchain has it's own length of rounds:
- MultiversX - 6 seconds;
- Ethereum - 12 seconds;
- Solana - 0.4 seconds;
- Cardano - 5 seconds;
- NEAR - 1 second;

A round is a time limit to execute a batch of transactions.

## Blocks

Every **round** a new **block** is being proposed.
That proposal can succeed or not. In case of success, a block is added to the blockchain history.
The missed blocks are due to not reaching consensus and are usually under 5%.


---
**NOTE**

For MultiversX case, _every 6 seconds starts new round_, but _not necessarly a new block gets notarized_.

---

## Epochs

This is another time measurement and it's used for larger periods of time (just like minutes, hours, years, decades, etc.).

Each blockchain has it's own length of epochs:
- MultiversX - 14.400 rounds (24 hours);
- Ethereum - 30.000 rounds (100 hours);
- Solana - 432.000 rounds (48 hours);
- Cardano - 432.000 rounds (5 days);
- NEAR - 43.200 rounds (12 hours).

## Block Timestamps

Blockchains organize transactions into blocks, and each block has a timestamp. This timestamp is crucial for several reasons:

1. **Order of Transactions:** The timestamp helps order transactions within a block. Transactions are grouped together and processed in the order in which they are added to a block. This ensures that everyone has a consistent view of the transaction history.

2. **Difficulty Adjustment:** Some blockchains, like Bitcoin, use the block timestamp to adjust the difficulty of the proof-of-work (PoW) algorithm. This keeps the block generation rate relatively constant, regardless of the total network hash rate.

3. **Block Validity:** Block timestamps are used to determine whether a block is valid. If the timestamp of a new block is too far in the future or past, it can be considered invalid.

## Practice

Check MultiversX [Explorer](https://explorer.multiversx.com/) and Ethereum [Explorer](https://etherscan.io/) and see how (almost) at every 6, respectively 12 seconds a new block appears.