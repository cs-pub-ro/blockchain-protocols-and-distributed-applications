# Resources and Useful Links

## List of Resources:

- [GitHub Repository](https://github.com/CostinCarabas/blockchain-protocols-and-distributed-applications)
- [Moodle Class](https://curs.upb.ro/2024/course/view.php?id=1947) (used for homework submissions, quizzes, announcements, etc.)
- [Planning](https://docs.google.com/spreadsheets/d/e/2PACX-1vSbaEavaQE8WgMM5QKNRFrrdETS6lYL6APvnAcAvIwYaMlpiL7hBJZqWW0q7wgWpu9LbCa2RUlH1qrB/pubhtml?gid=0&single=true)

## Books and interesting reading materials

- [Foundations of Distributed Consensus and Blockchains](http://elaineshi.com/docs/blockchain-book.pdf) - must read book for blockchain protocol enthusiasts;
- [Implications of Open Monetary and Information Networks](https://www.lynalden.com/open-networks/) - read to form an intuition about the need for blockchain and decentralized solutions;
- [Bitcoin Whitepaper](https://bitcoin.org/bitcoin.pdf);
- [Ethereum Whitepaper](https://ethereum.org/en/whitepaper/);
- [MultiversX Whitepaper](https://files.multiversx.com/multiversx-whitepaper.pdf);
- [State of Crypto](docs/state-of-crypto.pdf)
- [Step by step guide to becoming a blockchain developer in 2024](https://roadmap.sh/blockchain);
- [Secure Development Workflow for Smart Contracts](https://github.com/crytic/building-secure-contracts/blob/master/development-guidelines/workflow.md#secure-development-workflow);
- [DeFi Handbook — Introduction to Decentralized Finance](docs/defi.pdf)
- [Cryptozombies learning game](https://cryptozombies.io/en/multiversx)
- [The Rust Programming Language](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Improving the Rust Book (really interactive)](https://rust-book.cs.brown.edu)
- [Solidity docs](https://docs.soliditylang.org/en/v0.8.28/)

## Virtual Machine

You can use any Linux environment (native install, `WSL`, virtual machine, docker environment, etc.) for the OS class.
We provide Linux virtual machines with all the setup ready.

### VirtualBox / VMware

You can download the Linux virtual machine from [this link](https://repository.grid.pub.ro/cs/so/linux-2024-2025/so-vm-gui-2024.ova).

You can import the `.ova` file in [VirtualBox](https://www.virtualbox.org/) or [VMware](https://www.vmware.com/).
Follow the instructions on the official websites for installation.

### UTM (macOS >= 11)

If you are using an `M1` Apple system, you will not be able to run the virtual machine using VirtualBox or VMware.
You will need to use [`UTM`](https://mac.getutm.app/), along with a [`.qcow2`](https://repository.grid.pub.ro/cs/so/linux-2024/SO-Ubuntu-22-04-03-LTS.utm.zip) image.

After you install `UTM` and download and unzip the archive, you can import it using the `Open existing VM` option in `UTM`.

You can also follow the instructions for [running the VM using `qemu`](https://github.com/cs-pub-ro/operating-systems/blob/main/util/macos-vm/README.md).