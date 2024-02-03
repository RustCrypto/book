# Glossary

## Hash Function

A \[hash function\] takes in arbitrary data and produces a fixed-size key. Generally, a small change
in the input data produces a large change in the output hash, allowing for easy identification of
corrupted or modified data.

Good hash functions are resistant to searching; that is given data and a known hash, it should be
very difficult to find different input data that produces the same hash ("collisions").

## Cryptographic Hash Function <!--sort:ignore-->

[Cryptographic hash functions] are hash functions that resistant enough to searching that it is
reasonably unfeasible to locate collisions.

[cryptographic hash functions]: https://en.wikipedia.org/wiki/Cryptographic_hash_function
