# Solana dApp Frontend Integration

## Introduction
A pristine, highly optimized Rust smart contract is useless if users cannot interact with it easily. This lab covers how to connect a modern React/Next.js frontend to your Anchor programs on Solana.

We will use two crucial pieces of infrastructure:
1. **Solana Wallet Adapter:** To securely connect user wallets like Phantom and Solflare.
2. **Anchor TypeScript Client (`@coral-xyz/anchor`):** To generate strongly typed RPC calls to our smart contract.

## 1. Setting up the Wallet Adapter
Interacting with the blockchain requires an RPC connection. The Wallet Adapter provides this via context providers.

```bash
# In your Next.js project directory
npm install @solana/web3.js @solana/wallet-adapter-react \
@solana/wallet-adapter-react-ui @solana/wallet-adapter-wallets
```

Wrap your root application (`_app.tsx` or `layout.tsx`) in the necessary providers:

```tsx
import { ConnectionProvider, WalletProvider } from '@solana/wallet-adapter-react';
import { WalletModalProvider } from '@solana/wallet-adapter-react-ui';
import { clusterApiUrl } from '@solana/web3.js';
import '@solana/wallet-adapter-react-ui/styles.css';

export default function App({ Component, pageProps }) {
    // Configure to devnet
    const endpoint = clusterApiUrl('devnet');
    
    // Wallets that your dApp will support natively
    const wallets = [];

    return (
        <ConnectionProvider endpoint={endpoint}>
            <WalletProvider wallets={wallets} autoConnect>
                <WalletModalProvider>
                    <Component {...pageProps} />
                </WalletModalProvider>
            </WalletProvider>
        </ConnectionProvider>
    );
}
```

## 2. Connect Button
Users need a way to trigger the wallet connection prompt. The `@solana/wallet-adapter-react-ui` library gives us a drop-in component:

```tsx
import { WalletMultiButton } from '@solana/wallet-adapter-react-ui';

export const Header = () => (
    <header>
        <h1>My Solana dApp</h1>
        <WalletMultiButton /> {/* This renders "Select Wallet" -> "Connect" */}
    </header>
);
```

## 3. Using Anchor to Call the Contract
When you build an Anchor program (`anchor build`), it generates an IDL (Interface Description Language) JSON file in `target/idl/`. This file describes all instructions and accounts your program has.

You must copy this IDL JSON and the generated TypeScript types into your frontend repository.

```tsx
import { useConnection, useWallet } from '@solana/wallet-adapter-react';
import { Program, AnchorProvider, web3 } from '@coral-xyz/anchor';
import idl from '../idl/my_program.json'; // The imported IDL

export const InteractComponent = () => {
    const { connection } = useConnection();
    const wallet = useWallet();

    const callContract = async () => {
        if (!wallet.publicKey) return;

        // 1. Setup the Anchor Provider
        const provider = new AnchorProvider(
            connection, 
            wallet as any, 
            { commitment: "confirmed" }
        );

        // 2. Initialize the Program instance
        const program = new Program(idl as any, idl.metadata.address, provider);

        // 3. Send the RPC call
        try {
            const txHash = await program.methods
                .myInstructionName("Argument 1")
                .accounts({
                    user: wallet.publicKey,
                    systemProgram: web3.SystemProgram.programId,
                    // Note: If you have a PDA, derive it here using PublicKey.findProgramAddressSync
                })
                .rpc();
                
            console.log("Transaction details:", txHash);
        } catch (err) {
            console.error(err);
        }
    }

    return <button onClick={callContract}>Send Transaction</button>
}
```

## 4. Assignment
Build a simple frontend for the **To-Do List** Anchor program you created in the advanced smart contracts lab.
1. Implement the Wallet Adapter.
2. Render a button to `create_task`.
3. Fetch the created PDAs mapping to the user's pubkey using `program.account.todoTask.all([{ memcmp: { offset: 8, bytes: wallet.publicKey.toBase58() } }])` to filter by the user's tasks.
4. Render the list of tasks.
