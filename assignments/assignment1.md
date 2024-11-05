# Trading Card Games

## Deadline

- TODO

## Objective
The aim of this assignment is to familiarize you with interacting with smart contracts deployed on a blockchain, specifically the MultiversX blockchain. Through this assignment, you will learn how to interact with a deployed smart contract, manage, and exchange Card Games (Non-Fungible Tokens - NFTs).

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
the available NFTs' data stored in the SC, and allow a user of your API to then trade
a NFT they own with one of the NFTs which is stored inside the contract, that exactly matches the class, power and rarity attributes.

A complete flow would look like this.

- Through your API, query the SC and see the available NFTs
- Call the appropriate function from the SC that will return the NFT you should try to trade with. Each NFT has:
  - power: Medium
  - class: Priest
  - rarity: Legendary
- We will call it BPDA-NFT
- Once the contract assigned your BPDA-NFT, you need to create your own NFT that must be sent to the contract in order to receive the SC's NFT. Let's call it Student-NFT
- In order to receive the BPDA-NFT, the Student-NFT must be created with exactly the same
attributes as the BPDA-NFT, otherwise the SC will tell you that the NFT's do not match.
- Note: the Student-NFT must be named with your moodle account. For example, if your institutional account is cristian.paris@stud.acs.upb.ro, your moodle account is **cristian.paris**

## Assignment Tasks

### 1. Setup Environment:
- Set up a MultiversX wallet to be used for devnet interactions (you do not need to create a new wallet if you already have one);
- Install mxpy (MultiversX tool based on Python SDK) for interacting with the blockchain.
- Basically, make sure you have done the [Setting up the Lab Environment](https://cs-pub-ro.github.io/blockchain-protocols-and-distributed-applications/Practical%20Sessions/Env%20Setup/setup) section.

### 2. Interacting with the Smart Contract:
- **Smart Contract Address**: **erd1qqqqqqqqqqqqqpgqu7ga346e5nfnmsxcm8hxpgthvx86paw5uvaqh4f3xf**
- **Smart Contract Code**: [See the assignment for the first homework](https://github.com/cs-pub-ro/blockchain-protocols-and-distributed-applications/tree/main/assignments)

#### Core Interactions:
- **Query Smart Contract Data**: Query the smart contract for available NFTs and their metadata.
  - **Function to call**: See the view functions inside the SC. You may also need to use the Python/Rust/TS/JS/etc SDK to gather more information about the NFT's, after you get the tokenId.
  - **Return**: A list of NFTs with details such as token ID, rarity, and owner.

- **Get Your Assigned NFT**: Call the `getYourNftCardProperties` endpoint to receive the properties of the NFT you have to trade with.

- **Get the NFT nonce**: Query the smart contract for the available NFTs, parse the data and then get the NFT's nonce. You will need the nonce for the exchange part.

- **Exchange NFTs**: Implement a function to exchange an NFT with another user. This simulates a trading card game scenario where players exchange cards.
  - **Function to call**: See the `exchangeNFT(nonce)` inside the SC
  - **Requirements**: Make sure the NFT you are going to send has your moodle ID as name and the exact attributes as the one you are trying to trade with.

#### Note
You may also deploy the homework's SC for local testing.

### 3. Handling NFT Metadata:
- Each NFT will contain attributes such as rarity, strength, and card type (e.g., Warrior, Mage, etc.).
- Query this metadata using the view functions to display it within your application.

### 4. Frontend Application (BONUS 1):
Develop a simple front-end (React or other frameworks) that allows users to interact with the smart contract through a web interface.

The front-end should:
- Display available NFTs.
- Allow users to trade their NFTs with others.
- Show the history of previous trades.

### 5. Bug Hunting (BONUS 2):
Present a reproducible way of exploiting the SC or generating unwanted behavior.

## Grading Criteria
- **Code Functionality (80%)**: Correct interaction with the smart contract, querying data, and executing NFT exchanges.
- **README (20%)**: Explanation of contract interactions, NFT trading logic, and challenges faced.
- **Bonus 1 - UI (20%)**: If a front-end is developed
- **Bonus 2 - Bug Hunting (10%)**: During the lab, reproduce the bug or exploit.

Note: It is not enough to show the lab assistant the code. During the lab, you must also do a demo and show that the NFT with your moodle ID is successfully transferred to the SC, and that you get one of the BPDA trading cards.

## Notes
- **Testing**: The assignment will be tested on the MultiversX Devnet. Ensure that you use the appropriate network when deploying and interacting with the contract.
- **Collaboration**: This is an individual assignment. Each student must submit its own code. Also, each student must present their GitHub during the lab and show that the last changes have been committed before the deadline.
- **Questions and Help**: Please ask any questions on the forum.

## Useful Resources
- [The official docs](https://docs.multiversx.com/)
- [Course site](https://cs-pub-ro.github.io/blockchain-protocols-and-distributed-applications/)
