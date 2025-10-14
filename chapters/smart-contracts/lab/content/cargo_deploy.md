# Smart Contract deployment via Rust


Let's deploy our smart contract(SC) on the blockchain. We will deploy the `adder` contract from the previous section.

In the [repo](https://github.com/multiversx/mx-contracts-rs/tree/main/contracts/adder/) there is a folder named [interact](https://github.com/multiversx/mx-contracts-rs/tree/main/contracts/adder/interact).


Let's take a look into the [basic_interact_cli.rs](https://github.com/multiversx/mx-contracts-rs/blob/add-potlock-contract/contracts/adder/interact/src/basic_interact_cli.rs) file:
```rust
#[derive(Clone, PartialEq, Eq, Debug, Subcommand)]
pub enum InteractCliCommand {
    #[command(name = "add", about = "Add value")]
    Add(AddArgs),
    #[command(name = "deploy", about = "Deploy contract")]
    Deploy,
    #[command(name = "feed", about = "Feed contract EGLD")]
    Feed,
    #[command(name = "multi-deploy", about = "Multiple deploy contracts")]
    MultiDeploy(MultiDeployArgs),
    #[command(name = "sum", about = "Print sum")]
    Sum,
}
```

We have 5 arguments we can use to interact with this contract. We will `deploy` for now.


Let's take a look into the [basic_interact.rs](https://github.com/multiversx/mx-contracts-rs/blob/add-potlock-contract/contracts/adder/interact/src/basic_interact.rs) file:


```Rust
async fn main() {
    env_logger::init();

    let mut basic_interact = AdderInteract::init().await;

    let cli = basic_interact_cli::InteractCli::parse();
    match &cli.command {
        Some(basic_interact_cli::InteractCliCommand::Add(args)) => {
            basic_interact.add(args.value).await;
        }
        Some(basic_interact_cli::InteractCliCommand::Deploy) => {
            basic_interact.deploy().await;
        }
        Some(basic_interact_cli::InteractCliCommand::Feed) => {
            basic_interact.feed_contract_egld().await;
        }
        Some(basic_interact_cli::InteractCliCommand::MultiDeploy(args)) => {
            basic_interact.multi_deploy(&args.count).await;
        }
        Some(basic_interact_cli::InteractCliCommand::Sum) => {
            basic_interact.print_sum().await;
        }
        None => {}
    }
}
```

## SC Deploy

The main function checks the arguments and calls the designated function.
We will be using `deploy` async function to deploy our `adder` smart contract:
```rust
    async fn deploy(&mut self) {
        // warning: multi deploy not yet fully supported
        // only works with last deployed address

        self.set_state().await;

        let new_address = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .typed(adder_proxy::AdderProxy)
            .init(0u32)
            .code(&self.adder_code)
            .returns(ReturnsNewBech32Address)
            .prepare_async()
            .run()
            .await;

        println!("new address: {new_address}");
        self.state.set_adder_address(new_address);
    }
```

The interactor will make a call to the blockchain from a wallet address. This is a test address and it's referenced from the [MultiversX Framework SDK](https://github.com/multiversx/mx-sdk-rs/tree/master/sdk/core/src/test_wallets).


Let's deploy a contract:
```bash
$ cargo run deploy
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.38s
     Running `/Users/costincarabas/mvx/mx-contracts-rs/target/debug/basic-interact deploy`
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
    async fn print_sum(&mut self) {
        let sum = self
            .interactor
            .query()
            .to(self.state.current_adder_address())
            .typed(adder_proxy::AdderProxy)
            .sum()
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("sum: {sum}");
    }
```

Notice that we will make a `query` on the blockchain to retrieve information.

**Question 1:** Will this transaction show on the Explorer?

**Question 2:** Will all the validators execut this transaction?

Remember from the `main` function and the `basic_interact_cli.rs` file that we need to call the `sum` parameter:
```bash
$ cargo run sum
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.36s
     Running `/Users/costincarabas/mvx/mx-contracts-rs/target/debug/basic-interact sum`
sum: 0
```

The `sum` storage is 0, because this was the value used by the interactor.

## SC execute endpoint

Let's execute our first endpoint, the `add` function:


```rust
    async fn add(&mut self, value: u64) {
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_adder_address())
            .typed(adder_proxy::AdderProxy)
            .add(value)
            .prepare_async()
            .run()
            .await;

        println!("successfully performed add");
    }
```

Notice that we use the `tx` functionality of the interactor. We call the `add` endpoint of the smart contract.

Please inspect [basic_interact.rs](https://github.com/multiversx/mx-contracts-rs/blob/add-potlock-contract/contracts/adder/interact/src/basic_interact.rs) file one more time:

```rust
pub struct AddArgs {
    /// The value to add
    #[arg(short = 'v', long = "value", verbatim_doc_comment)]
    pub value: u64,
}
```

We must provide a parameter `-v $value` or `--value $value`. Let's try it both ways:

```bash
$ cargo run add --value 2
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.23s
     Running `/Users/costincarabas/mvx/mx-contracts-rs/target/debug/basic-interact add --value 2`
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.27s
     Running `/Users/costincarabas/mvx/mx-contracts-rs/target/debug/basic-interact sum`
sum: 2
```

Let's try the other way (`-v $value`):

```bash
 $ cargo run add -v 4
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `/Users/costincarabas/mvx/mx-contracts-rs/target/debug/basic-interact add -v 4`
sender's recalled nonce: 5500
-- tx nonce: 5500
sc call tx hash: a7f07975deb4ea01b19572146adb68b1b83abd1e7ed0396475cf389641ab03b5
successfully performed add
```

Let's check that the storage was incremented:
```bash
 $ cargo run sum
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.27s
     Running `/Users/costincarabas/mvx/mx-contracts-rs/target/debug/basic-interact sum`
sum: 6
```

**Question 1:** Will these transactions (endpoint calling) show on the Explorer?

**Question 2:** Will all the validators execute these transactions (endpoint calling)?
