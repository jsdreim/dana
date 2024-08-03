//! Module for macro definitions.
//!
//! It should never be necessary to import anything from this module. It is only
//!     public for technical reasons; Macros are exported at the crate level and
//!     re-exported in the [prelude].

#[allow(unused_imports)]
use crate::*;

pub use crate::{dim, qty, qtype, unit, utype};


#[doc(hidden)]
pub mod proc {
    pub use dana_macros::{dim, qty, qtype, unit, utype};
}


/// Macro to simplify [`Dimension`] definitions.
///
/// [`Dimension`]: dimension::Dimension
///
/// # Examples
///
/// A `Dimension` is effectively a type-level array of seven [`Integer`] types,
///     each corresponding to, and representing an exponent of, one of the [ISQ]
///     base dimensions. It is somewhat analogous to `[i32; 7]`.
///
/// [`Integer`]: typenum::Integer
/// [ISQ]: https://en.wikipedia.org/wiki/International_System_of_Quantities
///
/// Because `Dimension` is generic over seven [`typenum`] types, fully-explicit
///     definitions are quite opaque and hard to read without much practice,
///     not to mention extremely long:
/// ```
/// use dana::{dimension::Dimension, prelude::*};
/// use typenum::{N2, P1, P2, Z0};
///
/// let unit_anon: UnitAnon<Dimension<P2, P1, N2, Z0, Z0, Z0, Z0>>
///     = (Power::KiloWatt * Time::Hour).anonymous();
/// ```
///
/// ## Macro Form 1
///
/// The first form of `dim!` allows defining a `Dimension` in terms of integer
///     literals, which alone already has a significant impact on readability:
/// ```
/// # use dana::prelude::*;
/// #
/// # let unit_anon = (Power::KiloWatt * Time::Hour).anonymous();
/// #
/// let _: UnitAnon<dim!(<2, 1, -2, 0, 0, 0, 0>)> = unit_anon;
/// ```
///
/// This form also assumes any unspecified dimensions to be zero, allowing most
///     definitions to be quite short:
/// ```
/// # use dana::prelude::*;
/// #
/// # let unit_anon = (Power::KiloWatt * Time::Hour).anonymous();
/// #
/// let _: UnitAnon<dim!(<2, 1, -2>)> = unit_anon;
/// ```
///
/// ## Macro Form 2
///
/// The second form mirrors the mathematical style of definition, as the product
///     of a sequence of powers of unique symbols:
/// ```
/// # use dana::prelude::*;
/// #
/// # let unit_anon = (Power::KiloWatt * Time::Hour).anonymous();
/// #
/// let _: UnitAnon<dim!(L^2 * M * T^-2)> = unit_anon;
/// ```
///
/// Because there is no way to express complicated relationships between
///     `Dimension`s, it is not possible for the macro to combine arbitrary
///     dimensions by name, as it is for units with [`unit!`]. This macro is,
///     therefore, strictly limited to the following symbols:
/// - `L`:      [Length](dimension::Length)
/// - `M`:      [Mass](dimension::Mass)
/// - `T`:      [Time](dimension::Time)
/// - `I`:      [Electrical Current](dimension::Current)
/// - `Θ`, `K`: [Temperature](dimension::Temp)
/// - `N`:      [Substance Amount](dimension::Amount)
/// - `J`:      [Luminous Intensity](dimension::Intensity)
//  TODO: It may actually be possible, using op trait `Output` types in place of
//      compound unit struct types. Investigate.
#[macro_export]
macro_rules! dim {($($t:tt)*) => {$crate::macros::proc::dim!($($t)*)}}


/// Macro to simplify [`Quantity`] definitions.
///
/// Note: This macro is designed to be used in conjunction with imports from the
///     [`symbols`] module.
///
/// # Examples
/// ## Quantity Definitions
///
/// To define a new quantity, the macro first expects a numeric literal, to be
///     used as the dimensionless value. After finding a value, it then expects
///     a unit specifier, in the same format expected by [`unit!`].
///
/// This allows definition of quantities in very terse mathematical form,
///     especially with imports from [`symbols`]:
/// ```
/// use dana::{qty, symbols::*};
///
/// let gravity = qty![9.81 m/s^2];
/// ```
///
/// Using braces, it is also possible to supply an expression for the value:
/// ```
/// # use dana::{qty, symbols::*};
/// #
/// let meters = 5.0;
/// let distance = qty![{meters} m];
/// ```
///
/// ## Unit Conversions
///
/// In addition to defining new [`Quantity`] values, `qty!` may also be used for
///     unit conversion. There are two operators that can be used for this
///     purpose:
/// - `as`: Conversion to the base unit of a [`Unit`] type.
/// - `in`: Conversion to a specific unit.
///
/// ```
/// # use dana::{qty, symbols::*};
/// #
/// let heat_energy = qty![73.0 W*s as E];
/// let speed_limit = qty![45.0 mph in m/s];
/// ```
///
/// Because these conversion operators evaluate to [`Quantity::convert`] and
///     [`Quantity::convert_to`] calls, they are still subject to dimensional
///     compatibility requirements:
/// ```compile_fail
/// # use dana::{qty, symbols::*};
/// #
/// let speed_limit = qty![5.0 m*s in m/s];
/// //                ^^^^^^^^^^^^^^^^^^^^ error[E0271]: type mismatch
/// ```
///
/// ## Quantity Operations
///
/// Mathematical operations are also supported between existing quantities, in
///     order to make subsequent conversions clearer:
/// ```
/// # use dana::{qty, symbols::*};
/// #
/// let pressure = qty![12.0 psi];
/// let area = qty![16.0 cm^2];
/// let mass = qty![2.0 kg];
/// let time = qty![4.0 s];
///
/// let velocity_1 = qty![pressure * area / mass * time in m/s];
///
/// //  Parentheses are also available.
/// let velocity_2 = qty![(((pressure * area) / mass) * time) in m/s];
/// ```
///
/// It is also possible to mix definitions and operations, although preexisting
///     quantities should be enclosed in braces, to prevent them from being
///     interpreted as part of a unit specifier:
/// ```
/// # use dana::{qty, symbols::*};
/// #
/// let area = qty![16.0 cm^2];
/// let time = qty![4.0 s];
/// let n_kg = 2.0;
///
/// let velocity = qty![
///     12.0 psi
///     * {area}
///     / {n_kg} kg
///     * {time}
///     in m/s
/// ];
/// ```
///
/// A star can be placed at the front of the macro to "dereference" a quantity,
///     returning the dimensionless value, after completion of all operations.
///     Among other things, this allows for particularly readable assertions:
/// ```
/// # use dana::{qty, symbols::*};
/// #
/// let d = qty![30.0 km];
/// let v = qty![45.0 kph];
///
/// //  At 45 kph, travelled 30 km after 40 minutes.
/// assert_eq!(qty![*(d/v) in min], 40.0);
/// ```
///
/// ## Recursion
///
/// Square brackets can be used to perform recursion, allowing for definition,
///     calculation, conversion, and value output, all in a single invocation:
/// ```
/// # use dana::{qty, symbols::*};
/// #
/// assert_eq!(qty![*[3.3 V] / [150.0 Ω] in mA], 22.0);
/// ```
#[macro_export]
macro_rules! qty {($($t:tt)*) => {$crate::macros::proc::qty!($($t)*)}}


/// Macro to simplify [`Quantity`] type definitions.
///
/// # Examples
///
/// Basic usage is effectively a passthrough to [`utype!`], with `Quantity`
///     around it:
/// ```
/// use dana::{prelude::*, symbols::*};
///
/// let qty = qty![9.81 m/s^2];
///
/// //  Fully-explicit type:
/// let explicit: Quantity<UnitDiv<Length, UnitSquared<Time>>> = qty;
///
/// //  Type with `Quantity` and `utype!` macro:
/// let macro_utype: Quantity<utype!(L/T^2)> = qty;
///
/// //  Type with `qtype!` macro:
/// let macro_qtype: qtype!(L/T^2) = qty;
/// ```
///
/// The only variation is to specify a type for the dimensionless value. This
///     can be expressed in a few ways:
/// - Before the unit specifier, followed by a semicolon, similar to the `[T;N]`
///     syntax of array types
/// - Before the unit specifier, followed by `as`, closer to a natural-language
///     expression
/// - After the unit specifier, separated by a comma, mirroring the generic
///     parameters of `Quantity<U, V>`
/// ```
/// # use dana::{prelude::*, symbols::*};
/// #
/// let _: qtype!(Time) = qty![500.0 ms]; // Default (f64).
///
/// let _: qtype!(_; Time) = qty![500 ms]; // Implicit (i32).
/// let _: qtype!(u64; Time) = qty![500 ms]; // Explicit (u64).
///
/// let _: qtype!(_ as Time) = qty![500 ms]; // Implicit (i32).
/// let _: qtype!(u64 as Time) = qty![500 ms]; // Explicit (u64).
///
/// let _: qtype!(Time, _) = qty![500 ms]; // Implicit (i32).
/// let _: qtype!(Time, u64) = qty![500 ms]; // Explicit (u64).
/// ```
//  TODO: Decide which "before" form is better.
#[macro_export]
macro_rules! qtype {($($t:tt)*) => {$crate::macros::proc::qtype!($($t)*)}}


/// Macro to simplify compound unit definitions.
//  TODO
#[macro_export]
macro_rules! unit {($($t:tt)*) => {$crate::macros::proc::unit!($($t)*)}}


/// Macro to simplify compound unit type definitions.
///
/// # Examples
///
/// Syntax is identical to [`unit!`], but this macro resolves to a type, rather
///     than an expression:
/// ```
/// use dana::{prelude::*, symbols::*};
///
/// type UnitAccel = utype!(L/T^2);
///
/// let unit_accel: UnitAccel = unit!(m/s^2);
/// let qty_accel: Quantity<UnitAccel> = unit_accel.quantity(9.81);
/// ```
#[macro_export]
macro_rules! utype {($($t:tt)*) => {$crate::macros::proc::utype!($($t)*)}}


/// Asserts that one [`Quantity`] is *almost* equal to another, by comparing
///     their difference (in the units of the first quantity) to a limit
///     (default `1e-12`).
///
/// The limit can be changed by passing `<=` and a literal ahead of the first
///     argument.
///
/// On panic, this macro will print the values of the quantities.
///
/// Like [`assert_eq!`], this macro has another form, where a custom panic
///     message can be provided.
///
/// # Examples
///
/// ```
/// # use dana::{*, symbols::{length_si::*, volume_si::*}};
/// let a = qty![1.0 cm^3];
/// let b = qty![1.0 mL];
///
/// //  Default limit:
/// assert_qty_approx!(a, b);
/// assert_qty_approx!(a, b, "we are testing proximity of {} to {}", a, b);
///
/// //  Custom limit:
/// assert_qty_approx!(<= 3e-16, a, b);
/// assert_qty_approx!(<= 3e-16, a, b, "testing {} and {} more tightly", a, b);
/// ```
#[macro_export]
macro_rules! assert_qty_approx {
    ($lhs:expr, $rhs:expr $(, $($t:tt)*)?) => {
        $crate::assert_qty_approx!(<= 1e-12, $lhs, $rhs $(, $($t)*)?);
    };
    (<= $limit:expr, $lhs:expr, $rhs:expr $(,)?) => {
        if !(($lhs - $rhs).abs().value <= $limit) {
            panic!(
                "assertion `left ~= right` failed\
                \n  left: {lhs}\
                \n right: {rhs_conv} (from {rhs})\
                \n delta: {delta}\
                \n limit: {limit}",
                lhs = $lhs,
                rhs = $rhs,
                rhs_conv = $rhs.convert_to($lhs.unit),
                delta = ($lhs - $rhs).abs().value,
                limit = $limit,
            );
        }
    };
    (<= $limit:expr, $lhs:expr, $rhs:expr, $($t:tt)+) => {
        if !(($lhs - $rhs).abs().value <= $limit) {
            panic!(
                "assertion `left ~= right` failed: {message}\
                \n  left: {lhs}\
                \n right: {rhs_conv} ({rhs})\
                \n delta: {delta}\
                \n limit: {limit}",
                lhs = $lhs,
                rhs = $rhs,
                rhs_conv = $rhs.convert_to($lhs.unit),
                delta = ($lhs - $rhs).abs().value,
                limit = $limit,
                message = format_args!($($t)+),
            );
        }
    };
}


/// Equivalent to [`assert_qty_approx!`], but only runs in debug builds.
#[macro_export]
macro_rules! debug_assert_qty_approx {
    ($($t:tt)*) => {
        if cfg!(debug_assertions) {
            assert_qty_approx!($($t)*);
        }
    };
}


macro_rules! dummy {
    ($(#[$attr:meta])* $vis:vis trait $name:ident: $($traits:tt)*) => {
        $(#[$attr])*
        $vis trait $name: $($traits)* {}
        impl<T: $($traits)*> $name for T {}
    };
}
