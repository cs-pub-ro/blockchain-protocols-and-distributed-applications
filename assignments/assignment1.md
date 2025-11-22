# Renting a Football Field Using a Smart Contract on the Blockchain

## 1. Objectives

The purpose of this assignment is to develop a smart contract that manages the process of renting a football field: reservation, advance payment, participation, confirmation, cancellation, and final payment to the field administrator.  
The student must follow a complete development flow: writing, testing, deployment, validation, and presentation.

### Specific objectives

- Implementing real business logic inside a Smart Contract.
- Deployment on a blockchain, on a dev or test network (Ethereum, Polygon, Avalanche, Arbitrum, Solana, MultiversX, Sui etc.).
- Writing tests using the chosen framework.
- Emitting events and correctly managing contract storage (contract state).

---

## 2. Requirements

- Use of the concepts studied: storage, data structures, endpoints, events, interactions between contracts, etc.
- Use of a version control system (GitHub / GitLab).
- Code must be clearly structured and documented.
- Automated testing using a framework of choice (Hardhat, Truffle, MvX Scenarios, Sui Tests etc.) or scripts that call the contract’s endpoints.
- Deployment on the chosen blockchain (test or dev network).

---

## 3. Deadline and Working Period

- **Working period:** November 22 – December 7, 2025  
- **Estimated duration:** ~2 weeks  
- **Presentation and demo:** during the laboratory session

---

## 4. Automatic Evaluation

- There is no centralized automatic checker.
- Each student must present their own testing scenario, using scripts, SDK frameworks, etc.
- These must reproduce the complete logic: create slot, participate, confirm, cancel, pay, etc.

---

## 5. Grading and Scoring

- Points are awarded per method correctly implemented and tested.
- The assignment will be presented during a laboratory slot, in physical format.  
  The code presented must be the **same code uploaded in the repository before the deadline**.  
  Tests will be run on a contract deployed before the deadline.  
  If the contract is not deployed, deployment will be done using the repository code.  
  Violation → **max 80p**.
- There is no restriction on using AI/LLMs. However, we expect the final solution to deliver a higher than average quality if such tools are used, and we will grade those homeworks accordingly.

- **Total score:** 100p  
- **Bonus:** up to 20p (max total: 120p)

---

## 6. Assignment Statement

### General description

Blockchain technology provides a secure and transparent way to record every action related to renting the field.  
This ensures that payments, reservations, and cancellations are handled correctly without third-party involvement.

Smart contracts enable:

- automated rules,
- penalties,
- instant confirmations,
- transparent on-chain logs.

The task:  
Implement a smart contract that manages the rental of a football field for specific time intervals.  
The contract must allow:

- slot reservation,
- payment of a guarantee,
- adding participants,
- slot cancellation,
- reservation confirmation,
- payment to the field administrator.

All actions must emit events and store relevant data on-chain.

---

## 7. Detailed Technical Requirements (100p)

### 7.1. Deploy contract on testnet (10p)

- Contract must be compiled and deployed on chosen testnet/devnet.
- Contract address must be saved in repository.
- Requirements may not cover all flows; students may add extra logic.

---

### 7.2. Storage (10p)

The contract must define the following data structures (according to the chosen framework):

- `football_field_manager_address`
- `football_court_cost`
- `participants`
- `reserved_slot: Slot` or optional  
  Structure: `{ start, end, payer, amount, confirmed }`

Note: these are just some general guidelines, you can also define your own fields and data structures if that helps you.

---

### 7.3. Endpoint: createFootballSlot (15p)

#### Behavior

This function lets a user start a new football game session.
The caller sends a small fixed deposit; if no session already exists, the contract records the new slot, registers the caller as the initiator/first participant, and logs that a new session was created.

---

### 7.4. Endpoint: participateToFootballSlot (10p)

This allows another user to join an already created session.
They pay the same fixed deposit, and if everything is valid (session exists, user not already joined), they are added to the participant list and the action is logged.

---

### 7.5. Endpoint: cancelFootballSlot (10p)

The session creator can cancel the session before it is confirmed.
When this happens, everyone who joined gets their deposit back, and the session data is completely cleared. A cancellation event is recorded.

---

### 7.6. Endpoint: setFootballFieldManager (10p)

The contract owner can assign the field manager — the entity who ultimately receives the payment.
Once set, the contract records this assignment and emits an event confirming the change.

---

### 7.7. Endpoint: payCourt (20p)

This function transfers the accumulated funds to the previously assigned field manager.
It performs basic checks (manager and price must be set, enough funds collected, slot must exist) and then executes the payment, logging the transfer.

---

### 7.8. Endpoint: setFootballCourtCost (5p)

The owner defines the overall field rental cost.
This value is stored and later used when making the final payment to the manager.

---

### 7.9. Endpoint: confirmSlot (10p)

The manager (or owner) confirms that the session is valid and approved.
Once confirmed, the slot is marked accordingly so that no further changes (like cancelation or re-confirmation) can occur.

---

### 7.10. Endpoint: getSlotStatus (5p)

Returns:

- slot data
- participants
- accumulated amount
- confirmation status

---

## 8. Bonus (max 20p)

Any other extra functionality may count as bonus (ask TA if needed).

### B1. Simple web interface (UI dApp) – up to 10p

- Forms for reservation / participation.
- Wallet connection.

### B2. Complete unit tests – up to 10p

- Full coverage, including edge cases.

### B3. Penalty logic (partial refund) – 10p

Examples:

- cancellation < 1h → 0% refund  
- cancellation 12–24h → 50% refund

### B4. Live multi-user demo – 10p
