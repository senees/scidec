# Number converter from scientific notation

The library converts the input text containing a number in scientific notation, e.g:

```
1234.5678e-2
```

to number, represented by the following tuple:

```
(sign, mantissa-hi-64-bits, mantissa-lo-64-bits, exponent)
```

Example:

```Rust
use scidec::{Number, number_from_string};

let result = number_from_string("1234.5678e-2");
match result {
  Number::Fin(false, 0, 12345678, -6) => {}
  _ => panic!()
}
```