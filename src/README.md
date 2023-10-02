# Introduction

The [RustCrypto] organization provides pure Rust implementations of various cryptographic
algorithms.

## What do I need?

| What you need to do                               | What you should read                                        | Note                                                                                                                                                            |
| ------------------------------------------------- | ----------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Verify that data is correct                       | [Data Hashing](hashing-data/index.html)                     | Hash algorithms like SHA-256 provide a one-way map of data to a fixed-length that can be easily compared                                                        |
| Obscure a password                                | [Password Hashing](hashing-password/index.html)             | Data hash algorithms are designed to be fast; password hash algorithms like `bcrypt`are designed to be slow. Use these to store password results in a database. |
| Prove that public data originally came from _you_ | [Message Authentication](message-authentication/index.html) | MACs use a hash algorithm like SHA-256 with a secret to create a signature for data. With the secret key, you can prove the data is correct.                    |
|                                                   |                                                             |                                                                                                                                                                 |

[rustcrypto]: https://github.com/RustCrypto/
