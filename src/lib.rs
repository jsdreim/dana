//! Compile-time [dimensional analysis] via generic types.
//!
//! [dimensional analysis]: https://en.wikipedia.org/wiki/Dimensional_analysis
//!
//! The core of the crate is the [`Unit`] trait, which marks a struct or enum as
//!     being a representation of a dimensional unit. Units may have multiple
//!     variants, each with a different scaling factor.
//!
//! The most important exported type is [`Quantity`]. A `Quantity` is generic
//!     over `Unit` types and numeric [`Value`] types, and serves to pair a
//!     dimensionless value with a dimensional unit. The default scalar type is
//!     `f64`, but [`Value`] is automatically implemented for any type that
//!     implements the correct set of traits.
//!
//! # Examples
//!
//! The goal of dimensional analysis is to ensure correctness when doing
//!     calculations on physical quantities. Operations are only valid between
//!     quantities of compatible units, and units must be conserved in order to
//!     get correct results.
//!
//! The fundamental principle of this library is to represent quantities with
//!     incompatible units as different types, so that attempting to use them
//!     together results in a compiler error.
//!
//! The following function takes a distance an object has moved, as well as the
//!     amount of time it took to move that distance, and calculates the average
//!     speed of the object:
//! ```
//! use dana::{Quantity, units::*};
//!
//! fn speed(dist: Quantity<Length>, time: Quantity<Time>) -> Quantity<Speed> {
//!     dist / time
//! }
//! ```
//!
//! This calculation is correctly performed by dividing the distance moved by
//!     the time taken. Attempting to perform the wrong operation will produce
//!     the wrong type, resulting in a "mismatched types" error:
//! ```compile_fail
//! # use dana::{Quantity, units::*};
//! fn speed(dist: Quantity<Length>, time: Quantity<Time>) -> Quantity<Speed> {
//!     time / dist
//! }
//! ```
//!
//  New quantities.
//! ## Defining Quantities
//!
//! Using the full syntax is verbose to the point of near-unreadability:
//! ```
//! use dana::{Quantity, units::*};
//!
//! let grav: Quantity<UnitDiv<Length, UnitSquared<Time>>> = Quantity {
//!     unit: UnitDiv::new(Length::Meter, UnitSquared::new(Time::Second)),
//!     value: 9.81,
//! };
//! ```
//!
//! This can be reduced somewhat by using type inference, [`Quantity::new`] or
//!     [`Unit::quantity`], standard library math operators, and the methods of
//!     unit traits. The result is better, but still difficult to read for more
//!     complex expressions:
//! ```
//! use dana::{Quantity, units::{concrete::*, traits::CanSquare}};
//!
//! let grav = Quantity::new(
//!     Length::Meter / Time::Second.squared(),
//!     9.81,
//! );
//! ```
//!
//! To make large units more workable, the [`qty`] macro interprets combinations
//!     of units using a wider range of operators than the standard library
//!     traits provide:
//! ```
//! use dana::{qty, units::concrete::*};
//!
//! let grav = qty![9.81 Length::Meter / (Time::Second ^ 2)];
//! ```
//!
//! Finally, the [`symbols`] module provides standard SI unit symbols as
//!     constants and type aliases, bringing the syntax very close to a pure
//!     mathematical form:
//! ```
//! use dana::{qty, symbols::*};
//!
//! let grav = qty![9.81 m/s^2];
//! ```
//!
//!
//  Conversion.
//! ## Unit Conversion
//!
//! In addition to defining new [`Quantity`] values, the [`qty`] macro may also
//!     be used for conversion and reorganization. There are three operators
//!     that can be used for this purpose:
//! - `->`: Simple reorganization.
//! - `as`: Conversion to the base unit of a [`Unit`] type.
//! - `in`: Conversion to a specific unit.
//!
//! These operations can also be chained:
//! ```
//! use dana::{constants::CONST_C2, qty, symbols::*, units::*};
//!
//! let quantity = qty![
//!     1.0 g // One gram.
//!     * {CONST_C2} // Multiply by c².
//!     as Energy // Convert to energy according to E=mc².
//!     as P * T  // Express as the product of power and time.
//!     in W * 1/Hz // Reinterpret time as "per Hertz".
//!     -> P / f // Reorganize to be power divided by frequency.
//!     in A * V / Hz // Reinterpret power as the product of amps and volts.
//! ];
//! ```
//!
//! A star can be used to "dereference" a quantity, returning the scalar value,
//!     after performing any conversions. Among other things, this allows for
//!     particularly readable assertions:
//! ```
//! # use dana::{qty, symbols::*};
//! let d = qty![30.0 km];
//! let v = qty![45.0 km/h];
//!
//! assert_eq!(qty![*(d/v) -> T in min], 40.0);
//! ```
//!
//! Square brackets can be used to perform recursion, allowing for definition,
//!     calculation, and conversion all in a single call:
//! ```
//! # use dana::{qty, symbols::*};
//! assert_eq!(qty![*[3.3 V] / [150.0 Ω] in mA], 22.0);
//! ```

#![cfg_attr(feature = "simd", feature(portable_simd))]
#![warn(missing_docs)]

//  NOTE: Hack to allow proc macros to work both inside and outside the crate.
extern crate self as dana;

#[macro_use]
extern crate dana_macros;
pub use dana_macros::{dim, qty, qtype, unit, utype};

#[macro_use]
mod macros;

pub mod prelude;

pub mod constants;
#[allow(missing_docs)]
pub mod dimension;
pub mod equations;
pub mod quantity;
pub mod symbols;
pub mod units;
pub mod value;

#[cfg(feature = "simd")]
#[allow(missing_docs)]
pub mod simd;

pub use quantity::Quantity;
pub use units::Unit;
pub use value::Value;


#[cfg(test)]
mod tests {
    use num_traits::Inv;
    use crate::{Quantity, symbols::*, units::{*, traits::*}};

    #[test]
    pub fn test_macros() {
        //  Start with a basic length unit, and ensure `unit!` correctly
        //      produces one.
        let u: Length = m;
        assert_eq!(u.dimension(), u.inv().inv().dimension());

        //  Confirm that `utype!` produces types that agree.
        let _: utype!(L) = u;

        //  Check implicitly-positive exponents.
        let _: utype!(L^ 1) = u;
        let _: utype!(L^ 2) = u.squared();
        let _: utype!(L^ 3) = u.cubed();
        // let _: utype!(L^ 4) = u.pow(4.0);
        // let _: utype!(L^ 2.0) = u.pow(2.0);

        //  Check explicitly-positive exponents.
        let _: utype!(L^+1) = u;
        let _: utype!(L^+2) = u.squared();
        let _: utype!(L^+3) = u.cubed();
        // let _: utype!(L^+4) = u.pow(4.0);
        // let _: utype!(L^+2.0) = u.pow(2.0);

        //  Check explicitly-negative exponents.
        let _: utype!(L^-1) = u.inv();
        let _: utype!(L^-2) = u.squared().inv();
        let _: utype!(L^-3) = u.cubed().inv();
        // let _: utype!(L^-4) = u.pow(4.0).inv();
        // let _: utype!(L^-2.0) = u.pow(2.0).inv();


        //  Use that unit for a quantity, and ensure the `qty!` macro correctly
        //      produces one.
        let q: Quantity<Length> = qty!(2.0 u);
        assert_eq!(q, q.inv().inv());

        //  Confirm that `qtype!` produces types that agree.
        let _: qtype!(L) = q;

        //  Check implicitly-positive exponents.
        let _: qtype!(L^ 1) = q;
        let _: qtype!(L^ 2) = q.squared();
        let _: qtype!(L^ 3) = q.cubed();
        let _: qtype!(L^ 4) = q.squared().squared();
        // let _: qtype!(L^ 4) = q.pow(4.0);
        // let _: qtype!(L^ 2.0) = q.pow(2.0);

        //  Check explicitly-positive exponents.
        let _: qtype!(L^+1) = q;
        let _: qtype!(L^+2) = q.squared();
        let _: qtype!(L^+3) = q.cubed();
        let _: qtype!(L^+4) = q.squared().squared();
        // let _: qtype!(L^+4) = q.pow(4.0);
        // let _: qtype!(L^+2.0) = q.pow(2.0);

        //  Check explicitly-negative exponents.
        let _: qtype!(L^-1) = q.inv();
        let _: qtype!(L^-2) = q.squared().inv();
        let _: qtype!(L^-3) = q.cubed().inv();
        let _: qtype!(L^-4) = q.squared().squared().inv();
        // let _: qtype!(L^-4) = q.pow(4.0).inv();
        // let _: qtype!(L^-2.0) = q.pow(2.0).inv();

        //  Check powers and roots.
        assert_eq!(q.squared(), q.pow::<2>());
        assert_eq!(q.squared(), q.pow::<8>().root::<4>());
        assert_eq!(q.pow::<6>(), q.pow::<2>().pow::<3>());
    }

    #[test]
    fn test_quantity_norm() {
        fn test<U: UnitStep>(q1: Quantity<U>) {
            let q2 = q1.normalize();

            // eprintln!("{q1:>9e} -> {q2:>8.3}");
            assert_eq!(q2, q1, "Precision loss different from pure convert.");
            assert_qty_approx!(<= 1e-6, q1, q2);
        }

        test(qty![4.321_e+9  mm/s]);
        test(qty![4.321_e+8  mm/s]);
        test(qty![4.321_e+7  mm/s]);
        test(qty![4.321_e+6  mm/s]);
        // eprintln!();
        test(qty![4.321_e+5  mm/s]);
        test(qty![4.321_e+4  mm/s]);
        test(qty![4.321_e+3  mm/s]);
        // eprintln!();
        test(qty![4.321_e+2  mm/s]);
        test(qty![4.321_e+1  mm/s]);
        test(qty![4.321_e00  mm/s]);
        // eprintln!();
        test(qty![4.321_e-1  mm/s]);
        test(qty![4.321_e-2  mm/s]);
        test(qty![4.321_e-3  mm/s]);
        // eprintln!();
        test(qty![4.321_e-4  mm/s]);
        test(qty![4.321_e-5  mm/s]);
        test(qty![4.321_e-6  mm/s]);
        // eprintln!();
        test(qty![4.321_e-7  mm/s]);
        test(qty![4.321_e-8  mm/s]);
        test(qty![4.321_e-9  mm/s]);
        // eprintln!();
        test(qty![4.321_e-10 mm/s]);
        test(qty![4.321_e-11 mm/s]);
        test(qty![4.321_e-12 mm/s]);
    }

    #[test]
    fn test_unit_scale() {
        fn test<U: UnitStep + PartialOrd + std::fmt::Debug>() {
            let base = U::base();

            assert_eq!(
                1.0, base.scale(),
                "base unit `{base:?}` has the wrong scale",
            );

            assert!(base >= base.step_to_bottom());
            assert!(base <= base.step_to_top());
        }

        test::<Length>();
        test::<Mass>();
        test::<Time>();
        test::<Current>();
        test::<Temp>();
        test::<Amount>();
        test::<Intensity>();

        test::<Charge>();
        test::<Energy>();
        test::<Force>();
        test::<Frequency>();
        test::<One>();
        test::<Power>();
        test::<Pressure>();
        test::<Resistance>();
        test::<Voltage>();
    }

    #[test]
    fn test_scale() {
        let dist = Length::MilliMeter.quantity(50.0);

        let as_mm = dist.with_unit(Length::MilliMeter).value;
        let as_cm = dist.with_unit(Length::CentiMeter).value;

        assert_eq!(as_mm, as_cm * 10.0);
    }
}
