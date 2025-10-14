# Transactions

So we have a wallet we know how a transaction looks lets send a transaction to another address in the blockchain.

First we need to know the address where we want to send the coins.

This is the format of a transaction.

```

//transaction struct
type Transaction struct {
 BlockNumber int
 Time        int64
 Hash        string
 Inputs      []TxInput
 Outputs     []TxOutput
}

type TxInput struct {
 Txid      int64
 Value     int64
 Signature string
 PubKey    string
}

type TxOutput struct {
 Txid       int64
 Value      int64
 PubKeyHash string
 Signature  string
}

```

After we send the transaction from our node, the data will be sent across the network to all the nodes, now the nodes will start to validate the transaction(what means to validate the transaction? — first to check that input money are valid by verify the signature of the one that send money to the person that is sending towards you.

If a transaction is invalid the node will not send the transaction further in the network and will send a message to the node that send the invalid transaction that the transaction is invalid.

But if the transaction is valid will be send further to the other nodes and stored in a mempool until…a node will chose to mine a block and to add the transaction into the blockchain.