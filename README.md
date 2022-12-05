# Scientific to decimal number format converter

This library parses the input text containing a number in scientific format like:

```
1234.5678e-2
```

and converts to a number represented by the following tuple:

```
(sign, mantissa-hi-64-bits, mantissa-lo-64-bits, exponent)
```

for example:

```Rust
use scidec::{Number, number_from_string};

let result = number_from_string("1234.5678e-2");
match result {
  Number::Fin(false, 0, 12345678, -6) => {}
  _ => panic!()
}
```