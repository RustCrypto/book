# Introduction

The [RustCrypto] organization provides pure Rust implementations of various cryptographic
algorithms.

This book is a work in progress.

## What do I need?

The below is a quick overview of the RustCrypto organization's projects based on the use case:

| Action                                              | Relevant Section                                            | Note                                                                                                                                                                 |
| --------------------------------------------------- | ----------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Verify that data is correct                         | [Data Hashing](hashing-data/index.html)                     | Hash algorithms like SHA-256 provide a one-way map of data to a fixed-length that can be easily compared                                                             |
| Obscure a password                                  | [Password Hashing](hashing-password/index.html)             | Data hash algorithms are designed to be fast; password hash algorithms like `bcrypt`are designed to be slow. Use these to store password results in a database.      |
| Prove that public data originally came from you     | [Message Authentication](message-authentication/index.html) | MACs use a hash algorithm like SHA-256 with a secret to create a signature for data. With the secret key, you can prove the data is correct.                         |
| Prove that public data originally came from someone | [Digital Signatures](signing/index.html)                    | Digital signatures provide a way to verify data came from a specific source using their public key. Roughly a reverse asymmetric encryption.                         |
| Hide data with a shared secret                      | [Symmetric Encryption](encryption-symmetric/index.html)     | When the encryptor and decryptor can both use the same key, symmetric encryption is used. AES and ChaCha are examples.                                               |
| Hide data without a shared secret                   | [Asymmetric Encryption](encryption-asymmetric/index.html)   | When the encryptor and decryptor do not share a common key, asymmetric encryption can be used to exchange data (such as a new common key). RSA and ECDH are examples |

[rustcrypto]: https://github.com/RustCrypto/
