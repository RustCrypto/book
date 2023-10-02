# Data Hashing

RustCrypto provides a number of [hash functions], which take arbitrary data and create a
fixed-length hash, which can then be directly compared to verify the original data. Common
algorithms include `SHA-256`, `SHA-1`, and `MD5`.

See the [repository] for a list of available hashes.

RustCrypto hashes implement the [`Digest`] trait, which provides the hashing interface. This comes
from the [`digest`](https://docs.rs/digest/latest/digest/) crate, and is always reexported in the
hash algorithm crates.

## What should I use?

When in doubt, `SHA-256` (provided in the [`sha2`] crate) is the current industry standard.

```rust,editable
# fn main() {
use sha2::{Sha256, Digest};
use data_encoding::HEXLOWER;

// Two slightly different strings
let data1 = "Hello, world!";
let data2 = "Hello, world";

// Compute the hash of the first string
let mut hash1 = Sha256::new();
hash1.update(data1.as_bytes());
let result1 = hash1.finalize();
println!("Hash 1: {}", HEXLOWER.encode(&result1));

// Compute the hash of the second string
let mut hash2 = Sha256::new();
hash2.update(data2.as_bytes());
let result2 = hash2.finalize();
println!("Hash 2: {}", HEXLOWER.encode(&result2));

// The data is very similar, but our hashes are different!
assert_ne!(result1, result2);
# }
```

[hash functions]: https://en.wikipedia.org/wiki/Hash_function
[repository]: https://github.com/RustCrypto/hashes/tree/master
[`digest`]: https://docs.rs/digest/latest/digest/trait.Digest.html
[`sha2`]: https://docs.rs/sha2/latest/sha2/
