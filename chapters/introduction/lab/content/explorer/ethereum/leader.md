# Blockchain Leader

We will continue with the same [block](https://etherscan.io/block/24892306) from the previous section (Block Height: 24892306).

![Leader](../../../media/eth_proposal.png)

We can observe that the **Leader** (also called the **proposer**) has address `0x8ae3fb1cacd9a3c44ff7789617f6ae1b700823610c0459f5e3dede3a3110133cbcf5bc613abbae951fb7c005e7e7cc27`.

To find who the leader is, look for the *Proposed On* section, then, on the redirected page, look for the *Proposer Index*.

The leader, also known as the block producer or validator, is responsible for performing several critical tasks within a consensus round:

1. **Block Proposal:** The leader is tasked with proposing a new block containing a set of transactions. This block proposal is the first step in the process of reaching consensus. The leader selects transactions to include in the proposed block and creates a cryptographic hash of the block's contents.

2. **Signature and Verification:** After proposing a block, the leader signs it with their private key to prove its authenticity. Other network participants can verify the leader's signature using their public key. This verification ensures that the proposed block is indeed from the authorized leader.

3. **Broadcasting the Proposal:** Once the block is signed, the leader broadcasts it to the network. Other nodes in the network receive and validate the proposed block to ensure its compliance with the consensus rules.

4. **Agreement and Confirmation:** If the proposed block meets the consensus criteria, it is accepted by the network. The leader's proposed block becomes part of the blockchain, and consensus is achieved for that round.

Let's see some details about this proposer:

![Leader](../../../media/eth_leader.png)

We can see the public key, and the total income.

## Practice

- Open 3 different blocks in 3 different tabs. Observe that every time there is another leader.
Why do you think the system is designed this way?
