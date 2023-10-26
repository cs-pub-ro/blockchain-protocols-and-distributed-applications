# Setting up an Observer

In this section we will be setting up an Observer on the [MultiversX Testnet](https://testnet-explorer.multiversx.com/).

Clone the installer repository:

```shell
git clone https://github.com/multiversx/mx-chain-scripts
```

Edit `ENVIRONMENT` and `CUSTOM_HOME` in `config/variables.cfg` config file:

```
ENVIRONMENT="testnet"

CUSTOM_HOME="/home/costin"
```

Please check that the `CUSTOM_HOME`` directory exists. Run the installation script as follows:

```
./script.sh observing_squad
```

Start the nodes and the Proxy using the command:

```
./script.sh start
```

If you encounter any issue please check the [MultiversX Observing Squad Documentation](https://docs.multiversx.com/integrators/observing-squad/).
