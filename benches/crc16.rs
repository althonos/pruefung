#![no_std]
#![feature(test)]
#[macro_use]
extern crate crypto_tests;
extern crate pruefung;

bench_digest!(pruefung::crc::crc16::Crc16);
