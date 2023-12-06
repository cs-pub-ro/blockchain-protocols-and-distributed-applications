# Smart Contract call

Now let's call our previous `adder` smart contract.

Inspect the `deploy` function in our `testnet.snippets.sh`. Observer that we provided the arguments: `--arguments 0`.
In our `init` function, we provided the argument and saved it in our `sum` global variable (storage):

```bash
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
DEBUG:cli.contracts:call
DEBUG:accounts:AccountBase.sync_nonce()
DEBUG:urllib3.connectionpool:Starting new HTTPS connection (1): testnet-api.multiversx.com:443
DEBUG:urllib3.connectionpool:https://testnet-api.multiversx.com:443 "GET /address/erd1ld6er5zpdze3cynzkapur9qhzh826jje6n87g7tvdfrtszs8jn2qv44nqd HTTP/1.1" 200 363
DEBUG:accounts:AccountBase.sync_nonce() done: 2
INFO:transactions:Transaction.send: nonce=2
DEBUG:urllib3.connectionpool:Starting new HTTPS connection (1): testnet-api.multiversx.com:443
DEBUG:urllib3.connectionpool:https://testnet-api.multiversx.com:443 "POST /transaction/send HTTP/1.1" 201 106
INFO:transactions:Hash: 260ca64134d2d14781493959bf5b4c5046b1b4bb6aa46b76d075aa6ec170fa7b
INFO:utils:View this transaction in the MultiversX Testnet Explorer: https://testnet-explorer.multiversx.com/transactions/260ca64134d2d14781493959bf5b4c5046b1b4bb6aa46b76d075aa6ec170fa7b
{
    "emittedTransaction": {
        "nonce": 2,
        "value": "0",
        "receiver": "erd1qqqqqqqqqqqqqpgqgjdcmz3049s0g2zwm6dzfrnk5s3qwn8yjn2qltmga4",
        "sender": "erd1ld6er5zpdze3cynzkapur9qhzh826jje6n87g7tvdfrtszs8jn2qv44nqd",
        "gasPrice": 1000000000,
        "gasLimit": 5000000,
        "data": "YWRkQDAy",
        "chainID": "T",
        "version": 1,
        "signature": "d67b1da6c9151aec3950c16b37df91a7789b49edc53574152c29db12339662eaef8896566e88684b8b547f8856ee0f4afd97031ea32e49a0ea3519ca3b3e600e"
    },
    "emittedTransactionData": "add@02",
    "emittedTransactionHash": "260ca64134d2d14781493959bf5b4c5046b1b4bb6aa46b76d075aa6ec170fa7b",
    "contractAddress": "erd1qqqqqqqqqqqqqpgqgjdcmz3049s0g2zwm6dzfrnk5s3qwn8yjn2qltmga4"
}
```

Inspect the MultiversX Testnet Explorer to see the transaction.

Let's call the `getSum` view function to read from the storage:

```bash
costin@Byblos:~/mvx/mx-contracts-rs/contracts/adder/interaction$ getSum 
DEBUG:cli.contracts:query
DEBUG:urllib3.connectionpool:Starting new HTTPS connection (1): testnet-api.multiversx.com:443
DEBUG:urllib3.connectionpool:https://testnet-api.multiversx.com:443 "POST /vm-values/query HTTP/1.1" 201 525
[
    {
        "base64": "Ag==",
        "hex": "02",
        "number": 2
    }
]
```

Notice that the view functions (the queries) are not transactions on blockchain. We didn't pay any gas for this. We didn't create any transaction.