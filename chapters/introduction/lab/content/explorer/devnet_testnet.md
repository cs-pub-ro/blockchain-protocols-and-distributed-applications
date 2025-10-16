# Devnet & Testnet Blockchains

While developing blockchain applications, there is a need for development and testing environments that simulate the mainnet without the risk of using real assets. These environments allow developers to test smart contracts, dApps, and other blockchain applications safely.

## Table of Contents

- [What are Devnets and Testnets?](#what-are-devnets-and-testnets)
- [MultiversX Networks](#multiversx-networks)
- [Ethereum Networks](#ethereum-networks)
- [Solana Networks](#solana-networks)
- [Network Comparison](#network-comparison)
- [Getting Test Tokens](#getting-test-tokens)
- [Best Practices](#best-practices)

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

---

## Ethereum Networks

### Sepolia Testnet (Recommended)

**Purpose**: Primary testnet for Ethereum application development

**Features**:
- Stable environment for testing
- Proof-of-Stake consensus mechanism
- Similar to mainnet in functionality
- Long-term support

**Access**:
- **Explorer**: [sepolia.etherscan.io](https://sepolia.etherscan.io/)
- **RPC URL**: `https://sepolia.infura.io/v3/YOUR_INFURA_KEY`
- **Chain ID**: `11155111`
- **Currency Symbol**: `ETH`

**Configuration**:
```javascript
// MetaMask or other wallet
Network Name: Sepolia test network
RPC URL: https://sepolia.infura.io/v3/YOUR_INFURA_KEY
Chain ID: 11155111
Currency Symbol: ETH
Block Explorer: https://sepolia.etherscan.io
```

### Holesky Testnet

**Purpose**: Testing validation and staking features

**Features**:
- Focus on validator testing
- Staking protocol testing
- Network upgrade testing
- More experimental than Sepolia

**Access**:
- **Explorer**: [holesky.etherscan.io](https://holesky.etherscan.io/)
- **RPC URL**: `https://holesky.infura.io/v3/YOUR_INFURA_KEY`
- **Chain ID**: `17000`
- **Currency Symbol**: `ETH`

### Goerli Testnet (Deprecated)

**Purpose**: Legacy testnet (being phased out)

**Status**: Deprecated in favor of Sepolia and Holesky
**Note**: Still functional but not recommended for new projects

---

## Solana Networks

### Solana Devnet

**Purpose**: Primary development environment for Solana applications

**Features**:
- Closely mirrors mainnet
- Stable for development
- Regular airdrops available
- Full program support

**Access**:
- **Explorer**: [explorer.solana.com/?cluster=devnet](https://explorer.solana.com/?cluster=devnet)
- **RPC URL**: `https://api.devnet.solana.com`
- **Currency Symbol**: `SOL`

**Configuration**:
```shell
solana config set --url https://api.devnet.solana.com
```

### Solana Testnet

**Purpose**: Testing network upgrades and new features

**Features**:
- Used by Solana core team
- Testing upcoming releases
- Network performance testing
- Validator behavior testing

**Access**:
- **Explorer**: [explorer.solana.com/?cluster=testnet](https://explorer.solana.com/?cluster=testnet)
- **RPC URL**: `https://api.testnet.solana.com`
- **Currency Symbol**: `SOL`

**Configuration**:
```shell
solana config set --url https://api.testnet.solana.com
```

### Solana Localnet

**Purpose**: Local development environment

**Features**:
- Runs on your local machine
- Complete control over the environment
- No network dependencies
- Perfect for unit testing

**Configuration**:
```shell
solana-test-validator
solana config set --url localhost
```

---

## Network Comparison

| Feature | MultiversX Devnet | MultiversX Testnet | Ethereum Sepolia | Ethereum Holesky | Solana Devnet | Solana Testnet |
|---------|-------------------|-------------------|------------------|------------------|---------------|----------------|
| **Purpose** | Development | Testing | Development | Staking Testing | Development | Network Testing |
| **Stability** | High | Medium | High | Medium | High | Medium |
| **Reset Frequency** | Regular | Less Frequent | Rare | Rare | Regular | Less Frequent |
| **Faucet Available** | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| **Explorer** | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| **Wallet Support** | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| **Smart Contracts** | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| **Recommended for** | Most Development | Protocol Testing | Most Development | Validator Testing | Most Development | Core Testing |

---

## Getting Test Tokens

### MultiversX Faucets

**Devnet Faucet**:
1. Visit [devnet-wallet.multiversx.com/faucet](https://devnet-wallet.multiversx.com/faucet)
2. Connect your wallet or create a new one
3. Request testnet eGLD tokens

**CLI Faucet**:
```shell
mxpy wallet faucet --proxy https://devnet-gateway.multiversx.com
```

### Ethereum Faucets

**Sepolia Faucets**:
1. **Alchemy**: [sepoliafaucet.com](https://sepoliafaucet.com/)
2. **QuickNode**: [faucet.quicknode.com/ethereum/sepolia](https://faucet.quicknode.com/ethereum/sepolia)
3. **Chainlink**: [faucets.chain.link/sepolia](https://faucets.chain.link/sepolia)

**Holesky Faucets**:
1. **Holesky Faucet**: [faucet.holesky.ethpandaops.io](https://faucet.holesky.ethpandaops.io/)

### Solana Faucets

**Web Faucet**:
1. Visit [solfaucet.com](https://solfaucet.com/)
2. Enter your wallet address
3. Request test SOL tokens

**CLI Faucet**:
```shell
# For Devnet
solana config set --url https://api.devnet.solana.com
solana airdrop 2 <your-wallet-address>

# For Testnet
solana config set --url https://api.testnet.solana.com
solana airdrop 2 <your-wallet-address>
```

---

## Best Practices

### Development Workflow

1. **Start with Devnet/Testnet**: Always begin development on test networks
2. **Use Local Networks**: For unit testing, use local networks when possible
3. **Test Thoroughly**: Comprehensive testing before mainnet deployment
4. **Monitor Network Status**: Check for network maintenance or issues

### Security Considerations

1. **Separate Wallets**: Use different wallets for testnet and mainnet
2. **Never Mix Networks**: Keep testnet and mainnet environments separate
3. **Verify Networks**: Always double-check which network you're using
4. **Backup Testnet Keys**: Even testnet keys should be backed up securely

### Network Selection Guide

**For Development**:
- **MultiversX**: Use Devnet
- **Ethereum**: Use Sepolia
- **Solana**: Use Devnet

**For Testing**:
- **MultiversX**: Use Testnet for protocol testing
- **Ethereum**: Use Holesky for staking tests
- **Solana**: Use Testnet for network testing

---

## Inspect the Networks

Explore these testnet explorers to see the differences:

**MultiversX**:
- [MultiversX Devnet Explorer](https://devnet-explorer.multiversx.com/)
- [MultiversX Testnet Explorer](https://testnet-explorer.multiversx.com/)

**Ethereum**:
- [Sepolia Ethereum Testnet Explorer](https://sepolia.etherscan.io/)
- [Holesky Ethereum Testnet Explorer](https://holesky.etherscan.io/)

**Solana**:
- [Solana Devnet Explorer](https://explorer.solana.com/?cluster=devnet)
- [Solana Testnet Explorer](https://explorer.solana.com/?cluster=testnet)

### What to Observe

When inspecting these networks, notice:
- **Transaction volumes**: Testnets typically have lower activity
- **Token values**: All tokens are "fake" and have no real value
- **Network features**: Some features may differ from mainnet
- **Block times**: May be different from mainnet
- **Gas fees**: Usually much lower or free on testnets

---

**⚠️ IMPORTANT NOTE**

All tokens on devnets and testnets are "fake" tokens. They are only used to mock the real tokens on the mainnet and have no real value. Never send real cryptocurrency to testnet addresses, and never use testnet private keys for mainnet wallets.