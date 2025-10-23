# MultiversX Command Line

## Question Text

What command is used to create a new MultiversX wallet via command line?

## Question Answers

- `mxpy wallet create --format pem --outfile wallet.pem`
+ `mxpy wallet new --format pem --outfile new_wallet.pem`
- `mxpy generate wallet --type pem --output wallet.pem`
- `mxpy create --wallet --format pem --file wallet.pem`
- `mxpy new --wallet --pem --output wallet.pem`

## Feedback

The correct command is `mxpy wallet new --format pem --outfile new_wallet.pem`. This command creates a new wallet file in PEM format and saves it to the specified output file. The `new` subcommand is used to create a new wallet, and the `--format pem` option specifies the file format.
