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


Let's compile the contract:

```bash
costin@Byblos:~/mvx/mx-contracts-rs/contracts/adder$ sc-meta all build
 /home/costin/mvx/mx-contracts-rs/contracts/adder

Found 1 contract crates.

(1/1)
In /home/costin/mvx/mx-contracts-rs/contracts/adder/meta
Calling `cargo run build`
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.05s
     Running `/home/costin/mvx/mx-contracts-rs/target/debug/adder-meta build`
Using workspace target directory: /home/costin/mvx/mx-contracts-rs/target ...
Building adder.wasm in /home/costin/mvx/mx-contracts-rs/contracts/adder/wasm ...
RUSTFLAGS="-C link-arg=-s -C link-arg=-zstack-size=131072" cargo build --target=wasm32-unknown-unknown --release --target-dir /home/costin/mvx/mx-contracts-rs/target
   Compiling proc-macro2 v1.0.86
   Compiling unicode-ident v1.0.13
   Compiling smallvec v1.13.2
   Compiling autocfg v1.4.0
   Compiling hex v0.4.3
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
   Compiling adder v0.0.0 (/home/costin/mvx/mx-contracts-rs/contracts/adder)
   Compiling multiversx-sc-wasm-adapter v0.53.2
   Compiling adder-wasm v0.0.0 (/home/costin/mvx/mx-contracts-rs/contracts/adder/wasm)
    Finished `release` profile [optimized] target(s) in 8.69s
Copying /home/costin/mvx/mx-contracts-rs/target/wasm32-unknown-unknown/release/adder_wasm.wasm to ../output/adder.wasm ...
Calling wasm-opt on ../output/adder.wasm ...
Extracting imports to ../output/adder.imports.json ...
Checking EI version: 1.3 ... OK
Packing ../output/adder.mxsc.json ...
Contract size: 696 bytes.
```

Let's check the contract:
```bash
costin@Byblos:~/mvx/mx-contracts-rs/contracts/adder$ ls -l output/
total 16
-rw-rw-r-- 1 costin costin 1792 nov  7 13:28 adder.abi.json
-rw-rw-r-- 1 costin costin  262 nov  7 13:28 adder.imports.json
-rw-rw-r-- 1 costin costin 4070 nov  7 13:28 adder.mxsc.json
-rwxrwxr-x 1 costin costin  696 nov  7 13:28 adder.wasm
```

We notice that the resulted contract (`adder.wasm`) has 696 bytes.

## Practice

* Compile the Adder contract.
