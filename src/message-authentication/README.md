# Message Authentication Code (MAC)

[MACs] provide a way to sign data such that a signer (you) can verify:

1. The data originated from you (authenticity)
1. The data has not been tampered with (integrity)

Typically, a MAC takes some data and a secret key and produces a signature. The signature and data
are then transmitted together. Upon receiving the data+signature, the secret key can be used to
recalculate the signature and verify it matches what is provided.

Note that data is not obscured! HMACs provide authentication (proof of origin) but not obfuscation
(hiding the information).

Hash-based MACS (HMACs) are some of the more popular MAC algorithms. See more information at
[HMACs](hash-based.md)

[macs]: https://en.wikipedia.org/wiki/Message_authentication_code
