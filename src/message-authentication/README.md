# Hash-Based Message Authentication (HMAC)

[HMACs] provide a way to sign data such that a signer (you) can verify:

1. The data has not been tampered with
1. The data was indeed sent by you

A HMAC takes in some data and a secret key, and produces a signature. The signature and data are
then transmitted together. Upon receiving the data+signature, the secret key can be used to
recalculate the signature and verify it matches what is provided.

HMACs are based on a hash algorithm and are named after that algorithm; e.g. `HMAC-SHA256`. The
[`hmac`] crate provides a wrapper type [`Hmac`] that implements the trait \[`Mac`\] for any type
that implements [`Digest`]: such as `sha2::Sha256`. See [Data Hashing](hashing-data/README.md) for
information on hash algorithms.

Note that data is not obscured! HMACs provide authentication (proof of origin) but not obfuscation
(hiding the information).

A good example of HMACs in use are with JWTs (JSON web tokens). Data that is roughly
`{"username": "ferris", "issued": "2023-10-02T21:00:00"}` is signed with the server's secret, then
given to a client upon successful login. The client can then provide this key back to the server to
"prove" who they are: the server verifies that it did indeed create the data by verifying the
signature with its secret.

See <https://jwt.io/> for a good demonstration JWTs using HMACs.

## What should I use?

When in doubt, `HMAC-SHA256` is the current industry standard.

```rust,editable
# fn main() {
use sha2::Sha256;
use hmac::{Hmac, Mac};
use data_encoding::HEXLOWER;

// Create alias for HMAC-SHA256
type HmacSha256 = Hmac<Sha256>;

let secret = b"server's secret key";
let data = "This user is authorized!";
let mut mac = HmacSha256::new_from_slice(secret).unwrap();
mac.update(data.as_bytes());

// Be careful with calling `.into_bytes`; see the `hmac` crate for details
let signature = mac.finalize().into_bytes();

// This gets sent somewhere
let message = format!("{data}${}", HEXLOWER.encode(&signature));

let expected = "This user is authorized!$4cd1ee787d0a32717765e0912cbad27fbbdd4d4540124f02405d3ce66731494f";
assert_eq!(message, expected);

// We receive this from a client
let (received_data, received_signature) = message.split_once('$').unwrap();
let mut mac = HmacSha256::new_from_slice(secret).unwrap();
mac.update(received_data.as_bytes());
mac.verify_slice(&HEXLOWER.decode(received_signature.as_bytes()).unwrap()).unwrap();

// A client tries to upgrade themselves to admin
let bad_message = "This user is admin!$4cd1ee787d0a32717765e0912cbad27fbbdd4d4540124f02405d3ce66731494f";
let (bad_data, bad_signature) = bad_message.split_once('$').unwrap();
let mut mac = HmacSha256::new_from_slice(secret).unwrap();
mac.update(bad_data.as_bytes());
mac.verify_slice(&HEXLOWER.decode(bad_signature.as_bytes()).unwrap()).unwrap_err();
# }
```

[hmacs]: https://en.wikipedia.org/wiki/HMAC
[`digest`]: https://docs.rs/digest/latest/digest/trait.Digest.html
[`hmac`]: https://docs.rs/hmac/latest/hmac/index.html
