# Testnet Wallets and Faucets

This document provides comprehensive information about testnet wallets and faucets for MultiversX, Ethereum, and Solana. These testnet environments allow you to develop and test your applications without spending real cryptocurrency.

## Table of Contents

- [MultiversX Testnet](#multiversx-testnet)
- [Ethereum Testnet](#ethereum-testnet)
- [Solana Testnet](#solana-testnet)

---

## MultiversX Testnet

### Testnet Networks

MultiversX provides several testnet environments for development:

- **Devnet**: For development and testing (resets frequently)
- **Testnet**: For more stable testing environment

### Getting Testnet EGLD

**Method 1 - MultiversX Faucet:**
1. Visit the [MultiversX Devnet Faucet](https://devnet-wallet.multiversx.com/faucet)
2. Connect your wallet or create a new one
3. Request testnet EGLD tokens

**Method 2 - mxpy CLI:**
```shell
mxpy wallet faucet --proxy https://devnet-gateway.multiversx.com
```

### Testnet Wallets

**MultiversX Web Wallet:**
- **Devnet**: [devnet-wallet.multiversx.com](https://devnet-wallet.multiversx.com/)
- **Testnet**: [testnet-wallet.multiversx.com](https://testnet-wallet.multiversx.com/)

### Testnet Explorer

- **Devnet Explorer**: [devnet-explorer.multiversx.com](https://devnet-explorer.multiversx.com/)
- **Testnet Explorer**: [testnet-explorer.multiversx.com](https://testnet-explorer.multiversx.com/)

---

## Ethereum Testnet

### Testnet Networks

Ethereum provides several testnet environments:

- **Sepolia**: Current recommended testnet (replaces Goerli)
- **Holesky**: Newer testnet for staking testing
- **Goerli**: Deprecated but still functional

### Getting Testnet ETH

**Method 1 - Sepolia Faucets:**
1. **Alchemy Faucet**: [sepoliafaucet.com](https://sepoliafaucet.com/)
2. **QuickNode Faucet**: [faucet.quicknode.com/ethereum/sepolia](https://faucet.quicknode.com/ethereum/sepolia)
3. **Chainlink Faucet**: [faucets.chain.link/sepolia](https://faucets.chain.link/sepolia)

**Method 2 - Holesky Faucets:**
1. **Holesky Faucet**: [faucet.holesky.ethpandaops.io](https://faucet.holesky.ethpandaops.io/)

### Testnet Wallets

**MetaMask Configuration:**
1. Open MetaMask
2. Click on the network dropdown
3. Select "Add Network" or "Add Testnet"
4. Add Sepolia network:
   - Network Name: Sepolia test network
   - RPC URL: https://sepolia.infura.io/v3/YOUR_INFURA_KEY
   - Chain ID: 11155111
   - Currency Symbol: ETH
   - Block Explorer: https://sepolia.etherscan.io

**Other Wallets:**
- **WalletConnect**: Supports testnet networks
- **Coinbase Wallet**: Built-in testnet support
- **Trust Wallet**: Multi-chain support including testnets

### Configuring Hardhat for Testnet

```javascript
// hardhat.config.js
module.exports = {
  networks: {
    sepolia: {
      url: "https://sepolia.infura.io/v3/YOUR_INFURA_KEY",
      accounts: [process.env.PRIVATE_KEY],
    },
    holesky: {
      url: "https://holesky.infura.io/v3/YOUR_INFURA_KEY",
      accounts: [process.env.PRIVATE_KEY],
    }
  }
};
```

### Testnet Explorers

- **Sepolia**: [sepolia.etherscan.io](https://sepolia.etherscan.io/)
- **Holesky**: [holesky.etherscan.io](https://holesky.etherscan.io/)

---

## Solana Testnet

### Testnet Networks

Solana provides two main testnet environments:

- **Devnet**: For development and testing (resets frequently)
- **Testnet**: For more stable testing environment

### Getting Testnet SOL

**Method 1 - Solana Faucet:**
1. Visit [solfaucet.com](https://solfaucet.com/)
2. Enter your wallet address
3. Request test SOL tokens

**Method 2 - Solana CLI Faucet:**
```shell
# Configure for testnet
solana config set --url https://api.testnet.solana.com

# Request airdrop
solana airdrop 2 <your-wallet-address>
```

**Method 3 - Devnet Faucet:**
```shell
# Configure for devnet
solana config set --url https://api.devnet.solana.com

# Request airdrop
solana airdrop 2 <your-wallet-address>
```

### Testnet Wallets

**Phantom Wallet:**
1. Open Phantom wallet
2. Click on the network name at the top
3. Select "Testnet" or "Devnet" from the dropdown

**Solflare Wallet:**
1. Open Solflare wallet
2. Click on the network dropdown
3. Select your desired testnet (Testnet or Devnet)

**Other Wallets:**
- **Sollet**: Web-based wallet with testnet support
- **Slope**: Mobile wallet with testnet support

### Configuring Solana CLI for Testnet

```shell
# For Testnet
solana config set --url https://api.testnet.solana.com

# For Devnet
solana config set --url https://api.devnet.solana.com

# Check current configuration
solana config get
```

### Testnet Explorers

- **Testnet Explorer**: [explorer.solana.com/?cluster=testnet](https://explorer.solana.com/?cluster=testnet)
- **Devnet Explorer**: [explorer.solana.com/?cluster=devnet](https://explorer.solana.com/?cluster=devnet)

---

## General Tips

### Security Best Practices

1. **Never use mainnet private keys** on testnet environments
2. **Create separate wallets** for testing purposes
3. **Keep testnet and mainnet environments** completely separate
4. **Don't share testnet private keys** even though they have no real value

### Development Workflow

1. **Start with testnet** for all development and testing
2. **Use local networks** when possible (like Hardhat localhost, Solana test validator)
3. **Test thoroughly** on testnet before considering mainnet deployment
4. **Keep track of testnet addresses** and transactions for debugging

### Common Issues

- **Faucet rate limits**: Most faucets have daily limits
- **Network congestion**: Testnets can be slower than mainnet
- **Frequent resets**: Some testnets reset data periodically
- **Different gas costs**: Testnet gas costs may differ from mainnet

**Note**: All testnet tokens have no real value and are used solely for development and testing purposes. Never send real cryptocurrency to testnet addresses.
