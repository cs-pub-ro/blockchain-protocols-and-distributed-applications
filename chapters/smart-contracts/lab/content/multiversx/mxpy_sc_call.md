# Smart Contract call

Now let's call our previous `adder` smart contract.

Inspect the `deploy` function in our `testnet.snippets.sh`. Observer that we provided the arguments: `--arguments 0`.
In our `init` function, we provided the argument and saved it in our `sum` global variable (storage):

```rust
#[init]
fn init(&self, initial_value: BigUint) {
    self.sum().set(initial_value);
}
```

Therefore in the SC there is a `sum` storage initialized with **0**.

Let's call the `add` endpoint to add a value to our storage:

```sh
adder/interaction$ add 
Enter number: 2
INFO     utils: View this transaction in the MultiversX Testnet Explorer: https://testnet-explorer.multiversx.com/transactions/fdea45d4fd03368d5dc56f763626ca12ebfc85eb859d8b92748dbcf209b010ae                        
{
    "emittedTransaction": {
        "nonce": 30202,
        "value": "0",
        "receiver": "erd1qqqqqqqqqqqqqpgq2yezuywz09j300reqsj3yvqn7lthv270d8sszpql8z",
        "sender": "erd1qyu5wthldzr8wx5c9ucg8kjagg0jfs53s8nr3zpz3hypefsdd8ssycr6th",
        "senderUsername": "",
        "receiverUsername": "",
        "gasPrice": 1000000000,
        "gasLimit": 1240948,
        "data": "YWRkQDAy",
        "chainID": "T",
        "version": 2,
        "options": 0,
        "guardian": "",
        "signature": "befbf6f1611517519d4e71ab8b9339be3f03dc840bbacfa4b223ab3289ee7a7b2ed9880aabeee2605f1e9028e65b86b31a13a44b8dacf5fa26ee674c84ae4a03",
        "guardianSignature": "",
        "relayer": "",
        "relayerSignature": ""
    },
    "emittedTransactionData": "add@02",
    "emittedTransactionHash": "fdea45d4fd03368d5dc56f763626ca12ebfc85eb859d8b92748dbcf209b010ae",
    "contractAddress": "erd1qqqqqqqqqqqqqpgq2yezuywz09j300reqsj3yvqn7lthv270d8sszpql8z"
}
```

Inspect the MultiversX Testnet Explorer to see the transaction.

Let's call the `getSum` view function to read from the storage:

```bash
adder/interaction$ getSum 
[
    "02"
]
```

Notice that the view functions (the queries) are not transactions on blockchain. We didn't pay any gas for this. We didn't create any transaction.
