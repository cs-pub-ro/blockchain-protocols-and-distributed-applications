# Setting up your environment

In this section we will install all the requirements needed for the practical sessions.
This includes the following:
- rustup - The Rust toolchain installer
- MetaMask - Ethereum wallet
- mxpy - tool for interacting with the blockchain
- sc-meta - universal smart contract management tool


## Rustup - The Rust toolchain installer

Rust is installed and managed by the `rustup` tool.
If you've installed `rustup` in the past, you can update your installation by running `rustup update`.

For more informantion, please check [rustup documentation](https://rust-lang.github.io/rustup/).

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```



### Configuring the PATH environment variable

In the Rust development environment, all tools are installed to the `~/.cargo/bin` directory, and this is where you will find the Rust toolchain, including `rustc`, `cargo`, and `rustup`.

During installation `rustup` will attempt to configure the `PATH`. Because of differences between platforms, command shells, and bugs in `rustup`, the modifications to `PATH` may not take effect until the console is restarted, or the user is logged out, or it may not succeed at all.

Please make sure you have included  `~/.cargo/bin` directory in your `PATH` environment variable.

Please verify that `rustup` was succesffully install with `rustup --version`.

### Uninstall Rust

If at any point you would like to uninstall Rust, you can run `rustup self uninstall`.

## MultiversX prerequisites

### mxpy

Mxpy is a tool for interaction with the blockchain:
```shell
----------------------
COMMAND GROUPS summary
----------------------
contract                       Build, deploy, upgrade and interact with Smart Contracts
tx                             Create and broadcast Transactions
validator                      Stake, UnStake, UnBond, Unjail and other actions useful for Validators
account                        Get Account data (nonce, balance) from the Network
ledger                         Get Ledger App addresses and version
wallet                         Create wallet, derive secret key from mnemonic, bech32 address helpers etc.
deps                           Manage dependencies or multiversx-sdk modules
config                         Configure multiversx-sdk (default values etc.)
localnet                       Set up, start and control localnets
data                           Data manipulation omnitool
staking-provider               Staking provider omnitool
dns                            Operations related to the Domain Name Service
```

Before installing `mxpy`, please make sure you have a working `Python 3` environment. You'll need `Python 3.8` or later on Linux or MacOS.

The recommended way to install `mxpy` is by using `pipx`.
If you'd like to use `mxpy` on Windows, we recommend installing it within the Windows Subsystem for Linux (WSL).

In order to install `mxpy` using `pipx`, run the following command:
```shell
pipx install multiversx-sdk-cli --force
```

To check that mxpy installed successfully you can run the following command:
```shell
mxpy --version
```

### wasm32-unknown-unknown

This is WebAssembly target which uses 32-bit memories. It is used to compile MultiversX smart contracts to WebAssembly.

To add it, please use:
```shell
rustup target add wasm32-unknown-unknown
```

### sc-meta

This tool is used to compile smart contracts.

To install it, run:
```shell
cargo install multiversx-sc-meta --locked
```

To verify that it's correctly installed, run:
```shell
sc-meta --version
```


## Ethereum prerequisites

### MetaMask

MetaMask is a web browser extension and mobile app that allows you to manage your Ethereum private keys. 
By doing so, it serves as a wallet for Ether and other tokens, and allows you to interact with decentralized applications, or *dapps*.

Please visit [this](https://support.metamask.io/getting-started/getting-started-with-metamask/#how-to-install-metamask) to install MetaMask as a browser extension (Safari is not supported) or as a mobile app.


### Hardhat

Hardhat is a development environment for Ethereum software. It consists of different components for editing, compiling, debugging and deploying your smart contracts and dApps, all of which work together to create a complete development environment.

To install Hardhat, use:
```shell
npm install --save-dev hardhat
```

To verify that hardhat is install, run:
```shell
$ npx hardhat
888    888                      888 888               888
888    888                      888 888               888
888    888                      888 888               888
8888888888  8888b.  888d888 .d88888 88888b.   8888b.  888888
888    888     "88b 888P"  d88" 888 888 "88b     "88b 888
888    888 .d888888 888    888  888 888  888 .d888888 888
888    888 888  888 888    Y88b 888 888  888 888  888 Y88b.
888    888 "Y888888 888     "Y88888 888  888 "Y888888  "Y888

👷 Welcome to Hardhat v2.22.12 👷‍

? What do you want to do? … 
  Create a JavaScript project
  Create a TypeScript project
  Create a TypeScript project (with Viem)
  Create an empty hardhat.config.js
❯ Quit
```

For the ones that prefer, there is a [Hardhat for Visual Studio Code](https://hardhat.org/hardhat-vscode/docs/overview).