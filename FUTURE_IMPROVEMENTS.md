# BPDA - Course Matrix & Future Improvements

Acest document sintetizează structura actuală a materiei **Blockchain Protocols and Distributed Applications (BPDA)** (cursuri, laboratoare, teme) și inventariază acțiunile necesare pentru actualizarea și îmbunătățirea conținutului didactic.

---

## 🗺️ Matricea Capitolelor (Cursuri vs. Laboratoare)

| # | Sesiune Curs (Teorie) | Modul Laborator (Practică) | Ecosistem / Stack | Stare Actuală | Acțiuni Necesare / Future Improvements |
|---|---|---|---|---|---|
| **01** | **Introduction to Blockchains**<br>`lectures/Session 01 - Introduction to Blockchains.pdf` | [Env Setup](chapters/introduction/lab/content/setup/setup.md)<br>[Testnet Wallets](chapters/introduction/lab/content/setup/testnet-wallets.md) | Universal (CLI, Web) | ✅ Complet | • Actualizare capturi de ecran pentru noile versiuni de extensii de portofel.<br>• Verificare funcționalitate faucet-uri testnet. |
| **02** | **Money - Current limitations**<br>`lectures/Session 02 - Money - Current limitations-2.pdf` | [Wallet Deep-Dive](chapters/introduction/lab/content/wallet/wallet.md)<br>(Keystore, CLI, Faucet, Tx) | MultiversX, Solana, Ethereum | ✅ Complet | • Verificare comenzi `mxpy` vs noile versiuni de SDK.<br>• Extindere quiz-uri pentru portofele Solana și EVM. |
| **03** | **Bitcoin & Blockchain Protocol Part 1**<br>`lectures/Session 03 - Bitcoin.pdf`<br>`lectures/Session 07 - Blockchain Protocol Part 1.pdf` | [Explorer & Time](chapters/introduction/lab/content/explorer/time.md)<br>(Blocks, Txs, Consensus, Observers) | MultiversX, Solana Explorer | ✅ Complet | • Adăugare exercițiu de comparare a exploratoarelor (SolanaFM / Solscan vs MultiversX Explorer vs Etherscan). |
| **04** | **Blockchain Protocol Part 2 / P2P**<br>`lectures/Session 04 - Blockchain Protocol Part 2.pdf` | [Basic Peer-to-Peer Blockchain](chapters/peer-to-peer/lab/content/what_we_build.md) | **Go** (`libp2p`, mining, wallet) | ✅ Complet | • Mutare din secțiunea "Extra" în programul principal al primelor săptămâni.<br>• Ghid de rulare a nodurilor locale în rețea între colegi. |
| **05** | **MultiversX Architecture**<br>`lectures/Session 05 - MultiversX.pdf` | [Learning Rust](chapters/rust/content/rust.md)<br>(Tour of Rust, CryptoZombies) | Rust | ⚠️ Parțial / Introductiv | • Înlocuire linkuri generice cu exerciții practice de Rust targetate pe Smart Contracts (ownership, memory, types, macros). |
| **06** | **Proof of Stake & Consensus**<br>`lectures/Session 06 - Proof of Stake.pdf` | [Smart Contracts Intro](chapters/smart-contracts/lab/content/prerequisites.md)<br>• [Adder SC](chapters/smart-contracts/lab/content/adder.md) (MultiversX)<br>• [Solana Hello World](chapters/smart-contracts/lab/content/solana_hello_world.md) | MultiversX (mxpy),<br>Solana CLI | 🔶 Bun, dar eterogen | • Sincronizare ghid MultiversX cu ultimele versiuni de framework.<br>• Structurare paralelă: același contract (ex: Adder) implementat pe MultiversX și Solana (Anchor). |
| **07** | **Scalability & Sharding**<br>`lectures/Session 08 - Scalability.pdf` | [SC Testing & Events](chapters/smart-contracts/lab/content/chain-sim.md)<br>• [Events](chapters/smart-contracts/lab/content/events.md)<br>• [Blackbox Testing](chapters/smart-contracts/lab/content/blackbox.md) | MultiversX (Chain-sim, Rust Interactors) | 🔶 Parțial | • Adăugare suport / ghid de testare echivalent pentru Solana (Anchor test framework / `solana-program-test`). |
| **08** | **Virtual Machines (VMs)**<br>`lectures/Session 09 - VMs.pdf` | [VM Lab](chapters/vm/lab/content/vm/README.md) | WASM, eBPF, EVM | 🔴 **TODO / Gol** | • **Prioritate:** Creare laborator practic: inspectare apeluri din loguri de nod, execuție locală de bytecode WASM / Solana eBPF. |
| **09** | **Tokens, NFTs & Tokenomics**<br>`lectures/Session 10 - Tokens, NFTs, Tokenomics.pdf` | [Tokens Lab](chapters/tokens/lab/content/standards.md)<br>• [Fungible](chapters/tokens/lab/content/fungible.md)<br>• [NFTs](chapters/tokens/lab/content/nft.md)<br>• [Solana SPL / Metaplex](chapters/tokens/lab/content/solana_spl_tokens.md) | MultiversX (ESDT),<br>Solana (SPL / Metaplex) | ✅ Complet | • Testare scripturi pe noile devnet-uri.<br>• Adăugare discuție despre Dynamic NFTs și Token Metadata extensions. |
| **10** | **Introduction to DeFi**<br>`lectures/Session 11 Introduction to DeFi.pdf` | [DeFi / Money Lab](chapters/money/lab/content/mint_tokens.md)<br>• [Swap xExchange](chapters/money/lab/content/swap.md)<br>• [Lend/Borrow](chapters/money/lab/content/lend_borrow.md) | MultiversX (xExchange, Hatom), Solana (Raydium) | ⚠️ Parțial | • **Prioritate:** Completare [`liq_provider.md`](chapters/money/lab/content/liq_provider.md) (momentan are 0 bytes).<br>• Extindere ghid de Lending/Borrowing cu pași compleți și capturi. |
| **11** | **Composability & Interoperability**<br>`chapters/composability/lecture` | [Composability Lab](chapters/composability/lab/content/README.md) | MultiversX / Cross-chain | 🔴 **TODO / Gol** | • **Prioritate:** Implementare laborator: interacțiuni cross-contract (contract calling contract), oracole de preț (Pyth / Switchboard), bridge-uri. |
| **12** | **Maximal Extractable Value (MEV)**<br>`lectures/Session 12 - Maximal Extractable Value (MEV).pdf` | [APIs & Scaling](chapters/api/content/api.md)<br>[dApps Integration](chapters/dApps/lab/content/dApp.md) | React, REST/GraphQL, Web3 APIs | 🔶 Bun pe dApps, fără MEV lab | • Laborator hands-on sau simulare demonstrativă de MEV (sandwich attack / arbitrage bot demo în mediu local).<br>• Actualizare template dApp la React 19 / Vite. |
| **13** | **Privacy & ZK on Blockchain**<br>`lectures/Session 13 - Privacy on the Blockchain.pdf` | [CTF Security Lab](chapters/ctf/ctf.md)<br>• [Bump](chapters/ctf/bump.md)<br>• [Coinflip](chapters/ctf/coinflip.md)<br>• [Gaspass](chapters/ctf/gaspass.md) | Rust / MultiversX SC Security | 🔶 Funcțional CTF, fără ZK lab | • Unificare `chapters/security` cu `chapters/ctf`.<br>• Adăugare vulnerabilități specifice Solana (missing signer check, account confusion).<br>• Exercițiu demonstrativ simplu de Zero-Knowledge (ex: zk-SNARK verifier). |

---

## 🎯 Plan de Acțiune Recomandat (Roadmap)

### Prioritatea 1: Fix-uri Critice & Închiderea Fișierelor Goale (1-2 săptămâni)
- [ ] **Completare `chapters/money/lab/content/liq_provider.md`**: Ghid pas-cu-pas pentru adăugare și retragere de lichiditate într-un pool AMM.
- [ ] **Completare `chapters/composability/lab/content/README.md`**: Exercițiu de apel sincron/asincron între două smart contracts și citire date dintr-un Oracle.
- [ ] **Rezolvare `assignments/assignment2.md`**: Decizie asupra Temei 2 (specificație temă vs. decizie de a păstra 1 temă mare + Proiect conform `util/grading.md`).
- [ ] **Curățare `config.yaml`**: Eliminarea referințelor comentate sau repararea build-ului pentru paginile neincluse în Docusaurus.

### Prioritatea 2: Consolidare & Unificare Multi-Chain (3-4 săptămâni)
- [ ] **Paralelism MultiversX - Solana**: Structurarea clară a laboratoarelor de Smart Contracts și dApps astfel încât studenții să poată alege sau compara direct paradigmele (Account-based vs UTXO/Program-Derived Accounts).
- [ ] **Unificare Slide-uri & Prelegeri**: Sincronizarea conținutului din `lectures/*.pdf` cu slide-urile interactive Reveal.js / Docusaurus.
- [ ] **Extindere Quiz-uri**: Adăugarea de întrebări interactive de tip quiz la finalul fiecărui laborator, nu doar la modulul de portofele.

### Prioritatea 3: Modernizare & Teme Avansate (Pe parcursul semestrului)
- [ ] **Extindere CTF**: Adăugarea de provocări pe Solana și EVM (Reentrancy, Integer Overflow, Access Control).
- [ ] **Modul Practic ZK / MEV**: Dezvoltarea unui demo interactiv de generare și verificare de dovezi ZK (ex: folosind Circom/SnarkJS sau Noir).
- [ ] **Tooling & Dev Environments**: Asigurarea de containere Docker / Devcontainers gata configurate cu toolchain-ul de Rust, `mxpy`, `solana-cli` și `anchor`.
