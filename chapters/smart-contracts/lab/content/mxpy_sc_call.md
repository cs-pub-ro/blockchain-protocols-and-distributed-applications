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
```
/mvx/mx-contracts-rs/contracts/adder/interaction$ add 
Enter number: 2
INFO     utils: View this transaction in the MultiversX Testnet Explorer: https://testnet-explorer.multiversx.com/transactions/2a906bc468008706f928cc5b7a669570c71556c604ae8d1ef2dfdfb100636f74                         
{
    "emittedTransaction": {
        "nonce": 313,
        "value": "0",
        "receiver": "erd1qqqqqqqqqqqqqpgq9ss82g55h3k96898kmdtp8am3a4qaefcuvaqutqjjd",
        "sender": "erd1mqa9wttlzwwdvwgk9dzsfdn79lv5raw0tfe9ynvn0dg92hpruvaqhhd2gx",
        "senderUsername": "",
        "receiverUsername": "",
        "gasPrice": 1000000000,
        "gasLimit": 5000000,
        "data": "YWRkQDAy",
        "chainID": "T",
        "version": 2,
        "options": 0,
        "guardian": "",
        "signature": "187f19d7daad9b75f3f6469b56698ab84442786f4a592f4cb86efb858d40d63e75307964a4dd838bf114303427e3e4d2c9322936de3ee520c0ee9878bd2a8c0b",
        "guardianSignature": ""
    },
    "emittedTransactionData": "add@02",
    "emittedTransactionHash": "2a906bc468008706f928cc5b7a669570c71556c604ae8d1ef2dfdfb100636f74",
    "contractAddress": "erd1qqqqqqqqqqqqqpgq9ss82g55h3k96898kmdtp8am3a4qaefcuvaqutqjjd"
}
```

Inspect the MultiversX Testnet Explorer to see the transaction.

Let's call the `getSum` view function to read from the storage:

```bash
costin@Byblos:~/mvx/mx-contracts-rs/contracts/adder/interaction$ getSum 
[
    "02"
]
```

Notice that the view functions (the queries) are not transactions on blockchain. We didn't pay any gas for this. We didn't create any transaction.