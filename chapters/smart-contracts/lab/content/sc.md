# Never Sea Festival Smart Contract

You are the Never Sea Festival 2024 organizers and you decide to create the registration via blockchain.
Starting from Smart Contract template you have to add more features to coordinate the event.

## Prerequisites
You need to have mxpy installed. Follow the installation guide [here](https://docs.multiversx.com/sdk-and-tools/sdk-py/installing-mxpy/).



Clone the [Neversea](https://github.com/systems-cs-pub-ro/Foundation-Of-Blockchains/tree/master/labs/lab03/Neversea) project.

## Compile and deploy the Smart Contract template

Use [tutorial](https://docs.multiversx.com/developers/tutorials/counter/#build-the-contract) to build the smart contract.

To check that the contract was **successfully built**, verify that there was a **wasm** (WebAssembly) file generate: **output/neversea.wasm**. This is the compiled code of your contract.

To check that the contract was successfully deployed, check the devnet/testnet.

---
**NOTE**

Check the deployment on the explorer. Do not assume that the contract was successfully deployed if there are no command line errors.

Any modification of the contract must be succeeded by a compilation and deployment!

---

## Practice

* Make a contract call to register a user;
* Make a contract call to view the registered users;
* Modify the registration endpoint to enable VIP access;
* Create a new storage mapper **registration_fee_vip**;
* Create a new storage mapper **vip_participants** to save the VIP participants;
* In the registration endpoint, make a verification of the tokens received. If the tokens received is **registration_fee_vip**, add the user to **vip_participants**, if the amount is **registration_fee**, add them to participants, else, deny registration;
* Modify the registration fee to enable Early Bird and Full price access;
* Create a new endpoint that modifies the **registration_fee** and **registration_fee_vip** storage mapper. This endpoint should be call **only by the owner**.
* BONUS: Create a feature to enable 50% discount vouchers for friends and partners. Create a list of hardcoded discount codes. Create a new endpoint that receives a discount code as a parameter and registers a user with 50% discount.




---

**Hint**
Use #[only_owner] endpoint annotation.

---