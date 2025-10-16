# Devnet & Testnet Blockchains

While developing blockchain applications, there is a need for development and testing environments that simulate the mainnet without the risk of using real assets. These environments allow developers to test smart contracts, dApps, and other blockchain applications safely.

## Table of Contents

- [What are Devnets and Testnets?](#what-are-devnets-and-testnets)
- [MultiversX Networks](#multiversx-networks)

---

## What are Devnets and Testnets?

### Devnet (Development Network)
- **Purpose**: Closely mirrors the mainnet for development and testing
- **Stability**: More stable than testnet, resets less frequently
- **Use Case**: Primary development environment for most projects

### Testnet (Testing Network)
- **Purpose**: Used for testing network upgrades and new features
- **Stability**: May reset more frequently, less stable than devnet
- **Use Case**: Testing upcoming releases and network changes

---

## MultiversX Networks

### MultiversX Devnet

**Purpose**: Primary development environment for MultiversX applications

**Features**:
- Closely mirrors mainnet functionality
- Stable environment for development
- Regular resets for clean testing
- Full smart contract support

**Access**:
- **Explorer**: [devnet-explorer.multiversx.com](https://devnet-explorer.multiversx.com/)
- **Wallet**: [devnet-wallet.multiversx.com](https://devnet-wallet.multiversx.com/)
- **Gateway**: `https://devnet-gateway.multiversx.com`
- **Chain ID**: `D`

**Configuration**:
```shell
mxpy config set proxy https://devnet-gateway.multiversx.com
mxpy config set chainID D
```

### MultiversX Testnet

**Purpose**: Testing environment for network upgrades and new features

**Features**:
- More experimental than devnet
- Used for testing protocol upgrades
- May have different features than mainnet
- Less frequent resets than devnet

**Access**:
- **Explorer**: [testnet-explorer.multiversx.com](https://testnet-explorer.multiversx.com/)
- **Wallet**: [testnet-wallet.multiversx.com](https://testnet-wallet.multiversx.com/)
- **Gateway**: `https://testnet-gateway.multiversx.com`
- **Chain ID**: `T`

**Configuration**:
```shell
mxpy config set proxy https://testnet-gateway.multiversx.com
mxpy config set chainID T
```