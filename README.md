# `pruefung`

*Checksums in pure Rust.*

[![TravisCI](https://img.shields.io/travis/althonos/pruefung/master.svg?maxAge=600&style=flat-square)](https://travis-ci.org/althonos/pruefung/branches)
[![Source](https://img.shields.io/badge/source-GitHub-303030.svg?maxAge=86400&style=flat-square)](https://github.com/althonos/pruefung)
[![Crate](https://img.shields.io/crates/v/pruefung.svg?maxAge=86400&style=flat-square)](https://crates.io/crates/pruefung)
[![Documentation](https://img.shields.io/badge/docs-latest-4d76ae.svg?maxAge=86400&style=flat-square)](https://docs.rs/pruefung)
[![CargoMake](https://img.shields.io/badge/built%20with-cargo--make-yellow.svg?maxAge=86400&style=flat-square)](https://sagiegurari.github.io/cargo-make)
[![Changelog](https://img.shields.io/badge/keep%20a-changelog-8A0707.svg?maxAge=86400&style=flat-square)](http://keepachangelog.com/)
[![SayThanks](https://img.shields.io/badge/say-thanks!-1EAEDB.svg?maxAge=86400&style=flat-square)](https://saythanks.io/to/althonos)

###### THIS LIBRARY DOES NOT PROVIDE A STABLE API YET !

## Usage

Add this crate to the `Cargo.toml`:

```toml
[dependencies]
pruefung = "^0.1.0"
```

Check out the **Sums** section to see the minimal required version you need
depending on the algorithm you wish to use.

All the checksums are implemented using the same logic as the [hashes] crate of
the [RustCrypto] project, implementing the [`digest::Digest`] and the
[`core::hasher::Hasher`] traits.

Then, to compute a hash, for instance a `CRC32` (Ethernet standard):

```rust
extern crate pruefung;
use std::hash::Hasher;

let mut hasher = pruefung::crc32::CRC32();   // Instantiate a hasher
let data = b"Hello, world !";

hasher.write(data);                          // Feed the hasher
hasher.write("String data".as_bytes());      // (possibly multiple times)

let hash = hasher.finish();                  // Consume the hasher
println!("Result: {:x}", hash)               // print the result as native hex
```


## Sums

Latest version of the crate implements the following checksums:

Algorithm                                                         | *since* | `struct`
----------------------------------------------------------------- | ------- | --------
[Adler32](https://en.wikipedia.org/wiki/Adler-32)                 | `0.1.0` | `::adler32::Adler32`
[BSD checksum](https://en.wikipedia.org/wiki/BSD_checksum)        | `0.1.0` | `::bsd::BSD`
[CRC32](https://en.wikipedia.org/wiki/Cyclic_redundancy_check)    | `0.1.0` | `::crc32::CRC32`
[CRC32C](https://en.wikipedia.org/wiki/Cyclic_redundancy_check)   | `0.1.0` | `::crc32c::CRC32C`
[Fletcher16](https://en.wikipedia.org/wiki/Fletcher%27s_checksum) | `0.1.0` | `::fletcher16::Fletcher16`
[SysV checksum](https://en.wikipedia.org/wiki/SYSV_checksum)      | `0.1.0` | `::sysv::SysV`

These checksums are **NOT** cryptographically secure. They should not be used
for something else than data validation against *accidental* modifications:
an attacker could easily *forge* a file to pass any of these checksums ! For
secure checksums, look at the [hashes] implemented by the [RustCrypto] team.


## Why `pruefung` ?

*I was in Switzerland when I started this project. Yet, I don't really speak
german. But a slug version of `zyklische-redundanzprüfung` seemed like a nice
name, instead of another checksum, cksum, checksums, crc, etc. crate.*


[hashes]: https://github.com/RustCrypto/hashes
[RustCrypto]: https://github.com/RustCrypto
[`digest::Digest`]: https://docs.rs/digest/0.6.1/digest/trait.Digest.html
[`core::hasher::Hasher`]: https://doc.rust-lang.org/core/hash/trait.Hasher.html
