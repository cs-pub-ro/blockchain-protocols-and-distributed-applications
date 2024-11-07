# Prerequisites

All of these prerequisites were done at the first lab. You can skip this section if you have already done this.

## Install mxpy - blockchain interaction

We use `mxpy` to interact with the blockchain.

To install `mxpy` run:
```bash
pipx install multiversx-sdk-cli --force
```

To check the successful installation:
```bash
$ mxpy --version
MultiversX Python CLI (mxpy) 9.8.1
```

If you encounter any errors, follow the guide [here](https://docs.multiversx.com/sdk-and-tools/sdk-py/installing-mxpy/).

## Install sc-meta - contract interaction

We use `sc-meta` to compile the contracts and to upgrade the dependencies.

To install `sc-meta`, simply call:
```bash
$ cargo install multiversx-sc-meta
```

To check for successful installation:
```bash
$ sc-meta --version
multiversx-sc-meta 0.54.0
```

If you encounter any errors, follow the guide [here](https://docs.multiversx.com/developers/meta/sc-meta#introduction).

## Contracts examples

[Here](https://github.com/multiversx/mx-contracts-rs) is a list of Smart Contract examples. We will use part of them to understand smart contracts on MultiversX.