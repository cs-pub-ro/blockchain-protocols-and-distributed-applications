# First Blockchain transaction

Let's perform our first ever blockchain transaction.
We will do this on MultiversX blockchain.

First, we need a wallet. Every access, read, write, execute, etc to the blockchain needs to be sign by you with a private key (a wallet).

To create a wallet simply run:
```shell
$ mxpy wallet new --format pem --outfile new_wallet.pem
```

Let's interpret the output:
```shell
$ mxpy wallet new --format pem --outfile new_wallet.pem
Mnemonic: bid blind field captain bar produce brush salute luggage double hole wonder meadow glass destroy giraffe auction square crush catalog knee lizard century nasty
Wallet address: erd1pfhel08mq6ucua005qgyvwq0el78ap3ytpcugy35yvqfdeq7afqsydkj3d
```

First, there is a [mnemonic](https://en.wikipedia.org/wiki/Mnemonic): 24 random words.
These words are used to create your private key (wallet) at any time, so you must store them carefully.
For now, this is only a test wallet, so just store them in a text file on your computer.

Second, we have a wallet address. This is our blockchain address.

Third, we have a new file `new_wallet.pem` that contains our private key (wallet):
```shell
$ cat new_wallet.pem 
-----BEGIN PRIVATE KEY for erd1pfhel08mq6ucua005qgyvwq0el78ap3ytpcugy35yvqfdeq7afqsydkj3d-----
NjM1N2UxOGQxYjBjMDk5ZjY1MzM2OWUxZGFiZGM3Mzg1Yjc5ZmY0ZWNlZTBiNWY5
NWFmNjE1MzZjZDMwODNhOTBhNmY5ZmJjZmIwNmI5OGU3NWVmYTAxMDQ2MzgwZmNm
ZmM3ZTg2MjQ1ODcxYzQxMjM0MjMwMDk2ZTQxZWVhNDE=
-----END PRIVATE KEY for erd1pfhel08mq6ucua005qgyvwq0el78ap3ytpcugy35yvqfdeq7afqsydkj3d-----
```