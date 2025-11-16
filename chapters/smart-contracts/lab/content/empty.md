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

***
**NOTE**

Only **one** constructor and **one** upgrade method are allowed per smart contract.
***

Let's build the smart contract, running the next command in the **contract root**:

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
   Compiling serde_core v1.0.228
   Compiling num-traits v0.2.19
   Compiling hex v0.4.3
   Compiling bitflags v2.10.0
   Compiling hashbrown v0.16.0
   Compiling arrayvec v0.7.6
   Compiling utf8parse v0.2.2
   Compiling unwrap-infallible v0.1.5
   Compiling equivalent v1.0.2
   Compiling typenum v1.19.0
   Compiling semver v1.0.27
   Compiling foldhash v0.1.5
   Compiling anstyle-parse v0.2.7
   Compiling memchr v2.7.6
   Compiling anstyle-query v1.1.4
   Compiling colorchoice v1.0.4
   Compiling anstyle v1.0.13
   Compiling leb128fmt v0.1.0
   Compiling is_terminal_polyfill v1.70.2
   Compiling clap_lex v0.7.6
   Compiling hex-literal v1.1.0
   Compiling bumpalo v3.19.0
   Compiling multiversx-sc-codec-derive v0.23.1
   Compiling wasm-encoder v0.240.0
   Compiling anstream v0.6.21
   Compiling multiversx-sc-derive v0.62.1
   Compiling winnow v0.7.13
   Compiling strsim v0.11.1
   Compiling unicode-width v0.2.2
   Compiling anyhow v1.0.100
   Compiling clap_builder v4.5.51
   Compiling unicode-segmentation v1.12.0
   Compiling generic-array v1.3.5
   Compiling termcolor v1.4.1
   Compiling num-integer v0.1.46
   Compiling toml_writer v1.0.4
   Compiling ryu v1.0.20
   Compiling itoa v1.0.15
   Compiling convert_case v0.8.0
   Compiling rustc_version v0.4.1
   Compiling lazy_static v1.5.0
   Compiling colored v3.0.0
   Compiling num-bigint v0.4.6
   Compiling wast v240.0.0
   Compiling toml_parser v1.0.4
   Compiling multiversx-sc-codec v0.23.1
   Compiling indexmap v2.12.0
   Compiling serde v1.0.228
   Compiling serde_spanned v1.0.3
   Compiling toml_datetime v0.7.3
   Compiling serde_json v1.0.145
   Compiling clap v4.5.51
   Compiling hashbrown v0.15.5
   Compiling toml v0.9.8
   Compiling multiversx-chain-core v0.19.1
   Compiling multiversx-sc v0.62.1
   Compiling wasmparser v0.239.0
   Compiling wat v1.240.0
   Compiling empty v0.0.0 (/home/empty)
   Compiling wasmprinter v0.239.0
   Compiling multiversx-sc-meta-lib v0.62.1
   Compiling empty-meta v0.0.0 (/home/empty/meta)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 18.10s
     Running `/home/empty/target/debug/empty-meta build`
Using workspace target directory: /home/empty/target ...
Building empty.wasm in /home/empty/wasm ...
RUSTFLAGS="-C link-arg=-s -C link-arg=-zstack-size=131072" cargo build --target=wasm32v1-none --release --target-dir /home/empty/target
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 25 packages to latest compatible versions
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
  Downloaded multiversx-sc-wasm-adapter v0.62.1
  Downloaded 1 crate (17.3 KB) in 0.21s
    Blocking waiting for file lock on package cache
   Compiling proc-macro2 v1.0.103
   Compiling unicode-ident v1.0.22
   Compiling quote v1.0.42
   Compiling hex v0.4.3
   Compiling autocfg v1.5.0
   Compiling typenum v1.19.0
   Compiling smallvec v1.15.1
   Compiling rustversion v1.0.22
   Compiling endian-type v0.2.0
   Compiling unwrap-infallible v0.1.5
   Compiling arrayvec v0.7.6
   Compiling bitflags v2.10.0
error[E0463]: can't find crate for `core`
  |
  = note: the `wasm32v1-none` target may not be installed
  = help: consider downloading the target with `rustup target add wasm32v1-none`

For more information about this error, try `rustc --explain E0463`.
error: could not compile `bitflags` (lib) due to 1 previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `unwrap-infallible` (lib) due to 1 previous error
error: could not compile `arrayvec` (lib) due to 1 previous error

Installing target "wasm32v1-none"...
info: downloading component 'rust-std' for 'wasm32v1-none'
info: installing component 'rust-std' for 'wasm32v1-none'
wasm32v1-none target installed successfully
   Compiling proc-macro2 v1.0.103
   Compiling quote v1.0.42
   Compiling arrayvec v0.7.6
   Compiling bitflags v2.10.0
   Compiling unwrap-infallible v0.1.5
   Compiling rustversion v1.0.22
   Compiling nibble_vec v0.1.0
   Compiling num-traits v0.2.19
   Compiling typenum v1.19.0
   Compiling hex-literal v1.1.0
   Compiling radix_trie v0.3.0
   Compiling syn v2.0.110
   Compiling generic-array v1.3.5
   Compiling multiversx-sc-codec-derive v0.23.1
   Compiling multiversx-sc-derive v0.62.1
   Compiling multiversx-sc-codec v0.23.1
   Compiling multiversx-chain-core v0.19.1
   Compiling multiversx-sc v0.62.1
   Compiling empty v0.0.0 (/home/empty)
   Compiling multiversx-sc-wasm-adapter v0.62.1
   Compiling empty-wasm v0.0.0 (/home/empty/wasm)
    Finished `release` profile [optimized] target(s) in 5.42s
Copying /home/empty/target/wasm32v1-none/release/empty_wasm.wasm to ../output/empty.wasm ...
Calling wasm-opt on ../output/empty.wasm ...
Extracting imports to ../output/empty.imports.json ...
Checking EI version: 1.5 ... OK
Packing ../output/empty.mxsc.json ...
Contract size: 245 bytes.
```

The resulted contract is `output/empty.wasm`, a WebAssembly binary module that is only 245 bytes:

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
