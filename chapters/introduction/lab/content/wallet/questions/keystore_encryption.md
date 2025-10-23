# Keystore Encryption

## Question Text

What encryption algorithm is used in MultiversX keystore files?

## Question Answers

- RSA-256
+ AES-128-CTR
- Blowfish
- Triple DES
- ChaCha20

## Feedback

MultiversX keystore files use AES-128-CTR (Advanced Encryption Standard with 128-bit key in Counter mode) for encrypting the private key or mnemonic phrase. AES-128-CTR is a symmetric encryption algorithm that provides strong security while being efficient for encrypting large amounts of data.
