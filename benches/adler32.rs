#![no_std]
#![feature(test)]

extern crate digest;
extern crate pruefung;

digest::bench!(pruefung::adler32::Adler32);
