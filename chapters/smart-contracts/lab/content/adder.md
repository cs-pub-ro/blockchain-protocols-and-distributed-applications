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

    /// Add desired amount to the storage variable.
    #[payable("*")]
    #[endpoint]
    fn add(&self, value: BigUint) {
        self.sum().update(|sum| *sum += value);
    }
}
```

We notice 3 functions:
* sum - this is a global variable, a `SingleValueMapper` (a single value) of type BigUint (unsigned integer);
* init - the constructor;
* add - function that increments the global variable (`sum`) with the `value` parameter;

We notice 5 types of adnotations:
* #[view(getSum)] - this is a function that allows you to read the variable by calling the paramter(`getSum`);
* #[storage_mapper("sum")] - this is a global variable (also called a storage) stored in the contract
* #[init] - the constructor function
* #[endpoint] - An endpoint is a function callable directly by the user.


[Here](https://github.com/multiversx/mx-contracts-rs/blob/main/contracts/adder/src/adder.rs) is the smart contract code listed above and [here](https://github.com/multiversx/mx-contracts-rs/tree/main/contracts/adder) are all the files needed for compilation. 


Let's compile the contract:

```bash
costin@Byblos:~/mvx/mx-contracts-rs/contracts/adder$ sc-meta all build
 /home/costin/mvx/mx-contracts-rs/contracts/adder

Found 1 contract crates.

(1/1)
In /home/costin/mvx/mx-contracts-rs/contracts/adder/meta
Calling `cargo run build`
    Finished dev [unoptimized + debuginfo] target(s) in 0.08s
     Running `/home/costin/mvx/mx-contracts-rs/target/debug/adder-meta build`
Building adder.wasm in /home/costin/mvx/mx-contracts-rs/contracts/adder/wasm ...
RUSTFLAGS="-C link-arg=-s -C link-arg=-zstack-size=131072" cargo build --target=wasm32-unknown-unknown --release
   Compiling proc-macro2 v1.0.69
   Compiling unicode-ident v1.0.12
   Compiling syn v1.0.109
   Compiling version_check v0.9.4
   Compiling hex v0.4.3
   Compiling smallvec v1.11.1
   Compiling autocfg v1.1.0
   Compiling cfg-if v1.0.0
   Compiling zerocopy v0.7.25
   Compiling endian-type v0.1.2
   Compiling nibble_vec v0.1.0
   Compiling once_cell v1.18.0
   Compiling radix_trie v0.2.1
   Compiling arrayvec v0.7.4
   Compiling bitflags v1.3.2
   Compiling hex-literal v0.3.4
   Compiling num-traits v0.2.17
   Compiling ahash v0.8.6
   Compiling hashbrown v0.13.2
   Compiling quote v1.0.33
   Compiling multiversx-sc-codec-derive v0.18.1
   Compiling multiversx-sc-derive v0.44.0
   Compiling multiversx-sc-codec v0.18.1
   Compiling multiversx-sc v0.44.0
   Compiling multiversx-sc-wasm-adapter v0.44.0
   Compiling adder v0.0.0 (/home/costin/mvx/mx-contracts-rs/contracts/adder)
   Compiling adder-wasm v0.0.0 (/home/costin/mvx/mx-contracts-rs/contracts/adder/wasm)
    Finished release [optimized] target(s) in 5.31s
Copying ../wasm/target/wasm32-unknown-unknown/release/adder_wasm.wasm to ../output/adder.wasm ...
Calling wasm-opt on ../output/adder.wasm ...
Extracting imports to ../output/adder.imports.json ...
Checking EI version: 1.2 ... OK
Packing ../output/adder.mxsc.json ...
Contract size: 685 bytes.
```

Let's check the contract:
```bash
costin@Byblos:~/mvx/mx-contracts-rs/contracts/adder$ ls -l output/
total 16
-rw-rw-r-- 1 costin costin 1639 dec  6 17:02 adder.abi.json
-rw-rw-r-- 1 costin costin  262 dec  6 17:02 adder.imports.json
-rw-rw-r-- 1 costin costin 3249 dec  6 17:02 adder.mxsc.json
-rwxrwxr-x 1 costin costin  685 dec  6 17:02 adder.wasm
```

We notice that the resulted contract (`adder.wasm`) has 685 bytes.