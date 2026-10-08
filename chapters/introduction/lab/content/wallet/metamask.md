# Installing MetaMask

## Overview

MetaMask is a cryptocurrency wallet and gateway to blockchain applications. This guide will walk you through installing and setting up MetaMask for use in this workshop.

## Why MetaMask?

For our workshop on blockchain protocols and distributed applications, MetaMask provides:

- A secure wallet for storing cryptocurrency and tokens
- Connection to Ethereum, Arbitrum, Base, and other EVM-compatible networks
- Ability to interact with decentralized applications (dApps)
- Signing transactions for smart contract deployment and interaction

## Installation Process

### 1. Browser Extension Installation

MetaMask is available as a browser extension for Chrome, Firefox, Brave, and Edge.

1. Visit the [MetaMask website](https://metamask.io/download/)
2. Click "Install MetaMask for [Your Browser]"
3. You'll be redirected to your browser's extension store
4. Click "Add to [Browser]" or "Install"
5. After installation, the MetaMask fox icon will appear in your browser's toolbar

### 2. Creating a New Wallet

After installing the extension:

1. **Open the Extension**: Click on the MetaMask icon in your browser toolbar.
2. **Begin Setup**: You will be automatically redirected to the "Welcome" page. Select "Create new wallet".
3. **Login**: You can choose to log in using a third-party service (like Apple or Google). If you skip this, you will need to create a password for your wallet.
   - Use a strong, unique password.
   - **This is separate from your recovery phrase.**
4. **Finalize**: Agree to the terms of use.

### 3. Securing Your Recovery Phrase

This is a CRITICAL step:

1. **Generate the Phrase**: MetaMask will generate a unique 12-word Secret Recovery Phrase (often called a "seed phrase").
2. **Revel and Note**: Click "Reveal Secret Recovery Phrase" and write these words down **in order**.
3. **Store offline**: Store this phrase securely offline - preferably in multiple physical locations.
4. **NEVER share this phrase with anyone or store it digitally.**
5. **Verify**: Complete the verification by selecting the words in the correct order

:::info
The specific order of the words in your seed phrase is crucial. If you enter the words in the wrong order, you will not be able to access your funds.
:::

:::warning
If you lose your recovery phrase, you will permanently lose access to your wallet and all funds. There is no "forgot password" option for blockchain wallets.
:::

### 4. Wallet Configuration

Your wallet is now set up! You should see a screen with:

- Your account address (a long string starting with `0x...`)
- An ETH balance (initially 0)
- Options to buy, send, and swap cryptocurrencies

## Resources

- [MetaMask Official Documentation](https://support.metamask.io)
- [Ethereum Developer Documentation](https://ethereum.org/developers)