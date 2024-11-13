# Trading Card Games

## Deadline

- 20th of November 23:59

## Objective

The aim of this assignment is to familiarize you with interacting with smart contracts deployed on a blockchain, specifically the MultiversX blockchain. Through this assignment, you will learn how to interact with a deployed smart contract, manage, and exchange  Tradable Cards (Non-Fungible Tokens - NFTs).

## Learning Outcomes

By the end of this assignment, students will be able to:

- Understand and execute transactions on the MultiversX blockchain;
- Interact with a deployed smart contract by calling methods using the Python SDK (mxpy tool);
- Use NFT trading functionality provided by our smart contract;
- Query on-chain data from our smart contract and interpret results;
- Gain a better understanding of blockchain concepts while performing a transaction:
  - Private key
  - Gas limit
  - Proxy
  - Nonce

## Overview

Your job is to create an interactive API over a SC deployed on the MultiversX blockchain. This SC stores a number of NFTs. Each NFT has the following attributes:

- class
- power
- rarity

In order to solve this assignment you need to provide a way of retrieving and visualizing
the available NFTs' data stored in the SC. This should allow a user of your API to then trade
a NFT they own with one of the NFTs which is stored inside the contract. This NFT must exactly match the class, power and rarity attributes.

A complete flow would look like this.

- Through your API, query the SC and see the available NFTs
- Call the appropriate function from the SC that will return the NFT you should try to trade with. Each NFT has (for example):
  - power: Medium
  - class: Priest
  - rarity: Legendary
- Let's call it BPDA-NFT
- Once the contract assigned your BPDA-NFT properties, you need to create your own NFT that must be sent to the contract in order to receive the SC's NFT. Let's call the NFT you have to send **Student-NFT**
- In order to receive the BPDA-NFT, the **Student-NFT** must be created with exactly the same
attributes as the BPDA-NFT, otherwise the SC will tell you that the NFT's do not match.
- **Note**: the Student-NFT must be named with your moodle account. For example, if your institutional account is cristian.paris@stud.acs.upb.ro, your **Student-NFT** name is **cristian.paris**.

## Assignment Tasks

### 1. Setup Environment

- Set up a MultiversX wallet to be used for devnet interactions (you do not need to create a new wallet if you already have one);
- Install mxpy (MultiversX tool based on Python SDK) for interacting with the blockchain.
- Basically, make sure you have done the [Setting up the Lab Environment](https://cs-pub-ro.github.io/blockchain-protocols-and-distributed-applications/Practical%20Sessions/Env%20Setup/setup) section.
- It is mandatory to use the Python/Rust/TS/JS/etc SDK to solve the following tasks.

### 2. Interacting with the Smart Contract

- **Smart Contract Address**: **erd1qqqqqqqqqqqqqpgqrqz7r8yl5dav2z0fgnn302l2w7xynygruvaq76m26j**
- **Smart Contract Code**: [See the assignment for the first homework on GitHub](https://github.com/cs-pub-ro/blockchain-protocols-and-distributed-applications/tree/main/assignments/tema-1)

### Core Interactions

- **Get Your Assigned NFT**: Call the `getYourNftCardProperties` endpoint to receive the properties of the NFT you have to trade with. The properties you receive are hex encoded.
  - **Example**: Let's say you receive the following hex encoded properties: **020304**
  - Each of the bytes corresponds to one of the attributes (class, rarity, power), and they all have the same length. Each byte corresponds to the index of the enum variant of that attribute.

- **Query Smart Contract Data**: Query the smart contract for available NFTs and their metadata, parse the metadata and then get the NFT's nonce. You will need the nonce for the exchange part. The nonce is equal to the position of the metadata in the returned list.
  - **Note**: The vector indexing starts from 1.
  - **Function to call**: See the `nftSupply` view function inside the SC.
  - **Return**: A list of NFTs with details such as token ID, rarity, class, power.
  - **Note**: [Check out this link](https://docs.multiversx.com/developers/developer-reference/sc-api-functions#get_esdt_token_data) on how NFT properties are serialized inside the list of returned NFTs.

- **Exchange NFTs**: Implement a function to exchange an NFT with another user. This simulates a trading card game scenario where players exchange cards.
  - **Function to call**: See the `exchangeNFT(nonce)` inside the SC.
  - **Requirements**: Make sure the NFT you are going to send has your moodle ID as name and the exact attributes as the one you are trying to trade with.
  - **Note**: The collection name and other fields are irrelevant.

### Note

You can also deploy the homework's SC for local testing.

### How do I know if I have successfully finished the homework?

The homework is done if, at the end of the aforementioned flow, you have a **BPDA-TRADING-CARD** NFT in your MultiversX wallet. Any further attempts to call the functions `getYourNftCardProperties` or `exchangeNFT` will result in a SC error with the following message: **Congratulations! You already finished the homework!**.

### 3. Frontend Application (BONUS 1)

Develop a simple front-end (React or other frameworks) that allows users to interact with the smart contract through a web interface.

The front-end should:

- Display available NFTs.
- Allow users to trade their NFT with the SC.
- Show the history of previous trades.

### 4. Bug Hunting (BONUS 2)

Present a reproducible way of exploiting the SC or generating unwanted behavior.

## Grading Criteria

- **Code Functionality (95%)**: Correct interaction with the smart contract, querying data, and executing NFT exchanges.
- **README (5%)**: Explanation of contract interactions, NFT trading logic, and challenges faced.
- **Bonus 1 - UI (10%)**: If a front-end is developed.
- **Bonus 2 - Bug Hunting (10%)**: During the lab, reproduce the bug or exploit.

**Note**: The fact that you show the lab assistant the SC NFT inside your wallet is not enough. You will be asked questions based on your code for each task.

## Notes

- **Testing**: The assignment will be tested on the MultiversX Devnet. Ensure that you use the appropriate network when deploying and interacting with the contract.
- **Collaboration**: This is an individual assignment. Each student must submit its own code. Also, each student must present their GitHub during the lab and show that the last changes have been committed before the deadline. You will be asked to clone the repository on the spot.
- **Questions and Help**: Please ask any questions on the [Forum](https://curs.upb.ro/2024/mod/forum/view.php?id=56665).

## Useful Resources

- [The official docs](https://docs.multiversx.com/)
- [Course site](https://cs-pub-ro.github.io/blockchain-protocols-and-distributed-applications/)
