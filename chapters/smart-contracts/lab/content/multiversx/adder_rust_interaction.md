# Smart Contract deployment via Rust

Let's deploy our smart contract (SC) on the blockchain. We will deploy the `adder` contract from the previous section.

There is a folder named [interactor](https://github.com/multiversx/mx-contracts-rs/tree/main/contracts/adder/interactor).

Let's take a look into the [basic_interactor_cli.rs](https://github.com/multiversx/mx-contracts-rs/blob/add-potlock-contract/contracts/adder/interact/src/basic_interact_cli.rs) file:

```rust
#[derive(Clone, PartialEq, Eq, Debug, Subcommand)]
pub enum InteractCliCommand {
    #[command(name = "deploy", about = "Deploy contract")]
    Deploy,
    #[command(name = "upgrade", about = "Upgrade contract")]
    Upgrade(UpgradeArgs),
    #[command(name = "sum", about = "Print sum")]
    Sum,
    #[command(name = "add", about = "Add value")]
    Add(AddArgs),
}
```

We have 4 arguments we can use to interact with this contract. We will `deploy` for now.

Let's take a look into the [basic_interactor.rs](https://github.com/multiversx/mx-contracts-rs/blob/main/contracts/adder/interactor/src/basic_interactor.rs) file:

```Rust
pub async fn adder_cli() {
    env_logger::init();

    let config = Config::load_config();

    let mut basic_interact = BasicInteractor::new(config).await;

    let cli = basic_interactor_cli::InteractCli::parse();
    match &cli.command {
        Some(basic_interactor_cli::InteractCliCommand::Deploy) => {
            basic_interact.deploy().await;
        }
        Some(basic_interactor_cli::InteractCliCommand::Upgrade(args)) => {
            let owner_address = basic_interact.adder_owner_address.clone();
            basic_interact
                .upgrade(args.value, &owner_address, None)
                .await
        }
        Some(basic_interactor_cli::InteractCliCommand::Add(args)) => {
            basic_interact.add(args.value).await;
        }
        Some(basic_interactor_cli::InteractCliCommand::Sum) => {
            let sum = basic_interact.get_sum().await;
            println!("sum: {sum}");
        }
        None => {}
    }
}
```

## SC Deploy

The `adder_cli` function checks the arguments and calls the designated function.

We will be using `deploy` async function to deploy our `adder` smart contract:

```rust
pub async fn deploy(&mut self) {
    let new_address = self
        .interactor
        .tx()
        .from(&self.adder_owner_address.clone())
        .gas(100_000_000)
        .typed(adder_proxy::AdderProxy)
        .init(0u64)
        .code(ADDER_CODE_PATH)
        .returns(ReturnsNewBech32Address)
        .run()
        .await;

    println!("new address: {new_address}");
    self.state.set_adder_address(new_address);
}
```

Let’s break down the deploy command line by line. It follows a [unified syntax](https://docs.multiversx.com/developers/transactions/tx-overview), meaning the same structure is used for **contract development**, **interaction**, and **testing**.

* `.interactor.tx()` - Initializes the [environment](https://docs.multiversx.com/developers/transactions/tx-env#interactor) where an empty transaction block is created.
* `.from(&self.adder_owner_address.clone())` – Specifies the [sender](https://docs.multiversx.com/developers/transactions/tx-from#explicit-sender) of the transaction.
* `.gas(100_000_000)` - Sets the amount of [gas](https://docs.multiversx.com/developers/transactions/tx-gas) allocated for executing the transaction.
* `.typed(adder_proxy::AdderProxy).init(0u64)` - Invokes the deploy function through a [proxy](https://docs.multiversx.com/developers/transactions/tx-proxies). The proxy is an object that mirrors the smart contract, exposing methods with the same names and arguments as those defined in the contract. The corresponding code is located in `/src/adder_proxy.rs` (or can be found [here](https://github.com/multiversx/mx-contracts-rs/blob/main/contracts/adder/src/adder_proxy.rs)).
* `.code(ADDER_CODE_PATH)` - Attaches the smart contract’s bytecode to the deployment transaction.
* `.returns(ReturnsNewBech32Address)` - Defines the transaction’s expected output. In this case, it returns the deployed **contract’s address**. You can find all result handler types [here](https://docs.multiversx.com/developers/transactions/tx-result-handlers#list-of-result-decoders).
* `.run().await` – Executes the transaction.

The interactor will make a call to the blockchain from a wallet address. This is a test address and it's referenced from the [MultiversX Framework SDK](https://github.com/multiversx/mx-sdk-rs/tree/master/sdk/core/src/test_wallets).

Let's deploy a contract by running the following command in the `interactor` directory:

```bash
cargo run deploy
```

Below, it is the output of the command:

```sh
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.38s
     Running `/home/adder/target/debug/basic-interactor deploy`
wallet address: erd1uv40ahysflse896x4ktnh6ecx43u7cmy9wnxnvcyp7deg299a4sq6vaywa
sender's recalled nonce: 5498
-- tx nonce: 5498
sc deploy tx hash: 4133385c40fee378f5fc1b6318f53302750c2403a9d02e00ed727c35ba9b41ba
deploy address: erd1qqqqqqqqqqqqqpgqef8xmsatt4tkf5ycv538a2kme3h7dy37a4sqygv9p5
```

Notice the:

* sender's nonce - how many transactions he initiated;
* sc deploy tx hash - the hash of the transaction where we deployed the code on a new SC;
* deploy address - the address where the new SC is located.

**Question 1:** Will this transaction show on the Explorer?

**Question 2:** Will all the validators execute this transaction?

## SC Query

Let's read the storage from the SC:

```rust
pub async fn get_sum(&mut self) -> RustBigUint {
    self.interactor
        .query()
        .to(self.state.current_adder_address())
        .typed(adder_proxy::AdderProxy)
        .sum()
        .returns(ReturnsResultUnmanaged)
        .run()
        .await
}
```

Notice that we will make a `query` on the blockchain to retrieve information. Let's break down the command line by line

* `.interactor.query()` - Initializes the [environment](https://docs.multiversx.com/developers/transactions/tx-env#interactor) where an empty query block is created. Smart contract queries, or view functions, are endpoints that only read data from the contract. It has **no** cost.
* `.to(self.state.current_adder_address())` – Specifies the [receiver](https://docs.multiversx.com/developers/transactions/tx-to/) of the transaction.
* `.typed(adder_proxy::AdderProxy).sum()` - Invokes the query function through a [proxy](https://docs.multiversx.com/developers/transactions/tx-proxies).
* `.returns(ReturnsResultUnmanaged)` - Defines the transaction’s expected output. It returns the unmanaged version of the original result , which in this case is `RustBigUint`.
* `.run().await` – Executes the transaction.

**Question 1:** Will this transaction show on the Explorer?

**Question 2:** Will all the validators execute this transaction?

Remember from the `adder_cli` function and the `basic_interactor_cli.rs` file that we need to call `sum`. Keep in mind that this command is only available in the `interactor` directory:

```bash
cargo run sum
```

The output of the command is shown below:

```sh
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `/home/adder/target/debug/basic-interactor sum`
sum: 0
```

The `sum` storage is 0, because this was the value set in the deploy call by the interactor.

## SC execute endpoint

Let's execute our first endpoint, the `add` function:

```rust
pub async fn add(&mut self, value: u32) {
    self.interactor
        .tx()
        .from(&self.wallet_address)
        .to(self.state.current_adder_address())
        .gas(6_000_000u64)
        .typed(adder_proxy::AdderProxy)
        .add(value)
        .run()
        .await;

    println!("Successfully performed add");
}
```

Let's break down the command line by line:

* `.interactor.tx()` - Initializes the [environment](https://docs.multiversx.com/developers/transactions/tx-env#interactor) where an empty transaction is created.
* `.from(&self.wallet_address)` - The [sender](https://docs.multiversx.com/developers/transactions/tx-from#explicit-sender) of the transaction.
* `.to(self.state.current_adder_address())` – The [receiver](https://docs.multiversx.com/developers/transactions/tx-to/) of the transaction.
* `.gas(6_000_000u64)` - Sets the amount of [gas](https://docs.multiversx.com/developers/transactions/tx-gas) allocated for executing the transaction.
* `.typed(adder_proxy::AdderProxy).add(value)` - Invokes the endpoint method through a [proxy](https://docs.multiversx.com/developers/transactions/tx-proxies).
* `.run().await` – Executes the transaction.

Please inspect [basic_interactor.rs](https://github.com/multiversx/mx-contracts-rs/blob/add-potlock-contract/contracts/adder/interactor/src/basic_interactor.rs) file one more time:

```rust
pub struct AddArgs {
    /// The value to add
    #[arg(short = 'v', long = "value")]
    pub value: u32,
}
```

We must provide a parameter `-v $value` or `--value $value`. Let's try it both ways:

```bash
cargo run add --value 2
```

This is the output of the command:

```sh
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
     Running `/home/adder/target/debug/basic-interactor add --value 2`
sender's recalled nonce: 5499
-- tx nonce: 5499
sc call tx hash: 2b85ad0af8bed72f415abcf03ed1615ab2e72c33354400f66aafd22346d1871c
successfully performed add
```

Notice:

* tx nonce - this is the sender's nonce;
* sc call tx hash - this is the hash of the transaction we generated;

We recommend checking every action you perform:

```bash
 $ cargo run sum
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `/home/adder/target/debug/basic-interactor sum`
sum: 2
```

Let's try the other way (`-v $value`):

```bash
cargo run add -v 4
```

```sh
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
     Running `/home/adder/target/debug/basic-interactor add --value 4`
sender's recalled nonce: 5500
-- tx nonce: 5500
sc call tx hash: a7f07975deb4ea01b19572146adb68b1b83abd1e7ed0396475cf389641ab03b5
successfully performed add

```

Let's check that the storage was incremented:

```bash
 $ cargo run sum
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.27s
     Running `/home/adder/target/debug/basic-interactor sum`
sum: 6
```

**Question 1:** Will these transactions (endpoint calling) show on the Explorer?

**Question 2:** Will all the validators execute these transactions (endpoint calling)?
