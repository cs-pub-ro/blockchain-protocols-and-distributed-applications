# Create Wallet from command line

In this section you will learn how to create a wallet from the command line.
We will use `mxpy` tools installed in the previous section.

You can create a new wallet by using the `mxpy wallet new` command:

```bash
$ mxpy wallet new --help
usage: mxpy wallet new [-h] ...

Create a new wallet and print its mnemonic; optionally save as password-protected JSON (recommended) or PEM (not recommended)

options:
  -h, --help                                      show this help message and exit
  --format {raw-mnemonic,keystore-mnemonic,keystore-secret-key,pem}
                                                  the format of the generated wallet file (default: None)
  --outfile OUTFILE                               the output path and base file name for the generated wallet files
                                                  (default: None)
  --address-hrp ADDRESS_HRP                       the human-readable part of the address, when format is keystore-
                                                  secret-key or pem (default: erd)
```

## Create a wallet with pem format

**Notice**: Pem format is recommended **only** for development! The advantage is that you don't require any other input (password for example).
This is why it's also really dangerous to use pem format for personal wallets.

```bash
$ mxpy wallet new --format pem --outfile private_key.pem
Mnemonic: glide place surround pupil gold razor member shove detail love tray prefer fire marriage undo capital play runway injury emotion attitude spike goddess lion
INFO     cli.wallet: Wallet (pem) saved: /home/costin/mvx/mx-contracts-rs/contracts/adder/interaction/private_key.pem      
```

In blockchain, Mnemonics refers to a 12, 18 or 24-word phrase created for new cryptocurency wallet holders. Mnemonics are used to make cryptocurrency wallets more secure and easy to store.

Mnemonics aim to ensure that the cryptocurrency wallet can be accessed when the cryptocurrency wallet password is lost or forgotten. Given the decentralized and secure nature of cryptocurrency wallets, it is difficult to access the details of a cryptocurrency wallet. The details of decentralized and secure cryptocurrency wallets can be accessed through mnemonics. If the mnemonic is lost, access to the cryptocurrency wallet is completely lost.

**Notice**: In order to use this addres to deploy contracts or make transactions you must go to the [tesnet wallet](https://testnet-wallet.multiversx.com/unlock) or [devnet wallet](https://devnet-wallet.multiversx.com/unlock) and use the `Faucet` option to request xEGLD. The tokens are required to pay the fees.

**Notice**: Same private key can be used for devnet, testnet or mainnet, but the storage is completly different. For example if you have 1 EGLD on Testnet, you don't necessarly have 1 EGLD on Devnet.
