#![no_std]
#![feature(test)]

extern crate digest;
extern crate pruefung;

digest::bench!(pruefung::fnv::Fnv64);
