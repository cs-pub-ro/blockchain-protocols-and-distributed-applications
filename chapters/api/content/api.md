# Exposing and Querying Blockchain Data

While this lab utilizes the MultiversX network to demonstrate practical implementations, the fundamental principles of data exposure and querying—such as the distinction between direct VM interaction and off-chain indexing—are applicable across most blockchain architectures.

This section outlines the specific architecture and methods used within the MultiversX ecosystem to fetch both real-time and historical data..

## Objectives

By the end of this session, you will be able to:

* Understand how **Observers** facilitate **VM Queries** without participating in consensus.
* Navigate the **ElasticSearch** indices to find specific data fields.
* Differentiate between the **Gateway (Proxy)** and the **API** layer.
* Use the API to perform advanced filtering on transactions and account data.

---

## 1. Real-Time State (VM Queries) & Observers

As experimented with in the previous **dApps** labs, interacting with a Smart Contract doesn't always require a gas-consuming transaction.

To read the **current state** of a contract (e.g., a counter value, a storage variable), we use **VM Queries**.

### How it works: The Role of Observers

The MultiversX network consists of **Validators** (who participate in consensus and produce blocks) and **Observers**.

* **Observers** are nodes that **do not participate in consensus** (they don't produce blocks).
* However, they execute every transaction to stay **in sync** with the network state.
* Because they hold the full state of the blockchain locally, they offer an endpoint to execute **Read-Only** transactions.

When you perform a VM Query (via SDK or dApp), you are asking an Observer to simulate the execution of a function locally. The Observer runs the code, retrieves the data from its local storage, and returns the result immediately.

> **Note:** This action allows you to see the "live" state of the contract without broadcasting a transaction to the whole network.

---

## 2. Data Indexing with ElasticSearch

While VM Queries are perfect for *current* data, they cannot easily search through history. For this, MultiversX uses an off-chain indexing solution based on **ElasticSearch**.

Nodes send data (blocks, transactions, logs) to an ElasticSearch cluster, where it is indexed for fast retrieval.

### Example: Retrieving Balance History

For example, if you want to search for the historical balance of an address, you have two options:

1.  **Use the ElasticSearch Index:** You can query the [`accountshistory`](https://docs.multiversx.com/sdk-and-tools/indices/es-index-accountshistory) index directly.
2.  **Use the Public API:** Alternatively, you can use the API endpoint directly:
    `https://api.multiversx.com/accounts/:address/history`

### Investigating Indices

To master data retrieval, you must **go through the available indices** in the documentation. This is crucial for obtaining detailed explanations about the data offered, understanding what each specific field represents, and learning how to formulate precise queries based on those fields.

> **Reference:** [MultiversX Docs: ElasticSearch](https://docs.multiversx.com/sdk-and-tools/elastic-search)

<!-- <div align="center">
  <img src="../media/elastic_screenshot.png" alt="Elastic Search Structure" />
  <br />
  <em>(Make sure to check the fields available in the <code>transactions</code> and <code>sc-results</code> indices)</em>
</div> -->

<!-- --- -->

## 3. The MultiversX API System

When building an application, you rarely query ElasticSearch directly. Instead, you use the **MultiversX API**, which acts as an aggregation layer.

### Architecture: Gateway vs. API

There are two distinct layers for accessing network data:

1.  **The Gateway (Proxy):**
    * This is a direct interface to the MultiversX Nodes.
    * It provides **raw data** (protocol level).
    * It is used for broadcasting transactions and basic VM queries.
2.  **The API (`api.multiversx.com`):**
    * This layer sits **on top** of the Gateway and ElasticSearch.
    * It **aggregates** data from multiple sources.
    * It adds **caching** for performance.
    * It provides "human-friendly" data (e.g., decoding Smart Contract results).

### Practical Use Cases

You should primarily use the API for your dApps. Below are examples of how to retrieve specific data using the Devnet API.

#### Use Case A: Filtering Transactions
If you want to find all transactions sent by a specific address, but exclude smart contract results (internal transactions), you can use API filters.

**Request:**
`GET /transactions?sender=[ADDRESS]&withScResults=false&status=success`

#### Use Case B: Getting Specific Account Data
To get details about an address, including its token balances (ESDTs) and nonce, without querying the raw node state.

**Request:**
`GET /accounts/[ADDRESS]`

> The API automatically queries ElasticSearch for the history and the Gateway for the real-time balance, merging them into a single response.

**Reference:** [MultiversX API Documentation](https://docs.multiversx.com/sdk-and-tools/rest-api/multiversx-api)