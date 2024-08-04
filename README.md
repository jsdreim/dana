# Dana

[![Crates.io](https://img.shields.io/crates/v/dana?logo=rust)](https://crates.io/crates/dana)
[![docs.rs](https://docs.rs/dana/badge.svg)](https://docs.rs/dana)

**Compile-time [dimensional analysis](https://en.wikipedia.org/wiki/Dimensional_analysis) via generic types.**

## Overview

Dimensional analysis is a method to ensure the correctness of calculations, by tracking the relationships between units.
This can also be useful in working out relationships without having to look them up.
For example, say you have a time *t* (`T`) and a speed *v* (`L/T`), and you want to find a distance *d* (`L`).
Simple algebra can tell you that `L` = `(L/T)` × `T`, and accordingly, *d* = *v* × *t*.

This library implements these checks using Rust's static type system.
As a result, any incompatibility between units becomes a **compile-time error**, ensuring that all code is dimensionally sound.

---

This function compiles successfully:

```rust
use dana::{Quantity, units::{Length, Speed, Time}};

fn speed(dist: Quantity<Length>, time: Quantity<Time>) -> Quantity<Speed> {
    dist / time
}
```

Whereas *this* function, because `Speed` is defined as `UnitDiv<Length, Time>`, will not compile, raising a `mismatched types` error because the expression is the wrong way around:

```rust
use dana::{Quantity, units::{Length, Speed, Time}};

fn speed(dist: Quantity<Length>, time: Quantity<Time>) -> Quantity<Speed> {
    time / dist
}
```

```
error[E0308]: mismatched types
 --> src/main.rs:4:5
  |
3 | fn speed(dist: Quantity<Length>, time: Quantity<Time>) -> Quantity<Speed> {
  |                                                           --------------- expected `Quantity<UnitDiv<Length, Time>>` because of return type
4 |     time / dist
  |     ^^^^^^^^^^^ expected `Quantity<UnitDiv<Length, Time>>`, found `Quantity<UnitDiv<Time, Length>>`
  |
  = note: expected struct `Quantity<UnitDiv<Length, Time>>`
             found struct `Quantity<UnitDiv<Time, Length>>`
```

See the [library documentation](https://docs.rs/dana) for further information.
