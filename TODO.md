# Todo
Vague outline of future plans.

## Major
- [ ] Ensure support for `Vector2`, `Vector3`, etc, either via another crate or by building them.

## Minor
- [ ] Ensure serde support is suitable for more than just JSON.
- [ ] Determine whether `<=` or `<` is more correct in `Quantity::normalize()`.
- [ ] Decide what to do about Greek letters in symbols.
- [ ] Decide what to do about `V` as symbol for voltage vs volume.


## Schedule
### 0.4.1

### 0.5.0

### 0.6.0
- [ ] Remove `PerUnit` alias.

### 1.0.0
- [ ] Eliminate all fallible instances of `.unwrap()`.
- [ ] Eliminate all instances of `#[allow(missing_docs)]`.
- [ ] Support arbitrary roots in some way, either via a `UnitRoot` compound or fractional `UnitPow`.
