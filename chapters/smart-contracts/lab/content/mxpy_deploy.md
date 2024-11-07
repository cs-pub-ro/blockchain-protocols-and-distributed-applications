# Smart Contract deployment via Python (mxpy)

Let's deploy our smart contract on the blockchain. For this, we will use `mxpy` tools previously installed.
We will deploy the `adder` contract from the previous section.

In the [repo](https://github.com/multiversx/mx-contracts-rs/tree/main/contracts/adder/) there is a folder named [interaction](https://github.com/multiversx/mx-contracts-rs/tree/main/contracts/adder/interaction).

Let's inspect the `testnet.snippets.sh` file:

```bash
ALICE="${USERS}/alice.pem"
ADDRESS=$(mxpy data load --key=address-testnet)
PROJECT="../output/adder.wasm"
DEPLOY_TRANSACTION=$(mxpy data load --key=deployTransaction-testnet)
PROXY=https://testnet-api.multiversx.com

deploy() {
    mxpy --verbose contract deploy --project=${PROJECT} --recall-nonce --pem=${ALICE} --gas-limit=50000000 --arguments 0 --send --outfile="deploy-testnet.interaction.json" --proxy=${PROXY} --chain=T || return

    TRANSACTION=$(mxpy data parse --file="deploy-testnet.interaction.json" --expression="data['emittedTransactionHash']")
    ADDRESS=$(mxpy data parse --file="deploy-testnet.interaction.json" --expression="data['contractAddress']")

    mxpy data store --key=address-testnet --value=${ADDRESS}
    mxpy data store --key=deployTransaction-testnet --value=${TRANSACTION}

    echo ""
    echo "Smart contract address: ${ADDRESS}"
}

add() {
    read -p "Enter number: " NUMBER
    mxpy --verbose contract call ${ADDRESS} --recall-nonce --pem=${ALICE} --gas-limit=5000000 --function="add" --arguments ${NUMBER} --send --proxy=${PROXY} --chain=T
}

getSum() {
    mxpy --verbose contract query ${ADDRESS} --function="getSum" --proxy=${PROXY}
}
```

This file helps us to easily make deployment and transactions on the blockchain.

First, let's modify the `ALICE` variable and put our own `pem` file.

After that use `source` command or `.` in bash to interpret the file:
```bash
costin@Byblos:~/mvx/mx-contracts-rs/contracts/adder/interaction$ . testnet.snippets.sh 
```

This will load all the variables and functions in the environment.
Now we can call the `deploy` function:
```bash
costin@Byblos:~/mvx/mx-contracts-rs/contracts/adder/interaction$ deploy 
INFO     cli.contracts: Contract address: erd1qqqqqqqqqqqqqpgq9ss82g55h3k96898kmdtp8am3a4qaefcuvaqutqjjd
INFO     utils: View this contract address in the MultiversX Testnet Explorer: https://testnet-explorer.multiversx.com/accounts/erd1qqqqqqqqqqqqqpgq9ss82g55h3k96898kmdtp8am3a4qaefcuvaqutqjjd
INFO     utils: View this transaction in the MultiversX Testnet Explorer: https://testnet-explorer.multiversx.com/transactions/761d6fb8b42cb0eed546f9c9f55ffbf2457f3fc35854e6030162fb65b6b840df
WARNING  cli.data: Never use this command to store sensitive information! Data is unencrypted.
INFO     cli.data: Data has been stored at key = 'address-testnet', in partition = '*'.
WARNING  cli.data: Never use this command to store sensitive information! Data is unencrypted.
INFO     cli.data: Data has been stored at key = 'deployTransaction-testnet', in partition = '*'.

Smart contract address: erd1qqqqqqqqqqqqqpgq9ss82g55h3k96898kmdtp8am3a4qaefcuvaqutqjjd
```

Now we have performed a deployment of the wasm binary (our adder contract) in the blockchain.

Notice the _MultiversX Testnet Explorer: https://testnet-explorer.multiversx.com/transactions/761d6fb8b42cb0eed546f9c9f55ffbf2457f3fc35854e6030162fb65b6b840df_.
Click on the link to see your transaction. 

Notice the _Smart contract address: erd1qqqqqqqqqqqqqpgq9ss82g55h3k96898kmdtp8am3a4qaefcuvaqutqjjd_.
Go to the [Testnet Explorer](https://testnet-explorer.multiversx.com) and search for your SC address.

Notice the contract deployed on testnet:

![Contract deployed on Testnet](../media/contract.png)

Observe the fields **Owner**, **Deployed** (timestamp).
