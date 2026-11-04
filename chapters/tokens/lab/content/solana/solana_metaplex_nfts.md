# Non-Fungible Tokens (NFTs) with Metaplex

## Introduction
When you created your SPL fungible token in the previous lab, you might have noticed it showed up as "Unknown Token" with no image or name in your wallet.

The `TokenkegQ...` SPL Token Program knows about logic (balances, supply, transfers), but it knows absolutely nothing about names, symbols, or images.

To solve this, the **Metaplex Protocol** introduced the **Token Metadata Program**. 

## 1. The Metadata Account
To add a name and image to your token, you must create a separate **Metadata Account** that attaches to your Mint Account.

The Metaplex Token Metadata Program creates a PDA for this Metadata Account using the seeds:
`[b"metadata", metaplex_program_id, mint_address]`

### What goes in the Metadata?
1. **Name:** e.g., "Mad Lads"
2. **Symbol:** e.g., "MAD"
3. **URI:** A link (often Arweave, IPFS, or shadow drive) to a JSON file containing the actual image link and traits.
4. **Seller Fee Basis Points:** Royalties the creator earns on secondary sales (e.g., 500 = 5%).

## 2. Setting up Umi & Metaplex
Metaplex has migrated away from standard `@solana/web3.js` toward their own framework called **Umi** (`@metaplex-foundation/umi`).

Let's initialize a basic Node.js script to mint an NFT.

```javascript
import { createUmi } from '@metaplex-foundation/umi-bundle-defaults';
import { createNft, mplTokenMetadata } from '@metaplex-foundation/mpl-token-metadata';
import { generateSigner, keypairIdentity, percentAmount } from '@metaplex-foundation/umi';
import bs58 from 'bs58';

// 1. Connect to Devnet
const umi = createUmi('https://api.devnet.solana.com');

// 2. Load your wallet from your private key
// WARNING: Never expose your private key in production!
const secretKey = bs58.decode('YOUR_PRIVATE_KEY_HERE');
const keypair = umi.eddsa.createKeypairFromSecretKey(secretKey);
umi.use(keypairIdentity(keypair));

// 3. Register the Token Metadata Program
umi.use(mplTokenMetadata());
```

## 3. Minting the NFT
An NFT is simply an SPL Token with:
1. `decimals = 0` (You can't sell 0.5 of an NFT)
2. `supply = 1`
3. A Metadata account attached via Metaplex.

The Umi library handles all of this in one simple function: `createNft`.

```javascript
// 4. Generate a random Mint Address
const mint = generateSigner(umi);

// 5. Send the transaction
async function mintMyNFT() {
    const tx = await createNft(umi, {
        mint,
        name: "My First Student NFT",
        symbol: "UPBNFT",
        uri: "https://arweave.net/1234abcd5678efgh", // Link to off-chain JSON
        sellerFeeBasisPoints: percentAmount(5), // 5% royalties
    }).sendAndConfirm(umi);

    console.log("NFT Mint Address:", mint.publicKey);
}

mintMyNFT();
```

## 4. Off-Chain JSON Metadata Formats
Your `uri` points to a JSON file that conforms to the Metaplex standard. This is where you specify the actual image URL and attributes.

```json
{
  "name": "My First Student NFT",
  "symbol": "UPBNFT",
  "description": "Minted during the BPDA Course.",
  "image": "https://arweave.net/image_hash.png",
  "attributes": [
    { "trait_type": "Course", "value": "BPDA" },
    { "trait_type": "Grade", "value": "A+" }
  ]
}
```

## 5. Assignment
1. Write a script that mints an NFT using Umi on Devnet.
2. The NFT should have an off-chain JSON URI that points to an image hosted on GitHub Pages or a free IPFS gateway.
3. Verify your minted NFT on the Solana Explorer (it will show the image and name if structured correctly).
