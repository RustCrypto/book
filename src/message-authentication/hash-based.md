# Hash-Based Message Authentication (HMAC)

HMACs are [MACs](index.html) based on a hash algorithm and are named after that algorithm; e.g.
`HMAC-SHA256`. Usually, a HMAC algorithm will concatenate the data and the secret then hash them
together to create the signature.

The [`hmac`] crate provides a wrapper type [`Hmac`] that implements the trait [`Mac`] for any type
that implements [`Digest`]: for example, `sha2::Sha256`. See
[Data Hashing](../hashing-data/index.html) for information on hash algorithms.

A good example of HMACs in use are with JWTs (JSON web tokens). Upon successful login, a server
generates an identifying payload such as `{"name": "ferris", "issued": "2023-10-02T21:00:00"}`, then
signs that payload with a secret. The payload and signature together are given to the client.

When the client needs to prove themselves, it presents this token (payload and signature) to the
server. The server uses its secret to recalculate the payload's signature; if it matches what is
presented, great! The server knows it authored the token, and can trust that the user _is_ `ferris`.
If the signatures do not match, somebody may have tampered with the data and it should be rejected.

See <https://jwt.io/> for a good demonstration JWTs using HMACs.

## What should I use?

When in doubt, `HMAC-SHA256` is the current industry standard.

```rust,editable
# fn main() {
use data_encoding::HEXLOWER;
use hmac::{Hmac, Mac};
use sha2::Sha256;

// Create alias for HMAC-SHA256
type HmacSha256 = Hmac<Sha256>;

// We use this secret to sign and later verify some data
let secret = b"server's secret key";

let data = "This user is authorized!";
let mut mac = HmacSha256::new_from_slice(secret).unwrap();
mac.update(data.as_bytes());

// Create a signature from the provided data and signature
// Be careful with calling `.into_bytes`; see the `hmac` crate for details
let signature = mac.finalize().into_bytes();

// Combine the data and signature
let message = format!("{data}${}", HEXLOWER.encode(&signature));

// The result looks like this. We now send this data *somewhere*
let expected =
    "This user is authorized!$4cd1ee787d0a32717765e0912cbad27fbbdd4d4540124f02405d3ce66731494f";
assert_eq!(message, expected);

// Later on, we receive this data+signature back from *somewhere*. Let's verify it
let (received_data, received_signature) = message.split_once('$').unwrap();
let mut mac = HmacSha256::new_from_slice(secret).unwrap();
mac.update(received_data.as_bytes());

// Success! The signature lines up, so we must have created this data
assert!(mac
    .verify_slice(&HEXLOWER.decode(received_signature.as_bytes()).unwrap())
    .is_ok());

// A client tries to upgrade themselves to admin by only changing the data
let bad_message =
    "This user is admin!$4cd1ee787d0a32717765e0912cbad27fbbdd4d4540124f02405d3ce66731494f";
let (bad_data, bad_signature) = bad_message.split_once('$').unwrap();
let mut mac = HmacSha256::new_from_slice(secret).unwrap();
mac.update(bad_data.as_bytes());

// Error! We may have an imposter
assert!(mac
    .verify_slice(&HEXLOWER.decode(bad_signature.as_bytes()).unwrap())
    .is_err());
# }
```

[`digest`]: https://docs.rs/digest/latest/digest/trait.Digest.html
[`hmac`]: https://docs.rs/hmac/latest/hmac/index.html
[`mac`]: https://docs.rs/digest/latest/digest/trait.Mac.html
