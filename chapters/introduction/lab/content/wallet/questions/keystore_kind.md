# Keystore File Security

## Question Text

In a MultiversX keystore file, what does the "kind" field specify?

## Question Answers

- The encryption algorithm used
+ Whether the file contains an encrypted secret key or mnemonic
- The version of the keystore format
- The password strength requirements
- The network type (mainnet/testnet)

## Feedback

The "kind" field in a MultiversX keystore file specifies whether the encrypted content is a secret key or a mnemonic phrase. This allows the wallet to handle both older keystore files (which contained encrypted secret keys) and newer ones (which contain encrypted mnemonic phrases) for backward compatibility.
