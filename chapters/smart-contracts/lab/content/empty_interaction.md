# Empty Interaction

After building the contract, there are two main ways to interact with smart contracts:

- [via `mxpy`](https://docs.multiversx.com/sdk-and-tools/mxpy/smart-contract-interactions)
- [via Rust Interactors](https://docs.multiversx.com/developers/meta/interactor/interactors-overview)

## `mxpy` interaction

In the contract's **root directory**, you will create a file with a descriptive name. For instance, for out contract, which will be uploaded to Devnet, the file might be named: `devnet.snippets.sh`.

Once the contract is built, the WASM bytecode is generated in the `output/` directory, which we will utilize. This will be used within the newly created file:

```sh
WASM_PATH="/home/empty/output/empty.wasm"
```

To perform the **deployment**, we will use the special `deploy` function provided by `mxpy`:

```sh
WALLET_PEM="/home/my-wallet.pem"
PROXY="https://devnet-gateway.multiversx.com"

deploySC() {
    mxpy --verbose contract deploy \
        --bytecode=${WASM_PATH} \
        --pem=${WALLET_PEM} \
        --gas-limit=2000000 \
        --proxy=${PROXY} \
        --send || return
}
```

To deploy the **Empty** smart contract to Devnet, run the following commands in the terminal:

```sh
source devnet.snippets.sh
deploySC
```

Observe the output logs printed in the terminal. In the response JSON body, you will find the `contractAddress` field, which contains the address of the deployed contract.

```json
{
    "emittedTransaction": {
        "nonce": 19880,
        "value": "0",
        "receiver": "erd1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq6gq4hu",
        "sender": "erd1qyu5wthldzr8wx5c9ucg8kjagg0jfs53s8nr3zpz3hypefsdd8ssycr6th",
        "senderUsername": "",
        "receiverUsername": "",
        "gasPrice": 1000000000,
        "gasLimit": 1175521,
        "data": "MDA2MTczNmQwMTAwMDAwMDAxMGQwMzYwMDAwMDYwMDAwMTdmNjAwMjdmN2YwMDAyM2UwMzAzNjU2ZTc2MGY2NzY1NzQ0ZTc1NmQ0MTcyNjc3NTZkNjU2ZTc0NzMwMDAxMDM2NTZlNzYwYjczNjk2NzZlNjE2YzQ1NzI3MjZmNzIwMDAyMDM2NTZlNzYwZTYzNjg2NTYzNmI0ZTZmNTA2MTc5NmQ2NTZlNzQwMDAwMDMwMzAyMDAwMDA1MDMwMTAwMDMwNjBmMDI3ZjAwNDE5OTgwMDgwYjdmMDA0MWEwODAwODBiMDc0MTA2MDY2ZDY1NmQ2ZjcyNzkwMjAwMDQ2OTZlNjk3NDAwMDMwODYzNjE2YzZjNDI2MTYzNmIwMDA0MDc3NTcwNjc3MjYxNjQ2NTAwMDMwYTVmNWY2NDYxNzQ2MTVmNjU2ZTY0MDMwMDBiNWY1ZjY4NjU2MTcwNWY2MjYxNzM2NTAzMDEwYzAxMDEwYTE4MDIxMjAwMTAwMjEwMDAwNDQwNDE4MDgwMDg0MTE5MTAwMTAwMGIwYjAzMDAwMTBiMGIyMTAxMDA0MTgwODAwODBiMTk3NzcyNmY2ZTY3MjA2ZTc1NmQ2MjY1NzIyMDZmNjYyMDYxNzI2Nzc1NmQ2NTZlNzQ3M0AwNTAwQDA1MDA=",
        "chainID": "D",
        "version": 2,
        "options": 0,
        "guardian": "",
        "signature": "447288b50145233850e69b80e90acf5a1b5ad637701313580f0194a99badbf8a3ac556ef54f6e9bc77163b3af83aefa12d8e092d1af95646ee003c6c7cd3530d",
        "guardianSignature": "",
        "relayer": "",
        "relayerSignature": ""
    },
    "emittedTransactionData": "0061736d01000000010d036000006000017f60027f7f00023e0303656e760f6765744e756d417267756d656e7473000103656e760b7369676e616c4572726f72000203656e760e636865636b4e6f5061796d656e74000003030200000503010003060f027f00419980080b7f0041a080080b074106066d656d6f7279020004696e697400030863616c6c4261636b0004077570677261646500030a5f5f646174615f656e6403000b5f5f686561705f6261736503010c01010a180212001002100004404180800841191001000b0b0300010b0b210100418080080b1977726f6e67206e756d626572206f6620617267756d656e7473@0500@0500",
    "emittedTransactionHash": "974b6e2a15002071a6bd7ab6cc6e06d57fad937e9deb1ec04ea3b163789b72fe",
    "contractAddress": "erd1qqqqqqqqqqqqqpgqtj7xmd533wdnf7ckh5sy8dmjd8r752qdd8ssxdmx09"
}
```

## Rust Interactor

Rust interactors are used to interact with the blockchain via Rust.

Let's do this for the Empty smart contract. In the **contract root** run the next command:

```bash
sc-meta all snippets
```

Below, it is the output of the command:

```sh
 /home/empty

Found 1 contract crates.

(1/1)
In /home/empty/meta
Calling `cargo run snippets`
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.03s
     Running `/home/empty/target/debug/empty-meta snippets`
```

A new folder `interactor` was created.

This will generate code for all the endpoints and view functions you created.

As this is a new and separate Rust binary, you must **add** the interactor to the main `Cargo.toml`'s members, which is in the contract root, at path `empty/Cargo.toml`:

As this interactor is compiled as a new and separate Rust binary, you must add its path to the main `Cargo.toml`'s [workspace] members, located at the contract root, at path `empty/Cargo.toml`.

Before:

```toml
[workspace]
members = [
    ".",
    "meta",
]
```

After:

```toml
[workspace]
members = [
    ".",
    "meta",
    "interactor"
]

```

Now, you can deploy your contract by running the following command in the `interactor` directory.

```bash
$ cargo run deploy
[...]
sender's recalled nonce: 19922
-- tx nonce: 19922
sc deploy tx hash: a17a4f51305b6f6dd9c01ec4986d0f90266ef560599b15af613e9aadd816e705
deploy address: erd1qqqqqqqqqqqqqpgqchszakc8fm44c2rndjh09xeuh829g4tgd8sskk0m5e
```

## Practice

- Create an interactor for you Empty contract.
