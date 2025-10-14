# Token Standards

A crypto token is a representation of an asset or interest that has been tokenized on an existing cryptocurrency's blockchain. Crypto tokens and cryptocurrencies share many similarities, but cryptocurrencies are intended to be used as a medium of exchange, a means of payment, and a measure and store of value.

Crypto tokens are often used to raise funds for projects and are usually created, distributed, sold, and circulated through an initial coin offering (ICO) process, which involves a crowdfunding round.

In this section we will discuss 2 different approaches of token implementation:
* Ethereum Standard: ERC - Ethereum Request for Comments
* MultiversX: ESDT - eStandard Digital Token

## ERC - Ethereum Request for Comments

ERC (Ethereum Request for Comments) standards provide a set of guidelines and specifications for creating and implementing smart contracts, tokens, and other blockchain-based applications on the Ethereum network.
ERCs are a set of technical standards that are used to create and manage tokens on the Ethereum blockchain.
ERC standards define a set of rules and protocols that tokens must follow in order to be compatible with the Ethereum network. These rules cover a variety of aspects, such as token transfer functions, token ownership, and smart contract security. By following these standards, developers can ensure that their tokens are interoperable with other tokens and can be easily exchanged on the Ethereum network.

There are currently several ERC standards, including ERC-20, ERC-721, ERC-777, and ERC-1155, each of which has a specific set of features and functions. **ERC-20 is the most widely used token standard** and is used to create fungible tokens, while **ERC-721 is used to create non-fungible tokens (NFTs)**. Other ERC standards, such as ERC-777 and ERC-1155, provide additional features and functions for creating more complex tokens.

You can read more about [ERC-20](https://ethereum.org/en/developers/docs/standards/tokens/erc-20/) and [ERC-721](https://ethereum.org/en/developers/docs/standards/tokens/erc-721/).

[Here](https://etherscan.io/tokens) are the top Ethereum tokens.

## ESDT - eStandard Digital Token

The MultiversX network natively supports the issuance of custom tokens, without the need for contracts such as ERC20, but addressing the same use-cases. And due to the native in-protocol support, transactions with custom tokens do not require the VM at all.

Technically, the balances of ESDT tokens held by an Account are stored directly under the data trie of that Account. It also implies that an Account can hold balances of any number of custom tokens, in addition to the native EGLD balance. The protocol guarantees that no Account can modify the storage of ESDT tokens, neither its own nor of other Accounts.

ESDT tokens can be issued, owned and held by any Account on the MultiversX network, which means that both users and smart contracts have the same functionality available to them.

## Diferences between ERC and ESDT

The first difference we observe is that **ERC standards are smart contracts** deployed on blockchain while **ESDTs are tokens** assigned to users.
In ERC user balances are stored in the Smart Contract while in ESDT balances are stored in users' wallets.

[Here](https://explorer.multiversx.com/tokens) are the top MultiversX tokens.