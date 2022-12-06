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

//! Set of tested input strings.

pub const IN: [&str; 78] = [
  /*   0 */ "",
  /*   1 */ "12",
  /*   2 */ ".12",
  /*   3 */ "000.0",
  /*   4 */ "0.0",
  /*   5 */ "0.",
  /*   6 */ "1.0",
  /*   7 */ "1.",
  /*   8 */ "1.0e2",
  /*   9 */ "1.00e2",
  /*  10 */ "1.00e354",
  "1.000000000000000000000000000000000000000000e2",
  "100",
  "1e5",
  "0.0000000000000000000000000000000000000000000000000000000000000001001",
  "1234.5678e-2",
  "12e321",
  "938475E-03",
  "+12",
  "-12",
  "000001",
  "+000001",
  "-000001",
  "0.3",
  "0.3E2",
  "0.3e2",
  "0.3E02",
  "0.3E+02",
  "0.3E-02",
  "0.00003E-02",
  "9999999999999999999999999999999999",
  "inf",
  "+inf",
  "-inf",
  "infinity",
  "+infinity",
  "-infinity",
  "-inFINity",
  "NaN",
  "nan",
  "NAN",
  "+NaN",
  "-NaN",
  "SNaN",
  "+SNaN",
  "-SNaN",
  "-SNAN",
  "qNAN",
  "+A132",
  "1.1P2",
  "1..",
  ".",
  "1.123P12E",
  "0.1E",
  "0.1EP",
  "0.1E0P",
  "0.1E123P",
  "IE0",
  "InE0",
  "Infinety",
  "Infinizy",
  "Infinitz",
  "Infizity",
  "Infa",
  "nana",
  "nun",
  "snana",
  "infinitya",
  "sun",
  "123p4",
  "340282366920938463463374607431768211455",
  "-340282366920938463463374607431768211455",
  "+340282366920938463463374607431768211455E+2147483647",
  "-340282366920938463463374607431768211455E-2147483647",
  "99999999999999999999999999999999999999999999999999999999999999999",
  "-99999999999999999999999999999999999999999999999999999999999999999",
  "1E+2147483648",
  "-1E-2147483648",
];
