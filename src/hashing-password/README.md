# Password Hashing

RustCrypto provides a number of [password hash functions], which takes arbitrary data (a password)
and produces a hash with parameters, which is typically stored in a database. When an unknown
password is received, it is hashed with the same function and compared with the stored value.

Technically, password hash functions are "key derivation functions" which take a secret key (the
real password) and turn it into another secret key (the hash stored in the database1).

RustCrypto password hashes implement the [`PasswordHasher`] and [`PasswordVerifier`] traits, which
provide the hash and check interfaces. These come from the [`password-hash`] create, and are always
reexported in the hash algorithm crate.

## What should I use?

OWASP puts out a [Password Storage Cheat Sheet], which at time of writing lists the following order
of preferences:

1. Argon2id with minimum 19 MiB memory, 2 iterations, and 1 degree of parallelism
1. Scrypt with a memory cost parameter of 2<sup>17</sup>, block size of 8, and 1 degree of
   parallelism
1. Bcrypt with a minimum work factor of 10 and a password limit of 72 bytes
1. If FIPS-140 compliance is required, PBKDF2 with a minimum work factor of 600,000 and a SHA-256
   internal hash function

You don't need to worry about parameters; the default implementations match the reccomendations.

```rust,noplaypen
# fn main() {
use argon2::{Argon2, PasswordHasher, PasswordVerifier, password_hash::Salt};

let password = "password";
// b64 hash of "bad salt!". Don't do this; it should be random
let salt_str = "YmFkIHNhbHQh";

let argon2 = Argon2::default();
let salt: Salt = salt_str.try_into().unwrap();
let hash = argon2.hash_password(password.as_bytes(), salt).unwrap();

// This is the hash we will store. Notice our salt string is included, as well as parameters:
// version 0x13 (19), memory 19456KiB (19 MiB), 2 iterations (time), parallelism 1
let expected =
    "$argon2id$v=19$m=19456,t=2,p=1$YmFkIHNhbHQh$DqHGwv6NQV0VcaJi7jeF1E8IpfMXmXcpq4r2kKyqpXk";

assert_eq!(expected, hash.to_string());

// The verifier reads the salt and the parameters from the hash and verifies the result is equal
Argon2::default().verify_password(password.as_bytes(), &hash).expect("invalid password");
# }
```

Sometimes it may be necessary to be able to verify passwords that were hashed with different
algorithms or parameters. This can be done using the traits in `password_hash`:

```rust,noplaypen
# fn main() {
use password_hash::{PasswordHash, PasswordVerifier};

use argon2::Argon2;
use pbkdf2::Pbkdf2;
use scrypt::Scrypt;

// Can be: `$argon2`, `$pbkdf2`, or `$scrypt`
let hash_string =
    "$argon2id$v=19$m=19456,t=2,p=1$YmFkIHNhbHQh$DqHGwv6NQV0VcaJi7jeF1E8IpfMXmXcpq4r2kKyqpXk";
let input_password = "password";

let password_hash = PasswordHash::new(&hash_string).expect("invalid password hash");

// Trait objects for algorithms to support
let algs: &[&dyn PasswordVerifier] = &[&Argon2::default(), &Pbkdf2, &Scrypt];

password_hash.verify_password(algs, input_password).expect("invalid password");
# }
```

## Differences with Standard Hash Functions

Compared with [standard hash functions](hashing-data/README.md), password hashes have a few features
that make them well suited to this purpose:

### Salt

Imagine that we use SHA-256 to hash our password:

```rust,editable
# fn main() {
# use sha2::{Sha256, Digest};
# use data_encoding::HEXLOWER;
let input = "password";

let mut hash = Sha256::new();
hash.update(input.as_bytes());
let result = hash.finalize();

println!("Hash: {}", HEXLOWER.encode(&result));
# }
```

This prints `5e884898da...`, which we would store in our database.

Now some security event happens, and our database with all password hashes gets leaked. Now, the
public has this information:

```
+----------+------------+
| username | pw_sha256  |
+----------+------------+
| ferris   | 5e884898da | <- note this hash
| curie    | 437dd76609 |
| turing   | fc3a03a63b |
+----------+------------+
```

An attacker is ready, and has calculated a table of common password hashes:

```
+----------+------------+
| input    | sha_256    |
+----------+------------+
| 1234     | 03ac674216 |
| abcd     | 84e73dc50f |
| password | 5e884898da | <- note this hash
+----------+------------+
```

Bingo! user `ferris` has password hash `5e884898da`, which means the password `password` will work
to access ferris's account. The user is now compromised!

This is a simple version of a rainbow attack. To prevent this, we do what is called "salting". This
means that we pick a random salt that we store with the password in the database, and hash that salt
with the input password when we want to check it:

```
+----------+------------------------+
| username | pw                     |
+----------+------------------------+
| ferris   | $sha2$salty$d878e396b9 | # user's password is 'password'
| curie    | $sha2$spice$036ecfef69 |
| turing   | $sha2$spice$701f72c284 |
| lovelace | $sha2$sugar$a92ec3b280 | # user's password is also 'password'
+----------+------------------------+
```

Notice how no hashes from the attacker's list appear in our new password table, and that even though
two users have the same password, their hashes are different. This means that an attacker can no
longer look up a hash directly, and instead has to waste time calculating each possible input!

A salt function with SHA-256 would roughly look like:

```rust,editable
# fn main() {
# use sha2::{Sha256, Digest};
# use data_encoding::HEXLOWER;
// Input from the user
let input = "password";

// That user's salt, stored in the database
let salt = "salty";
let to_hash = format!("{salt}{input}");

// Compute salt+input hash
let mut hash = Sha256::new();
hash.update(to_hash.as_bytes());
let result = hash.finalize();
let digest = HEXLOWER.encode(&result);

println!("Hash: {digest}");

// Hash of salt+password in the database
let stored_value = "d878e396b9";
assert_eq!(stored_value, &digest[..10]);
# }
```

To be clear: you should _never_ use SHA-256 as a password hashing algorithm, it is just very simple
to use in this example. See the [What should I use?] section at the top of this page for better
suggestions.

### Slowness

While hashes like SHA-256 are meant to be fast, password hashes are meant to be slow. This is to
help slow down brute forcing, where an attacker tries hashing a wide variety of common passwords
with a known salt to try to compute a known hash.

As mentioned in [What should I use?], password hashing algorithms have parameters that can control
how slow or fast they are to solve. The defaults recommended by OWASP provide a good balance of high
security without being nonperformant.

### A Note on Pepper

Peppering is the process of encrypting hashed passwords with a secret key before storing in the
database. For example:

```
storage:

      per-row random salt -\
                            \
    plaintext password -> argon2 -> aes256-gcm encrypt -> database entry
                                     /
      per-database secret key ------/

verification:

      per-database secret key --\           
                                 \         /-- hash -------------------------------\
    database entry -> aes256-gem decrypt -|               argon2 -> input hash -> compare
                                           \              /  /
                                             \-- salt --/   /
                                                           /
      input password -------------------------------------/

```

OWSAP now suggests this process since it means that even if an attacker has access to the database
table, they cannot do anything with it unless they also have the encryption key (stored separately).

Peppering is not considered part of password hashing functions.

[password hash functions]: https://en.wikipedia.org/wiki/Key_derivation_function#Password_hashing
[password storage cheat sheet]: https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html
[what should i use?]: #what-should-i-use
[`password-hash`]: https://docs.rs/password-hash/latest/password_hash/
[`passwordhasher`]: https://docs.rs/password-hash/latest/password_hash/trait.PasswordHasher.html
[`passwordverifier`]: https://docs.rs/password-hash/latest/password_hash/trait.PasswordVerifier.html
