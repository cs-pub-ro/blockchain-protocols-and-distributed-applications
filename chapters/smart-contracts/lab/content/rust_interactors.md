# Creating a Rust interactor

Rust interactors are used to interact with the blockchain via Rust.

Let's do this for the empty SC:

```bash
sc-meta all snippets
 /Users/costincarabas/mvx/mx-contracts-rs/contracts/empty

Found 1 contract crates.

(1/1)
In /Users/costincarabas/mvx/mx-contracts-rs/contracts/empty/meta
Calling `cargo run snippets`
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.16s
     Running `/Users/costincarabas/mvx/mx-contracts-rs/target/debug/empty-meta snippets`
```

A new folder `interactor` was created.
This will generate code for all the endpoints and view functions you created.

As this is a new and separate Rust binary, you must add it to the main `Cargo.toml`'s members:

```toml
members = [
[...]
  "contracts/empty/interactor",
[...]
]
```

Now you can use it:
```bash
$ cargo run deploy
[...]
sender's recalled nonce: 10595
-- tx nonce: 10595
sc deploy tx hash: a17a4f51305b6f6dd9c01ec4986d0f90266ef560599b15af613e9aadd816e705
deploy address: erd1qqqqqqqqqqqqqpgqchszakc8fm44c2rndjh09xeuh829g4tgd8sskk0m5e
```

## Practice

* Create an interactor for you Empty contract.
