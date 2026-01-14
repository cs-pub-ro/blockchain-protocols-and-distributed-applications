# Solana Program Prerequisites

Before developing Solana programs (smart contracts), you need to install and configure the necessary tools. This guide assumes you have already completed the basic environment setup from the [Setup Guide](../../introduction/lab/content/setup/setup.md).

## Required Tools

### 1. Solana CLI

The Solana Command Line Interface (CLI) is essential for interacting with Solana clusters, managing accounts, and deploying programs.

**Installation**:
```bash
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
```

**Verify Installation**:
```bash
solana --version
```

**Configure for Devnet** (recommended for development):
```bash
solana config set --url https://api.devnet.solana.com
```

**Check Configuration**:
```bash
solana config get
```

### 2. Anchor Framework

Anchor is the most popular framework for Solana program development. It provides:
- A convenient way to build Solana programs
- IDL (Interface Definition Language) generation
- Client SDK generation
- Testing utilities
- Built-in security checks

**Installation using AVM (Anchor Version Manager)**:
```bash
cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
avm install latest
avm use latest
```

**Verify Installation**:
```bash
anchor --version
```

**Note**: If you encounter issues, you can also install Anchor directly:
```bash
cargo install --git https://github.com/coral-xyz/anchor anchor-cli --locked --force
```

### 3. Rust Toolchain

Solana programs are written in Rust. Make sure you have Rust installed:

**Check Rust Installation**:
```bash
rustc --version
cargo --version
```

**Install Rust** (if needed):
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**Install Required Rust Targets**:
```bash
rustup target add bpf-unknown-unknown
rustup target add sbf-solana-solana
```

### 4. Solana Test Validator (Optional but Recommended)

For local development, you can run a local Solana cluster:

**Start Local Validator**:
```bash
solana-test-validator
```

**Configure CLI for Local Validator**:
```bash
solana config set --url localhost
```

Keep the validator running in a separate terminal during development.

### 5. Node.js and npm (for Client Development)

For building client applications that interact with Solana programs:

**Install Node.js** (if not already installed):
- Visit [nodejs.org](https://nodejs.org/) or use a package manager

**Verify Installation**:
```bash
node --version
npm --version
```

**Install Solana Web3.js**:
```bash
npm install @solana/web3.js
```

## Development Environment Setup

### Create a Development Keypair

For development, you'll need a keypair to deploy programs:

**Generate a New Keypair**:
```bash
solana-keygen new
```

This will:
- Create a new keypair file (usually `~/.config/solana/id.json`)
- Ask you to set a passphrase (optional for devnet)
- Display your public key

**View Your Public Key**:
```bash
solana address
```

**Get Test SOL on Devnet**:
```bash
solana airdrop 2
```

This requests 2 SOL from the devnet faucet. You can request more if needed (up to the daily limit).

### Verify Your Setup

Run these commands to verify everything is configured correctly:

```bash
# Check Solana CLI
solana --version

# Check Anchor
anchor --version

# Check Rust
rustc --version

# Check your configuration
solana config get

# Check your balance (should show SOL on devnet)
solana balance
```

## IDE Setup (Optional but Recommended)

### VS Code Extensions

For a better development experience, install these VS Code extensions:

1. **Rust Analyzer**: For Rust language support
   - Extension ID: `rust-lang.rust-analyzer`

2. **Anchor**: For Anchor framework support
   - Extension ID: `coral-xyz.anchor`

3. **Solana**: For Solana development tools
   - Extension ID: `solana.solana`

### Installation

Open VS Code and go to Extensions (Ctrl+Shift+X / Cmd+Shift+X), then search for and install the extensions listed above.

## Troubleshooting

### Common Issues

**Issue**: `solana` command not found
- **Solution**: Add Solana to your PATH. Add this to your `~/.bashrc` or `~/.zshrc`:
  ```bash
  export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"
  ```
  Then reload: `source ~/.bashrc` or `source ~/.zshrc`

**Issue**: Anchor installation fails
- **Solution**: Make sure Rust is up to date: `rustup update`
- Try installing without `--locked` flag
- Check that you have the latest version of Cargo

**Issue**: Cannot get airdrop on devnet
- **Solution**: 
  - Wait a few minutes and try again (rate limiting)
  - Use a different RPC endpoint
  - Check devnet status at [status.solana.com](https://status.solana.com)

**Issue**: Test validator won't start
- **Solution**: 
  - Make sure port 8899 is not in use
  - Check that you have sufficient disk space
  - Try resetting: `solana-test-validator --reset`

## Next Steps

Once you have all prerequisites installed:

1. Verify all tools are working
2. Configure for devnet
3. Get test SOL
4. Proceed to [Your First Solana Program](./solana_hello_world.md)

## Additional Resources

- [Solana Documentation](https://docs.solana.com/)
- [Anchor Documentation](https://www.anchor-lang.com/)
- [Solana Cookbook](https://solanacookbook.com/)
- [Solana CLI Reference](https://docs.solana.com/cli)

