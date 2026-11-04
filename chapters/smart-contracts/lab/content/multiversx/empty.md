# The Empty SC

The smallest smart contract is an empty smart contract with no functionalities. Compiled, it is a binary that is accepted by the blockchain.

```rust
/// An empty contract. To be used as a template when starting a new contract from scratch.
#[multiversx_sc::contract]
pub trait EmptyContract {
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}
}
```

[Here](https://github.com/multiversx/mx-contracts-rs/blob/main/contracts/empty/src/empty.rs) is the smart contract code listed above and [here](https://github.com/multiversx/mx-contracts-rs/tree/main/contracts/empty) are all the files needed for compilation.

You can obtain this minimal contract code directly using the following command:

```bash
sc-meta new --template empty
```

We have the annotation:

- `#[init]` where we specify the constructor function;
- `#[upgrade]` which allows to update the contract code without changing its deploy address.

:::note
Only **one** constructor is allowed per smart contract.
:::

Let's build the smart contract, running the next command in the contract root:

```sh
sc-meta all build
```

Below, it is the output of the command:

```bash
 /home/empty

Found 1 contract crates.

(1/1)
In /home/empty/meta
Calling `cargo run build`
    Blocking waiting for file lock on build directory
    ...
    Compiling empty v0.0.0 (/home/empty)
    Compiling wasmprinter v0.239.0
    Compiling multiversx-sc-meta-lib v0.62.1
    Compiling empty-meta v0.0.0 (/home/empty/meta)
      Finished `dev` profile [unoptimized + debuginfo] target(s) in 18.10s
Copying /empty/target/wasm32v1-none/release/empty_wasm.wasm to ../output/empty.wasm ...
Calling wasm-opt on ../output/empty.wasm ...
Extracting imports to ../output/empty.imports.json ...
Checking EI version: 1.5 ... OK
Packing ../output/empty.mxsc.json ...
Contract size: 244 bytes.
```

The resulted contract is `output/empty.wasm`, a WebAssembly binary module that is only 244 bytes:

```sh
ls -l output/
total 16
-rw-rw-r-- 1 user user  874 nov 13 12:46 empty.abi.json
-rw-rw-r-- 1 user user   60 nov 13 12:47 empty.imports.json
-rw-rw-r-- 1 user user 1889 nov 13 12:47 empty.mxsc.json
-rwxrwxr-x 1 user user  245 nov 13 12:47 empty.wasm
```

## Practice

- Create Empty SC using `sc-meta`;
- Compile the contract.
