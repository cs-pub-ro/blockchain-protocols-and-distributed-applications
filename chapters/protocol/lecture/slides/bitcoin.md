---

## Bitcoin & Blockchain Protocol (V1)

**Decentralized Digital Cash & Nakamoto Consensus**

---

### Agenda

1. **The Digital Currency Problem & Cypherpunk Origins**
2. **Core Architecture & Block Anatomy**
3. **The UTXO Model & Transaction Lifecycle**
4. **Cryptography & Bitcoin Script Execution**
5. **Proof-of-Work & Nakamoto Consensus**
6. **Security, Game Theory & 51% Attacks**
7. **Scalability Limits & Layer 2 (Lightning)**

---

### Pre-Bitcoin Digital Cash Pioneers

- <!-- .element: class="fragment" -->**David Chaum (DigiCash, 1989):** Blind signatures for privacy, but relied on a centralized bank mint.
- <!-- .element: class="fragment" -->**Adam Back (Hashcash, 1997):** Proof-of-Work (PoW) computation to throttle email spam / DoS.
- <!-- .element: class="fragment" -->**Wei Dai (b-money, 1998) & Nick Szabo (Bit Gold, 1998):** Distributed ledgers with computational cost, but lacked decentralized consensus on state order.
- <!-- .element: class="fragment" -->**The Missing Link:** A Byzantine Fault Tolerant mechanism to prevent **Double-Spending** without trusted intermediaries.

---

### The Double-Spending Problem

- <!-- .element: class="fragment" -->Digital files are costless to copy: `Alice` sends 1 BTC file to `Bob`, then copies the same file to `Charlie`.
- <!-- .element: class="fragment" -->**Centralized Solution:** Bank ledger tracks balances and prevents duplicate transfers (Single Point of Failure / Trust).
- <!-- .element: class="fragment" -->**Decentralized Solution (Bitcoin):** An open, append-only, cryptographically linked ledger verified by all peer nodes in a P2P network.

---

### Satoshi Nakamoto & The Genesis Block

- <!-- .element: class="fragment" -->**October 31, 2008:** Whitepaper published: *"Bitcoin: A Peer-to-Peer Electronic Cash System"*.
- <!-- .element: class="fragment" -->**January 3, 2009:** Genesis Block (Block #0) mined by Satoshi Nakamoto.
- <!-- .element: class="fragment" -->**Genesis Message:** *"The Times 03/Jan/2009 Chancellor on brink of second bailout for banks"* (embedded in Coinbase).
- <!-- .element: class="fragment" -->**Core Philosophy:** Mathematical scarcity (21M cap), sovereign ownership, censorship resistance.

---

### Bitcoin Network Topology & Node Roles

- <!-- .element: class="fragment" -->**P2P Gossip Network:** Flat, unstructured TCP/IP overlay network (default port 8333).
- <!-- .element: class="fragment" -->**Full Nodes (e.g., Bitcoin Core):**
  - Maintain full transaction history (~600+ GB)
  - Independently validate every transaction & block against consensus rules
- <!-- .element: class="fragment" -->**Miners:** Aggregate valid pending transactions from the mempool, construct candidate blocks, execute Proof-of-Work.
- <!-- .element: class="fragment" -->**SPV (Light) Clients:** Store only 80-byte block headers; verify payments using Merkle inclusion proofs.

---

### Anatomy of a Bitcoin Block

A block consists of a **Header** (80 bytes) and a **Body** (array of serialized transactions):

| Field | Size | Description |
| :--- | :--- | :--- |
| **Version** | 4 bytes | Protocol & feature version flags |
| **Prev Block Hash** | 32 bytes | 256-bit hash of previous block header |
| **Merkle Root** | 32 bytes | 256-bit cryptographic root of all txs in block |
| **Timestamp** | 4 bytes | Unix epoch time when block was assembled |
| **Difficulty Target (Bits)** | 4 bytes | Compact encoding of the current PoW target |
| **Nonce** | 4 bytes | 32-bit counter incremented by miners |

---

### Merkle Trees & SPV Proofs

```
                 [ Merkle Root (Hash0123) ]
                       /            \
             [ Hash01 ]              [ Hash23 ]
              /      \                /      \
          [Hash0]   [Hash1]       [Hash2]   [Hash3]
             |         |             |         |
           [TX0]     [TX1]         [TX2]     [TX3]
```

- <!-- .element: class="fragment" -->Constructed using double SHA-256: `Hash01 = SHA256(SHA256(Hash0 || Hash1))`.
- <!-- .element: class="fragment" -->**Tamper Evidence:** Modifying even a single bit in `TX2` changes `Hash2`, `Hash23`, and the `Merkle Root`.
- <!-- .element: class="fragment" -->**SPV Verification:** To prove `TX2` is included, a light node only needs `[Hash3, Hash01]` $\to O(\log_2 N)$ hashes instead of downloading all $N$ transactions.

---

### UTXO vs. Account-Based Model

| Feature | UTXO Model (Bitcoin) | Account Model (Ethereum, Solana, MultiversX) |
| :--- | :--- | :--- |
| **State Storage** | Set of unspent transaction outputs (`UTXO set`) | Global state mapping `Address -> Balance, Nonce, Code` |
| **Balance** | Computed as $\sum \text{UTXO}$ controlled by user keys | Stored explicitly as a number in account state |
| **Parallelism** | High: Independent UTXOs can be spent concurrently | Lower: Sequential nonces per account create race conditions |
| **Privacy** | Better: Encourages new address per transaction | Lower: Single address re-used across transactions |

---

### Anatomy of a Transaction

```
======================================================
               BITCOIN TRANSACTION
------------------------------------------------------
 INPUTS:
   [0] Outpoint: (TXID_A, vout: 0) | ScriptSig (Unlock)
   [1] Outpoint: (TXID_B, vout: 1) | ScriptSig (Unlock)

 OUTPUTS:
   [0] Value: 1.5 BTC  | ScriptPubKey (Lock -> Bob)
   [1] Value: 0.4 BTC  | ScriptPubKey (Lock -> Alice Change)
------------------------------------------------------
 Implicit Fee: (Total Inputs) - (Total Outputs) = 0.1 BTC
======================================================
```

- <!-- .element: class="fragment" -->**Inputs consume existing UTXOs completely** (no partial spending).
- <!-- .element: class="fragment" -->**Change Output:** Excess funds are routed back to the sender via a newly generated change address.
- <!-- .element: class="fragment" -->**Transaction Fee:** Difference between inputs and outputs awarded to the miner.

---

### The Coinbase Transaction & Halving Schedule

- <!-- .element: class="fragment" -->**Coinbase Transaction ($TX_0$):** First transaction in every block; generates brand new coins without consuming inputs.
- <!-- .element: class="fragment" -->**Block Reward** = Block Subsidy + Transaction Fees.
- <!-- .element: class="fragment" -->**Halving Schedule:** Block subsidy cuts in half every 210,000 blocks (~4 years):

| Era | Years | Block Subsidy | Cumulative Supply |
| :--- | :--- | :--- | :--- |
| **Era 1** | 2009 - 2012 | 50.00 BTC | 10.50M (50%) |
| **Era 2** | 2012 - 2016 | 25.00 BTC | 15.75M (75%) |
| **Era 3** | 2016 - 2020 | 12.50 BTC | 18.375M (87.5%) |
| **Era 4** | 2020 - 2024 | 6.25 BTC | 19.687M (93.75%) |
| **Era 5** | 2024 - 2028 | 3.125 BTC | 20.343M (96.875%) |
| **Cap** | ~2140 | 0 BTC (fees only) | **21,000,000 BTC** |

---

### Keys & Addresses

- <!-- .element: class="fragment" -->**Private Key ($k$):** 256-bit random integer ($k \in [1, 2^{256}-1]$).
- <!-- .element: class="fragment" -->**Public Key ($K$):** Point on elliptic curve `secp256k1`:
  $$K = k \cdot G \quad (y^2 = x^3 + 7 \pmod p)$$
- <!-- .element: class="fragment" -->**Bitcoin Address ($A$):**
  $$A = \text{Base58Check}(\text{RIPEMD160}(\text{SHA256}(K)))$$
- <!-- .element: class="fragment" -->**Why Hash Public Keys?**
  - Address brevity (160 bits vs 512/256 bits)
  - Added security layer against quantum attacks before spending

---

### Bitcoin Script Engine

- <!-- .element: class="fragment" -->**Stack-Based Language:** Bytecode instructions executed on a LIFO (Last-In-First-Out) stack.
- <!-- .element: class="fragment" -->**Turing-Incomplete by Design:**
  - No loops (`for`, `while`) and no `GOTO` statements.
  - Guaranteed termination $\to$ Prevents Infinite Loop DoS attacks without needing gas metering.
- <!-- .element: class="fragment" -->**Validation Rule:**
  $$\text{Execute } [\text{ScriptSig} \ || \ \text{ScriptPubKey}]$$
  $$\text{Result: Top stack value must be non-zero (TRUE)}$$

---

### Standard Script: P2PKH (Pay to Public Key Hash)

**Locking Script (ScriptPubKey):**
`OP_DUP OP_HASH160 <PubKeyHash> OP_EQUALVERIFY OP_CHECKSIG`

**Unlocking Script (ScriptSig):**
`<Signature> <PublicKey>`

**Step-by-Step Stack Execution:**

| Step | Operation | Stack Content |
| :--- | :--- | :--- |
| 1 | Push `<Sig>`, `<PubKey>` | `[<Sig>, <PubKey>]` |
| 2 | `OP_DUP` | `[<Sig>, <PubKey>, <PubKey>]` |
| 3 | `OP_HASH160` | `[<Sig>, <PubKey>, HASH160(PubKey)]` |
| 4 | Push `<PubKeyHash>` | `[<Sig>, <PubKey>, ComputedHash, TargetHash]` |
| 5 | `OP_EQUALVERIFY` | `[<Sig>, <PubKey>]` (Fails if hashes mismatch) |
| 6 | `OP_CHECKSIG` | `[TRUE]` (1) if ECDSA signature matches |

---

### Advanced Scripts: Multisig & P2SH

- <!-- .element: class="fragment" -->**Multisig ($M$-of-$N$):** Requires $M$ valid signatures from a pool of $N$ authorized public keys.
  - `M <PubKey1> <PubKey2> ... <PubKeyN> N OP_CHECKMULTISIG`
- <!-- .element: class="fragment" -->**P2SH (Pay-to-Script-Hash, BIP 16):**
  - Sender pays to the hash of a script (`RedeemScript`): `OP_HASH160 <ScriptHash> OP_EQUAL`.
  - Recipient provides unlocking parameters + full `RedeemScript` at spending time.
  - Enables complex custody rules, escrow, and Lightning payment channels.

---

### The Proof-of-Work Mining Puzzle

- <!-- .element: class="fragment" -->Miners must find a `nonce` (and `extraNonce` in coinbase) such that:
  $$\text{SHA256}(\text{SHA256}(\text{BlockHeader})) < \text{Target}$$
- <!-- .element: class="fragment" -->**Target:** A 256-bit number. The lower the Target, the more leading zeros required $\to$ higher difficulty.
- <!-- .element: class="fragment" -->**Progress-Free (Memoryless) Poisson Process:**
  - Finding a valid hash is an independent Bernoulli trial with probability $p = \frac{\text{Target}}{2^{256}}$.
  - A miner with 10% of total network hashrate has a 10% chance of solving the next block, regardless of prior attempts.

---

### Difficulty Adjustment Algorithm (DAA)

- <!-- .element: class="fragment" -->**Target Cadence:** Exactly 1 block every 10 minutes on average ($2016 \text{ blocks} = 20,160 \text{ minutes} = 2 \text{ weeks}$).
- <!-- .element: class="fragment" -->**Adjustment Formula (evaluated every 2016 blocks):**
  $$\text{Target}_{\text{new}} = \text{Target}_{\text{old}} \times \frac{\text{Actual Time for last 2016 blocks}}{20,160 \text{ minutes}}$$
- <!-- .element: class="fragment" -->**Damping Filter:** Max adjustment is clamped by a factor of 4:
  $$0.25 \le \frac{\text{Target}_{\text{new}}}{\text{Target}_{\text{old}}} \le 4.0$$
- <!-- .element: class="fragment" -->Ensures monetary emission stability regardless of whether hashrate explodes or drops drastically.

---

### Nakamoto Consensus & The Longest Chain Rule

```
                   [Block 101] --- [Block 102A] (Orphaned / Stale)
                  /
[Block 100] -----
                  \
                   [Block 101] --- [Block 102B] --- [Block 103] (Canonical Chain)
```

- <!-- .element: class="fragment" -->**Fork Resolution:** If two miners find Block 101 simultaneously, nodes temporarily follow the first one received.
- <!-- .element: class="fragment" -->**Heaviest Chain (Longest Chain) Rule:** Nodes always switch to the branch with the highest cumulative Proof-of-Work.
- <!-- .element: class="fragment" -->When Block 103 is mined on branch B, branch A is abandoned (**stale/orphan block**). Unconfirmed txs return to the mempool.

---

### Probabilistic Finality & 6 Confirmations

- <!-- .element: class="fragment" -->Bitcoin consensus provides **probabilistic finality**, not deterministic finality.
- <!-- .element: class="fragment" -->Probability $P$ that an attacker with hash share $q < 0.5$ can secretly replace $z$ confirmed blocks:
  $$P \approx \sum_{k=0}^{\infty} \frac{\lambda^k e^{-\lambda}}{k!} \left( \frac{q}{p} \right)^{\max(z-k, 0)}, \quad \lambda = z \frac{q}{p}$$
- <!-- .element: class="fragment" -->**6 Confirmations Rule (~60 min):**
  - For $q = 10\%$, probability of double-spend reversal after 6 blocks is $< 0.1\%$.
  - For $q = 30\%$, probability drops to $< 1.3\%$.

---

### Sybil Resistance via Thermodynamic Cost

- <!-- .element: class="fragment" -->**Sybil Attack:** In open networks, an adversary can spin up $100,000$ virtual node IP addresses.
- <!-- .element: class="fragment" -->**PoW Solution:** Voting power is bound to thermodynamic energy and specialized hardware (ASICs), not IP addresses.
- <!-- .element: class="fragment" -->*"One CPU, One Vote"* $\to$ Fake identities without hashpower grant zero consensus weight.

---

### The 51% Attack & Double Spending

- <!-- .element: class="fragment" -->**What a 51% attacker CAN do:**
  - Reverse recent transactions they personally made (Double Spending).
  - Censor specific transactions or addresses from entering blocks.
  - Reorganize recent blocks by mining a private chain faster than the honest network.
- <!-- .element: class="fragment" -->**What a 51% attacker CANNOT do:**
  - Steal funds from other users' addresses (cannot forge ECDSA signatures).
  - Create coins out of thin air beyond the block reward rules.
  - Modify past block rules (full nodes reject invalid blocks instantly).

---

### Other Attack Vectors

- <!-- .element: class="fragment" -->**Selfish Mining (Eyal & Sirer):** Miner withholds private blocks to force honest miners to waste energy, gaining an unfair share of rewards (profitable at $>25-33\%$ hashrate).
- <!-- .element: class="fragment" -->**Eclipse Attack:** Surrounding a target node with attacker-controlled peers to feed it a false view of the blockchain.
- <!-- .element: class="fragment" -->**Routing / BGP Hijacks:** Intercepting traffic between mining pools to cause network partitioning.

---

### The Blockchain Trilemma & Bitcoin's Bottleneck

```
                     Decentralization
                           /\
                          /  \
                         /    \
                        /      \
            Security  /__________\  Scalability
```

- <!-- .element: class="fragment" -->**Bitcoin parameters:** 1 MB block limit $\div$ 10 min block interval $\approx$ **7 TPS**.
- <!-- .element: class="fragment" -->**Why not 1 GB blocks?** Larger blocks increase propagation delays $\to$ leads to node centralization and higher orphan rates.
- <!-- .element: class="fragment" -->**Trade-off:** Bitcoin maximizes **Decentralization** and **Security**, deliberately constraining base-layer throughput.

---

### Layer 2: The Lightning Network

```
 [On-Chain: Open Channel] (2-of-2 Multisig Funding Tx)
           |
           v
 [Off-Chain: Millions of Instant HTLC Payments] (Microsecond latency, zero gas)
           |
           v
 [On-Chain: Close Channel] (Settles Net Final Balance)
```

- <!-- .element: class="fragment" -->**Payment Channels:** Alice and Bob lock funds into a 2-of-2 multisig UTXO on Layer 1.
- <!-- .element: class="fragment" -->**Hashed Time-Locked Contracts (HTLCs):** Enable routed multi-hop payments across untrusted intermediaries without counterparty risk.
- <!-- .element: class="fragment" -->**Result:** Millions of transactions per second off-chain, backed by Layer-1 cryptographic settlement.

---

### Protocol Upgrades: SegWit & Taproot

- <!-- .element: class="fragment" -->**Segregated Witness (SegWit, BIP 141 - 2017):**
  - Separated signature data (`witness`) from transaction serialization.
  - Fixed Transaction Malleability (essential prerequisite for Lightning Network).
  - Effective block capacity increased from 1 MB to 4 MB Weight Units.
- <!-- .element: class="fragment" -->**Taproot (BIP 340-342 - 2021):**
  - **Schnorr Signatures:** Linear signature aggregation ($k$-of-$n$ multisig looks identical to single-sig).
  - **MAST (Merkelized Alternative Script Trees):** Reveals only the executed branch of a complex smart script, enhancing privacy and saving block space.

---

### Key Takeaways

1. <!-- .element: class="fragment" -->**Bitcoin solved Double-Spending** in a permissionless network using Proof-of-Work and Nakamoto Consensus.
2. <!-- .element: class="fragment" -->**The UTXO Model** provides high parallelism, explicit state dependencies, and deterministic verification.
3. <!-- .element: class="fragment" -->**Bitcoin Script** is intentionally Turing-incomplete to ensure security, predictability, and DoS immunity.
4. <!-- .element: class="fragment" -->**Base-layer constraints** (7 TPS, 10 min blocks) preserve full-node decentralization, while Layer 2 (Lightning) scales execution.

---

### What's Next?

- **Session 3: Ethereum & Blockchain V2**
  - From Digital Cash to the "World Computer"
  - Turing-Complete Smart Contracts & EVM
  - The Account-Based State Model
  - Gas Metering & State Transitions

---
