#![no_std]
#![feature(test)]

extern crate digest;
extern crate pruefung;

digest::bench!(pruefung::fletcher16::Fletcher16);
