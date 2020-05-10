# to-str

![](https://github.com/DoumanAsh/to-str/workflows/Rust/badge.svg)
[![Crates.io](https://img.shields.io/crates/v/to-str.svg)](https://crates.io/crates/to-str)
[![Documentation](https://docs.rs/to-str/badge.svg)](https://docs.rs/crate/to-str/)

Generic trait for efficient conversion to string.
Suitable for `no_std`

## Performance

It is up to `ToStr` implementation, but in general default impls are efficient alternative to Rust's core `fmt`

### Integers

Inspired by C++ [fmt](https://github.com/fmtlib/fmt) and [itoa](https://github.com/dtolnay/itoa)

Performance in general is close to `itoa`, but lags a bit behind due to some missing optimization opportunities (possibly due to slices in `ToStr`).

Differently from `itoa` though the implementation avoids generics per integer type, but rather uses common functions for `u8`, `u64` and `u128` types
