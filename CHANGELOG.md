# Changelog

---
## Upcoming

### Added
- Implemented `UnitRescale` type.
- Implemented `CanRoot<3>` for `Volume`. Output type is `UnitRescale<Length>`.
- Added `cc` as a symbol for cubic centimeters.
- Added `Quantity::cancel_exponent` method.
- Implemented `UnitMixed` trait.
- Implemented `UnitUnary` trait.
### Changed
- Reworked `serde` support for compound and anonymous unit types.
- Simplified trait bounds for unit operations.
### Fixed


---
## 0.3.1: 2024-08-05

### Fixed
- Corrected missed `std::` usage in `dim!` macro.


---
## 0.3.0: 2024-08-05

### Added
- Implemented unit tests and error type for `Quantity<Time>` conversions.
- Added `stable` and `unstable` as feature groups.
### Changed
- Enabled `no_std`.


---
## 0.2.0: 2024-08-04
- Published.
