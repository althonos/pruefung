# `pruefung`

*Checksums in pure Rust.*

[![Travis master](https://img.shields.io/travis/althonos/pruefung/master.svg?maxAge=600&style=flat-square)](https://travis-ci.org/althonos/pruefung/branches)
[![Source](https://img.shields.io/badge/source-GitHub-303030.svg?maxAge=86400&style=flat-square)](https://github.com/althonos/pruefung)
[![Cargo make](https://img.shields.io/badge/built%20with-cargo--make-yellow.svg?maxAge=86400&style=flat-square)](https://sagiegurari.github.io/cargo-make)
[![Keep a changelog](https://img.shields.io/badge/keep%20a-changelog-8A0707.svg?maxAge=86400&style=flat-square)](http://keepachangelog.com/)
[![Say Thanks!](https://img.shields.io/badge/say-thanks!-1EAEDB.svg?maxAge=86400&style=flat-square)](https://saythanks.io/to/althonos)


## Usage

Add this crate to the `Cargo.toml`:

```toml
[dependencies]
pruefung = "^0.1.0"
```

Check out the **Sums** section to see the minimal required version you need
depending on the algorithm you wish to use.

All the checksums are implemented using the same logic
as the [hashes](https://github.com/RustCrypto/hashes)
crate of the [RustCrypto](https://github.com/RustCrypto)
project, implementing the [`digest::Digest`](https://docs.rs/digest/0.6.1/digest/trait.Digest.html) trait.

Then, to compute a hash, for instance a `CRC32` (Ethernet standard):

```rust
extern crate pruefung;
let mut hasher = pruefung::crc32::CRC32();   // Instantiate a hasher
let data = b"Hello, world !";

hasher.input(data);                          // Feed the hasher
hasher.input("String data".as_bytes());      // (possibly multiple times)

let hash = hasher.result();                  // Consume the hasher
println!("Result: {:x}", hash)               // print the result as native hex
```

## Sums

Latest version of the crate implements the following checksums:


Algorithm       | since version
--------------- | -------------
[Adler32]       | `0.1.0`
[BSD checksum]  | `0.1.0`
[CRC32]         | `0.1.0`
[CRC32C]        | `0.1.0`
[Fletcher16]    | `0.1.0`
[SysV checksum] | `0.1.0`

[Adler32]: https://en.wikipedia.org/wiki/Adler-32
[BSD checksum]: https://en.wikipedia.org/wiki/BSD_checksum
[CRC32]: https://en.wikipedia.org/wiki/Cyclic_redundancy_check
[CRC32C]: https://en.wikipedia.org/wiki/Cyclic_redundancy_check
[Fletcher16]: https://en.wikipedia.org/wiki/Fletcher%27s_checksum
[SysV checksum]: https://en.wikipedia.org/wiki/SYSV_checksum

These checksums are **NOT** cryptographically secure. They should not be used
for something else than data validation against *accidental* modifications:
an attacker could easily *forge* a file to pass any of these checksums ! For
secure checksums, look at the [hashes](https://github.com/RustCrypto/hashes)
implemented by the RustCrypto developers.

## Why `pruefung` ?

*I was in Switzerland when I started this project. Yet, I don't really speak
german. But a slug version of `zyklische-redundanzprüfung` seemed like a nice
name, instead of another checksum, cksum, checksums, etc, crc crate.*
