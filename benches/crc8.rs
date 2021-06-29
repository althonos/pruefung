#![no_std]
#![feature(test)]

extern crate digest;
extern crate pruefung;

digest::bench!(pruefung::crc::crc8::Crc8);
