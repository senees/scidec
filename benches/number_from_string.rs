#![feature(test)]

extern crate test;

use scidec::number_from_string;
use test::Bencher;

#[bench]
fn bench_number_from_string_0001(b: &mut Bencher) {
  b.iter(|| {
    let _ = number_from_string("12");
  });
}

#[bench]
fn bench_number_from_string_0002(b: &mut Bencher) {
  b.iter(|| {
    let _ = number_from_string("938475E-03");
  });
}

#[bench]
fn bench_number_from_string_0003(b: &mut Bencher) {
  b.iter(|| {
    let _ = number_from_string("0.00003E-02");
  });
}

#[bench]
fn bench_number_from_string_0004(b: &mut Bencher) {
  b.iter(|| {
    let _ = number_from_string("9999999999999999999999999999999999");
  });
}
