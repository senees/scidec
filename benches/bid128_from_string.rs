/*
 * MIT License
 *
 * Copyright (c) 2022 Dariusz Depta
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

//! Benchmarks.

#![feature(test)]

extern crate test;

use scidec::bid128_from_string;
use test::Bencher;

const INPUTS: [&str; 6] = [
  "12",
  "938475E-03",
  "0.00003E-02",
  "9999999999999999999999999999999999",
  "+9999999999999999999999999999999999e+6111",
  "+1000000000000000000000000000000000e-6176",
];

#[bench]
fn bench_bid128_from_string_00(b: &mut Bencher) {
  b.iter(|| {
    let _ = bid128_from_string(INPUTS[0]);
  });
}

#[bench]
fn bench_bid128_from_string_01(b: &mut Bencher) {
  b.iter(|| {
    let _ = bid128_from_string(INPUTS[1]);
  });
}

#[bench]
fn bench_bid128_from_string_02(b: &mut Bencher) {
  b.iter(|| {
    let _ = bid128_from_string(INPUTS[2]);
  });
}

#[bench]
fn bench_bid128_from_string_03(b: &mut Bencher) {
  b.iter(|| {
    let _ = bid128_from_string(INPUTS[3]);
  });
}

#[bench]
fn bench_bid128_from_string_04(b: &mut Bencher) {
  b.iter(|| {
    let _ = bid128_from_string(INPUTS[4]);
  });
}

#[bench]
fn bench_bid128_from_string_05(b: &mut Bencher) {
  b.iter(|| {
    let _ = bid128_from_string(INPUTS[5]);
  });
}
