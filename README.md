# `pruefung`

*Checksums in pure Rust.*

[![TravisCI](https://img.shields.io/travis/althonos/pruefung/master.svg?maxAge=600&style=flat-square)](https://travis-ci.org/althonos/pruefung/branches)
[![Codecov](https://img.shields.io/codecov/c/github/althonos/pruefung.svg?maxAge=600&style=flat-square)](https://codecov.io/github/althonos/pruefung)
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

All the checksums are implemented using the same logic as the [hashes][1] crate
of the [RustCrypto][2] project, implementing the [`digest::Digest`][3], and the
[`core::hasher::Hasher`][4] traits when possible (less than 64 bits in the
output).

Then, to compute a hash, for instance a [`CRC32`][5] (Ethernet standard):

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


## Dependencies

The crate itself is [`no_std`][6], but provides [`digest::Digest`][3] implementations
for convenience and integration with the [`hashes`][1] crate. Those bindings can
be scrapped off however by disabling the default features of the crates, adding
the following line to yout `Cargo.toml`:

```toml
[dependencies.pruefung]
version = "^0.1.0"
default-features = false
```


## Sums

Latest version of the crate implements the following checksums:

Algorithm           | *since* | *implemented as*
------------------- | ------- | --------
[Adler32][7]        | `0.1.0` | [`::adler32::Adler32`][15]
[BSD checksum][8]   | `0.2.0` | [`::bsd::Bsd`][16]
[CRC8][5]           | `0.3.0` | [`::crc::crc8::Crc8`][28]
[CRC16][5]          | `0.3.0` | [`::crc::crc8::Crc16`][29]
[CRC32][5]          | `0.2.0` | [`::crc::crc32::Crc32`][17]
[CRC32C][5]         | `0.2.0` | [`::crc::crc32::Crc32c`][18]
[CRC64][5]          | `0.3.0` | [`::crc::crc64::Crc64`][30]
[Fletcher16][9]     | `0.1.0` | [`::fletcher16::Fletcher16`][19]
[FNV0-32][10]       | `0.2.0` | [`::fnv::fnv32::Fnv32z`][20]
[FNV1-32][11]       | `0.2.0` | [`::fnv::fnv32::Fnv32`][21]
[FNV1a-32][12]      | `0.2.0` | [`::fnv::fnv32::Fnv32a`][22]
[FNV0-64][10]       | `0.2.0` | [`::fnv::fnv64::Fnv64z`][23]
[FNV1-64][11]       | `0.2.0` | [`::fnv::fnv64::Fnv64`][24]
[FNV1a-64][12]      | `0.2.0` | [`::fnv::fnv64::Fnv64a`][25]
[SysV checksum][13] | `0.1.0` | [`::sysv::SysV`][26]
[UNIX checksum][14] | `0.2.0` | [`::unix::Unix`][27]


These checksums are **NOT** cryptographically secure. They should not be used
for something else than data validation against *accidental* modifications:
an attacker could easily *forge* a file to pass any of these checksums ! For
secure checksums, look at the [hashes][1] implemented by the [RustCrypto][2]
team.


## Credits

* [Martin Larralde](https://github.com/althonos)
* [Ferdia McKeogh](https://github.com/chocol4te)


## Why `pruefung` ?

*I was in Switzerland when I started this project. Yet, I don't really speak
german. But a slug version of `zyklische-redundanzprüfung` seemed like a nice
name, instead of another checksum, cksum, checksums, crc, etc. crate.*

<!-- General links -->
[1]: https://github.com/RustCrypto/hashes
[2]: https://github.com/RustCrypto
[3]: https://docs.rs/digest/*/digest/trait.Digest.html
[4]: https://doc.rust-lang.org/core/hash/trait.Hasher.html
[5]: https://en.wikipedia.org/wiki/Cyclic_redundancy_check
[6]: https://doc.rust-lang.org/1.11.0/book/no-stdlib.html

<!-- Wikipedia checksum articles -->
[7]: https://en.wikipedia.org/wiki/Adler-32
[8]: https://en.wikipedia.org/wiki/BSD_checksum
[9]: https://en.wikipedia.org/wiki/Fletcher%27s_checksum
[10]: https://en.wikipedia.org/wiki/Fowler-Noll-Vo_hash_function#FNV-0_hash_.28deprecated.29
[11]: https://en.wikipedia.org/wiki/Fowler-Noll-Vo_hash_function#FNV-1_hash
[12]: https://en.wikipedia.org/wiki/Fowler-Noll-Vo_hash_function#FNV-1a_hash
[13]: https://en.wikipedia.org/wiki/SYSV_checksum
[14]: https://en.wikipedia.org/wiki/Cksum

<!-- API documentation -->
[15]: https://docs.rs/pruefung/*/pruefung/adler32/struct.Adler32.html
[16]: https://docs.rs/pruefung/*/pruefung/bsd/struct.Bsd.html
[17]: https://docs.rs/pruefung/*/pruefung/crc/crc32/struct.Crc32.html
[18]: https://docs.rs/pruefung/*/pruefung/crc/crc32/struct.Crc32c.html
[19]: https://docs.rs/pruefung/*/pruefung/fletcher16/struct.Fletcher16.html
[20]: https://docs.rs/pruefung/*/pruefung/fnv/fnv32/struct.Fnv32z.html
[21]: https://docs.rs/pruefung/*/pruefung/fnv/fnv32/struct.Fnv32.html
[22]: https://docs.rs/pruefung/*/pruefung/fnv/fnv32/struct.Fnv32a.html
[23]: https://docs.rs/pruefung/*/pruefung/fnv/fnv64/struct.Fnv64z.html
[24]: https://docs.rs/pruefung/*/pruefung/fnv/fnv64/struct.Fnv64.html
[25]: https://docs.rs/pruefung/*/pruefung/fnv/fnv64/struct.Fnv64a.html
[26]: https://docs.rs/pruefung/*/pruefung/sysv/struct.SysV.html
[27]: https://docs.rs/pruefung/*/pruefung/unix/struct.Unix.html
[28]: https://docs.rs/pruefung/*/pruefung/crc/crc8/struct.Crc8.html
[29]: https://docs.rs/pruefung/*/pruefung/crc/crc16/struct.Crc16.html
[30]: https://docs.rs/pruefung/*/pruefung/crc/crc64/struct.Crc64.html
