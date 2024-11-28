# Never Sea Festival Smart Contract

You are the NeverSea Festival 2025 organizers and you decide to create the registration via blockchain.
Starting from Smart Contract template you have to add more features to coordinate the event.

You can use the empty SC template:
```bash
$ sc-meta new --name my_neversea_2025 --template empty
[...]

$ ll my-neversea-2025/
total 16
drwxr-xr-x@  8 costincarabas  staff   256B Nov  6 04:21 .
drwxr-x---+ 54 costincarabas  staff   1.7K Nov  6 04:21 ..
-rw-r--r--@  1 costincarabas  staff   343B Nov  6 04:21 Cargo.toml
drwxr-xr-x@  4 costincarabas  staff   128B Nov  6 04:21 meta
-rw-r--r--@  1 costincarabas  staff    26B Nov  6 04:21 multiversx.json
drwxr-xr-x@  3 costincarabas  staff    96B Nov  6 04:21 scenarios
drwxr-xr-x@  3 costincarabas  staff    96B Nov  6 04:21 src
drwxr-xr-x@  4 costincarabas  staff   128B Nov  6 04:21 tests
```

There are several other templates:
```bash
$ sc-meta templates
empty
ping-pong-egld
crypto-zombies
adder
```

## Compile the Smart Contract template

To check that the contract was **successfully built**, verify that there was a **wasm** (WebAssembly) file generate: **output/your-contract-name.wasm**. This is the compiled code of your contract.

---
**NOTE**

For any further actions, please check the compilation on your local machine (and the deployment on the explorer, if it's the case). Do not assume that the contract was successfully compiled and deployed.

Any modification of the contract must be succeeded by a compilation and deployment!
---


## Constructor

Implement the `init` function so that it will take an argument that it's the registration fee and set a `registration_fee` storage.
Implement the `upgrade` function so that it won't take any argument.

## Storages

Add 2 storages:
* `participants` - a set of addresses (you can use `ManagedAddress`) that stores the participants list;
* `registration_fee` - you can use `BigUint` to store the fee.

## Add endpoints

Add a `register` endpoint where clients will pay the fee via EGLD to register to the festival.
Add a `update_registration_fees` endpoint where the owner can update the fee. Use `#[only_owner]` annotation so that the endpoint can be called only by the owner.


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
Use `#[only_owner]` endpoint annotation.

---