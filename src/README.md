# Introduction

The [RustCrypto] organization provides pure Rust implementations of various cryptographic
algorithms.

This book aims to provide an overview of different cryptographic needs are and give guidance on how
to use RustCrypto crates to meet them.

This book is a work in progress.

## What is RustCrypto?

RustCrypto is an organization that focuses on providing cryptographic primitives written in pure
Rust. This means that the libraries are typically easily portable, conveniently generic, and
difficult to trivially misuse.

## What is Cryptography?

In general, cryptography is the practice of developing techniques for secure communication. There
are three core properties that cryptographic algorithms may provide:

1. Integrity (**I**): Verifying that data has not been tampered with (e.g. checking that nobody
   replaced a file with a malicious one)
1. Authenticity (**A**): Verifying that data came from the correct party (e.g. checking that public
   data originated from you and not a malicious party)
1. Confidentiality (**C**): Ensuring that data can only be accessed by the intended parties (e.g.
   hiding your data from malicious parties)

Usually _hashing_ algorithms provide integrity, _authentication_ or _signing_ algorithms provide
integrity and authenticity, and _encryption_ algorithms provide confidentiality (but not always
integrity!).

See the [glossary](glossary.md) for details about terminology.

## What do I need?

The below is a quick overview of the RustCrypto organization's projects based on the use case:

| Action                                              | Relevant Section                                            | Properties  | Note                                                                                                                                                                 |
| --------------------------------------------------- | ----------------------------------------------------------- | ----------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Verify that data is correct                         | [Data Hashing](hashing-data/index.html)                     | **I**       | Hash algorithms like SHA-256 provide a one-way map of data to a fixed-length that can be easily compared                                                             |
| Authenticate users with a password                  | [Password Hashing](hashing-password/index.html)             | **I**       | Password hashing algorithms are designed to be slow to prevent brute force attacks. Password hashes can be stored in a database.                                     |
| Prove that public data originally came from you     | [Message Authentication](message-authentication/index.html) | **I, A**    | MACs use a hash algorithm like SHA-256 with a secret to create a signature for data. With the secret key, you can prove the data is correct.                         |
| Prove that public data originally came from someone | [Digital Signatures](signing/index.html)                    | **I, A**    | Digital signatures provide a way to verify data came from a specific source using their public key. Roughly a reverse asymmetric encryption.                         |
| Hide data with a shared secret                      | [Symmetric Encryption](encryption-symmetric/index.html)     | **I, A, C** | When the encryptor and decryptor can both use the same key, symmetric encryption is used. AES and ChaCha are examples.                                               |
| Hide data without a shared secret                   | [Asymmetric Encryption](encryption-asymmetric/index.html)   | **I, A, C** | When the encryptor and decryptor do not share a common key, asymmetric encryption can be used to exchange data (such as a new common key). RSA and ECDH are examples |

[rustcrypto]: https://github.com/RustCrypto/
