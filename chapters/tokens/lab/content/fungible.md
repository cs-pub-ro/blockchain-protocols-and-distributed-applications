# Fungible Tokens

## Intro - Fungible vs Non-Fungible Tokens

* Fungibility refers to an asset's ability to be exchanged for something else of equal value;
* Some examples of fungible assets include currencies, commodities, and precious stones;
* Non-fungible assets are unique, requiring much more complex valuation before a sale and include things like real estate, art, and sports cards.

Throughout the lab, you will need to convert values from decimals to hex to ascii, etc. Use [this](https://utils.multiversx.com/converters) tool to make your job easier.

### Practice

Think about the following example if fungible or non-fungible:

* Diamond rock;
* Penthouse apartment in Manhattan;
* Mona Lisa painting;
* LeBron James sport card;
* LeBron James shirt he played in the 2020 NBA Finals;
* A character in World of Warcraft.

## Issuance of fungible ESDT tokens

ESDT tokens are issued via a request to the Metachain, which is a transaction submitted by the Account which will manage the tokens. When issuing a token, one must provide a token name, a ticker, the initial supply, the number of decimals for display purpose and optionally additional properties. The **issuance cost is set to 0.05 EGLD**.

The receiver address **erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u** is a built-in system smart contract (not a VM-executable contract), which only handles token issuance and other token management operations, and does not handle any transfers. The contract will add a random string to the ticker thus creating the token identifier. The random string starts with “-” and has 6 more random characters (3 bytes - 6 characters hex encoded). For example, a token identifier could look like **ALC-6258d2**.

Read more about issuance of fungible ESDT tokens [here](https://docs.multiversx.com/tokens/esdt-tokens#issuance-of-fungible-esdt-tokens).

### Issuance example:
```
IssuanceTransaction {
    Sender: <account address of the token manager>
    Receiver: erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u
    Value: 50000000000000000 # (0.05 EGLD)
    GasLimit: 60000000
    Data: "issue" +
          "@" + <token name in hexadecimal encoding> +
          "@" + <token ticker in hexadecimal encoding> +
          "@" + <initial supply in hexadecimal encoding> +
          "@" + <number of decimals in hexadecimal encoding>
}
```

### Practice

* Issue a token of your own. Choose whatever name and ticker you want. Check the existance of your token via [Testnet Explorer](https://testnet-explorer.multiversx.com/).


## Mint fungible ESDT tokens

---
**NOTE**

Actions **issue** and **mint** are distinct! First, one must **issue** the token to exist on the blockchain. Second, the owner/issuer of the token must **mint** tokens.

---

The transaction to mint tokens is:
```
LocalMintTransaction {
    Sender: <address with ESDTRoleLocalMint role>
    Receiver: <same as sender>
    Value: 0
    GasLimit: 300000
    Data: "ESDTLocalMint" +
          "@" + <token identifier in hexadecimal encoding> +
          "@" + <supply to mint in hexadecimal encoding>
}
```

Note that the sender and the receiver are the same address, which is the creator of the token.
After this transaction, he will receive in his wallet the minted tokens.

### Practice

* Use the token created at the previous section to mint some tokens. Verifiy on the [Testnet Explorer](https://testnet-explorer.multiversx.com/) and in your wallet.

## Setting ESDT Roles

Notice the **Properties** section in a Token page.
![UTK tokens](../media/utk.png)

There are several roles assigned to a token. 
To change those roles we need to make a blockchain transaction:
```
RolesAssigningTransaction {
    Sender: <address of the ESDT manager>
    Receiver: erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u
    Value: 0
    GasLimit: 60000000
    Data: "setSpecialRole" +
          "@" + <token identifier in hexadecimal encoding> +
          "@" + <address to assign the role(s) in a hexadecimal encoding> +
          "@" + <role in hexadecimal encoding> +
          "@" + <role in hexadecimal encoding> +
          ...
}
```

### Practice

* Observe the roles of your newly created token.
* Add **ESDTRoleLocalMint** for your address.


You can read more about roles in the [documentation](https://docs.multiversx.com/tokens/esdt-tokens/#setting-and-unsetting-special-roles).
