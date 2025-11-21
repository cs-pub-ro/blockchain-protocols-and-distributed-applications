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

The contract must define:

- `football_field_manager: ManagedAddress`
- `football_court_cost: BigUint`
- `participants: MultiValueMapper<ManagedAddress>`
- `reserved_slot: Slot` or optional  
  Structure: `{ start, end, payer, amount, confirmed }`

---

### 7.3. Endpoint: createFootballSlot (15p)

#### Behavior

- Receives exactly **0.1 EGLD** (or equivalent).
- Creates a new slot.
- Adds caller to participants.
- Stores slot data.
- Emits: `slot_created(caller)`.

#### Valid flow

- Slot does not exist → can be created.
- Exact payment → success.

#### Invalid cases

- Slot already exists.
- Amount ≠ 0.1 EGLD  
  *(Bonus: refund difference if amount > expected)*.
- Invalid data (e.g., wrong time interval).

---

### 7.4. Endpoint: participateToFootballSlot (10p)

#### Behavior

- Receives exactly **0.1 EGLD**.
- Adds caller to participants.
- Emits event: `participant_added(caller)`.

#### Invalid cases

- Slot does not exist.
- Incorrect payment amount.
- Caller already participating.

---

### 7.5. Endpoint: cancelFootballSlot (10p)

#### Behavior

- Refunds all participants.
- Clears storage (slot + participants).
- Emits: `slot_canceled()`.
- Callable only by the slot creator.

#### Invalid cases

- Slot does not exist.
- Insufficient contract balance.

---

### 7.6. Endpoint: set_football_field_manager (10p)

#### Behavior

- Sets the manager address.
- Only owner can call.
- Emits corresponding event.

#### Invalid cases

- Caller is not owner.
- Invalid address.

---

### 7.7. Endpoint: payCourt (20p)

#### Behavior

- Sends to manager the amount stored in `football_court_cost`.
- Emits event: `court_paid(manager, amount)`.

#### Invalid cases

- Manager not defined.
- Cost not defined.
- Insufficient contract balance.
- Slot does not exist.

---

### 7.8. Endpoint: setFootballCourtCost (5p)

- Only owner.
- Sets total field cost.
- Fail if zero cost or unauthorized caller.

---

### 7.9. Endpoint: confirmSlot (10p)

- Confirms reservation.
- Only owner/manager.
- Slot must exist and must not be confirmed.

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
