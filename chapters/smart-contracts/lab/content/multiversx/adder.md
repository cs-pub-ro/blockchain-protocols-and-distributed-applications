# The Adder SC

The Adder smart contract is a simple smart contract with an `add` functionality and a global variable that can be incremented.

```rust
/// One of the simplest smart contracts possible,
/// it holds a single variable in storage, which anyone can increment.
#[multiversx_sc::contract]
pub trait Adder {
    #[view(getSum)]
    #[storage_mapper("sum")]
    fn sum(&self) -> SingleValueMapper<BigUint>;

    #[init]
    fn init(&self, initial_value: BigUint) {
        self.sum().set(initial_value);
    }

    #[upgrade]
    fn upgrade(&self, initial_value: BigUint) {
        self.init(initial_value);
    }

    /// Add desired amount to the storage variable.
    #[endpoint]
    fn add(&self, value: BigUint) {
        self.sum().update(|sum| *sum += value);
    }
}
```

We notice 4 functions:

* **sum** - this is a global variable, a `SingleValueMapper` (a single value) of type BigUint (unsigned integer);
* **init** - the constructor;
* **add** - function that increments the global variable (`sum`) with the `value` parameter;
* **upgrade** - function executed when upgrading the contract.

We notice 5 types of annotations:

* `#[view(getSum)]` - this is a function that allows you to read the storage variable by calling the function `getSum`;
* `#[storage_mapper("sum")]` - this is a global **variable** (also called a storage) stored at the contract address;
* `#[init]` - the constructor function; this is called when deploying the contract;
* `#[upgrade]`  - this function is called when upgrading the contract;
* `#[endpoint]` - an endpoint is a function callable directly by the user; A function not having this annotation will not be exposed publicly.

[Here](https://github.com/multiversx/mx-contracts-rs/blob/main/contracts/adder/src/adder.rs) is the smart contract code listed above and [here](https://github.com/multiversx/mx-contracts-rs/tree/main/contracts/adder) are all the files needed for compilation.

You can obtain the adder smart contract code directly using the following command:

```bash
sc-meta new --template adder
```

Let's build the smart contract, running the next command in the contract root:

```sh
sc-meta all build
```

Below, it is the output of the command:

```bash
 /home/adder

Found 1 contract crates.

(1/1)
In /home/adder/meta
Calling `cargo run build`
   Compiling syn v2.0.110
   Compiling serde_core v1.0.228
   Compiling num-traits v0.2.19
   Compiling bitflags v2.10.0
   Compiling hex v0.4.3
   Compiling equivalent v1.0.2
   Compiling utf8parse v0.2.2
   Compiling unwrap-infallible v0.1.5
   Compiling hashbrown v0.16.0
   Compiling arrayvec v0.7.6
   Compiling typenum v1.19.0
   Compiling semver v1.0.27
   Compiling anstyle-query v1.1.4
   Compiling memchr v2.7.6
   Compiling anstyle-parse v0.2.7
   Compiling colorchoice v1.0.4
   Compiling is_terminal_polyfill v1.70.2
   Compiling leb128fmt v0.1.0
   Compiling foldhash v0.1.5
   Compiling anstyle v1.0.13
   Compiling unicode-width v0.2.2
   Compiling serde_json v1.0.145
   Compiling wasm-encoder v0.240.0
   Compiling winnow v0.7.13
   Compiling hex-literal v1.1.0
   Compiling anstream v0.6.21
   Compiling strsim v0.11.1
   Compiling bumpalo v3.19.0
   Compiling clap_lex v0.7.6
   Compiling anyhow v1.0.100
   Compiling ryu v1.0.20
   Compiling clap_builder v4.5.51
   Compiling toml_writer v1.0.4
   Compiling generic-array v1.3.5
   Compiling itoa v1.0.15
   Compiling unicode-segmentation v1.12.0
   Compiling termcolor v1.4.1
   Compiling rustc_version v0.4.1
   Compiling lazy_static v1.5.0
   Compiling colored v3.0.0
   Compiling num-integer v0.1.46
   Compiling convert_case v0.8.0
   Compiling num-bigint v0.4.6
   Compiling wast v240.0.0
   Compiling toml_parser v1.0.4
   Compiling indexmap v2.12.0
   Compiling serde_spanned v1.0.3
   Compiling toml_datetime v0.7.3
   Compiling toml v0.9.8
   Compiling multiversx-sc-codec-derive v0.23.1
   Compiling serde_derive v1.0.228
   Compiling multiversx-sc-derive v0.62.1
   Compiling clap_derive v4.5.49
   Compiling multiversx-sc-codec v0.23.1
   Compiling clap v4.5.51
   Compiling multiversx-chain-core v0.19.1
   Compiling multiversx-sc v0.62.1
   Compiling serde v1.0.228
   Compiling hashbrown v0.15.5
   Compiling wat v1.240.0
   Compiling wasmparser v0.239.0
   Compiling adder v0.0.0 (/home/adder)
   Compiling wasmprinter v0.239.0
   Compiling multiversx-sc-meta-lib v0.62.1
   Compiling adder-meta v0.0.0 (/home/adder/meta)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 13.82s
     Running `/home/adder/target/debug/adder-meta build`
Using workspace target directory: /home/adder/target ...
Building adder.wasm in /home/adder/wasm ...
RUSTFLAGS="-C link-arg=-s -C link-arg=-zstack-size=131072" cargo build --target=wasm32v1-none --release --target-dir /home/adder/target
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 25 packages to latest compatible versions
    Blocking waiting for file lock on package cache
   Compiling proc-macro2 v1.0.103
   Compiling unicode-ident v1.0.22
   Compiling quote v1.0.42
   Compiling hex v0.4.3
   Compiling smallvec v1.15.1
   Compiling typenum v1.19.0
   Compiling rustversion v1.0.22
   Compiling autocfg v1.5.0
   Compiling bitflags v2.10.0
   Compiling endian-type v0.2.0
   Compiling unwrap-infallible v0.1.5
   Compiling arrayvec v0.7.6
   Compiling hex-literal v1.1.0
   Compiling num-traits v0.2.19
   Compiling nibble_vec v0.1.0
   Compiling radix_trie v0.3.0
   Compiling syn v2.0.110
   Compiling generic-array v1.3.5
   Compiling multiversx-sc-codec-derive v0.23.1
   Compiling multiversx-sc-derive v0.62.1
   Compiling multiversx-sc-codec v0.23.1
   Compiling multiversx-chain-core v0.19.1
   Compiling multiversx-sc v0.62.1
   Compiling adder v0.0.0 (/home/adder)
   Compiling multiversx-sc-wasm-adapter v0.62.1
   Compiling adder-wasm v0.0.0 (/home/adder/wasm)
    Finished `release` profile [optimized] target(s) in 8.55s
Copying /home/adder/target/wasm32v1-none/release/adder_wasm.wasm to ../output/adder.wasm ...
Calling wasm-opt on ../output/adder.wasm ...
Extracting imports to ../output/adder.imports.json ...
Checking EI version: 1.5 ... OK
Packing ../output/adder.mxsc.json ...
Contract size: 700 bytes.
```

Let's check the contract:

```bash
ls -l output/
total 16
-rw-rw-r-- 1 user user 1727 nov 13 15:09 adder.abi.json
-rw-rw-r-- 1 user user  262 nov 13 15:09 adder.imports.json
-rw-rw-r-- 1 user user 4078 nov 13 15:09 adder.mxsc.json
-rwxrwxr-x 1 user user  700 nov 13 15:09 adder.wasm
```

We notice that the resulted contract (`adder.wasm`) has 700 bytes.

## Practice

* Compile the Adder contract.
