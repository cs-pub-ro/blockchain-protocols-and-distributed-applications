# The Empty SC

The smallest smart contract is an empty smart contract with no functionalities. Compiled, it is a binary that is accepted by the blockchain.

```rust
/// An empty contract. To be used as a template when starting a new contract from scratch.
#[multiversx_sc::contract]
pub trait EmptyContract {
    #[init]
    fn init(&self) {}
}
```

[Here](https://github.com/multiversx/mx-contracts-rs/blob/main/contracts/empty/src/empty.rs) is the smart contract code listed above and [here](https://github.com/multiversx/mx-contracts-rs/tree/main/contracts/empty) are all the files needed for compilation. 

We have the annotation `#[init]` where we specify the constructor function. Only one constructor is allowed per smart contract.

Let's build the smart contract:

```bash
costin@Byblos:~/mvx/mx-contracts-rs/contracts/empty$ sc-meta all build
 /home/costin/mvx/mx-contracts-rs/contracts/empty

Found 1 contract crates.

(1/1)
In /home/costin/mvx/mx-contracts-rs/contracts/empty/meta
Calling `cargo run build`
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.05s
     Running `/home/costin/mvx/mx-contracts-rs/target/debug/empty-meta build`
Using workspace target directory: /home/costin/mvx/mx-contracts-rs/target ...
Building empty.wasm in /home/costin/mvx/mx-contracts-rs/contracts/empty/wasm ...
RUSTFLAGS="-C link-arg=-s -C link-arg=-zstack-size=131072" cargo build --target=wasm32-unknown-unknown --release --target-dir /home/costin/mvx/mx-contracts-rs/target
   Compiling proc-macro2 v1.0.86
   Compiling unicode-ident v1.0.13
   Compiling hex v0.4.3
   Compiling smallvec v1.13.2
   Compiling autocfg v1.4.0
   Compiling endian-type v0.1.2
   Compiling arrayvec v0.7.6
   Compiling unwrap-infallible v0.1.5
   Compiling bitflags v2.6.0
   Compiling hex-literal v0.4.1
   Compiling nibble_vec v0.1.0
   Compiling radix_trie v0.2.1
   Compiling num-traits v0.2.19
   Compiling quote v1.0.37
   Compiling syn v2.0.77
   Compiling multiversx-sc-codec-derive v0.21.0
   Compiling multiversx-sc-derive v0.53.2
   Compiling multiversx-sc-codec v0.21.0
   Compiling multiversx-sc v0.53.2
   Compiling empty v0.0.0 (/home/costin/mvx/mx-contracts-rs/contracts/empty)
   Compiling multiversx-sc-wasm-adapter v0.53.2
   Compiling empty-wasm v0.0.0 (/home/costin/mvx/mx-contracts-rs/contracts/empty/wasm)
    Finished `release` profile [optimized] target(s) in 8.17s
Copying /home/costin/mvx/mx-contracts-rs/target/wasm32-unknown-unknown/release/empty_wasm.wasm to ../output/empty.wasm ...
Calling wasm-opt on ../output/empty.wasm ...
Extracting imports to ../output/empty.imports.json ...
Checking EI version: 1.3 ... OK
Packing ../output/empty.mxsc.json ...
Contract size: 231 bytes.
```

The resulted contract is `output/empty.wasm`, a WebAseembly binary module that is only 232 bytes:

```
costin@Byblos:~/mvx/mx-contracts-rs/contracts/empty$ ls -l output/
total 16
-rw-rw-r-- 1 costin costin  868 nov  7 12:17 empty.abi.json
-rw-rw-r-- 1 costin costin   60 nov  7 12:17 empty.imports.json
-rw-rw-r-- 1 costin costin 1774 nov  7 12:17 empty.mxsc.json
-rwxrwxr-x 1 costin costin  231 nov  7 12:17 empty.wasm
```

## Practice

* Clone the Empty SC repo;
* Compile the contract.
