#![feature(test)]

extern crate c_utf8;
extern crate test;
use c_utf8::CUtf8;
use test::{black_box, Bencher};

#[bench]
fn from_bytes(b: &mut Bencher) {
    let s = "abcdéfghîjklmnöpqrstúvwxÿz\0";
    b.iter(|| {
        let s = black_box(s.as_bytes());
        black_box(CUtf8::from_bytes(s).unwrap());
    });
}