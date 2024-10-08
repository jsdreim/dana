//! Module for macro definitions.
//!
//! It should never be necessary to import anything from this module. It is only
//!     public for technical reasons; Macros are exported at the crate level and
//!     re-exported in the [prelude].

#![doc(hidden)]

#[allow(unused_imports)]
use crate::{*, prelude::*};

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
///     = UnitAnon::from_unit(Power::KiloWatt * Time::Hour);
/// ```
///
/// ## Macro Form 1
///
/// The first form of `dim!` allows defining a `Dimension` in terms of integer
///     literals, which alone already has a significant impact on readability:
/// ```
/// # use dana::prelude::*;
/// #
/// # let unit_anon = UnitAnon::from_unit(Power::KiloWatt * Time::Hour);
/// #
/// let _: UnitAnon<dim!(<2, 1, -2, 0, 0, 0, 0>)> = unit_anon;
/// ```
///
/// This form also assumes any unspecified dimensions to be zero, allowing most
///     definitions to be quite short:
/// ```
/// # use dana::prelude::*;
/// #
/// # let unit_anon = UnitAnon::from_unit(Power::KiloWatt * Time::Hour);
/// #
/// let _: UnitAnon<dim!(<2, 1, -2>)> = unit_anon;
/// ```
///
/// ## Macro Form 2
///
/// The second form mirrors the mathematical style of definition, as the product
///     of a sequence of dimensions with optional exponents:
/// ```
/// # use dana::prelude::*;
/// #
/// # let unit_anon = UnitAnon::from_unit(Power::KiloWatt * Time::Hour);
/// #
/// use dana::symbols::dimensions::*;
///
/// let _: UnitAnon<dim!(_L^2 * _M * _T^-2)> = unit_anon;
/// ```
///
/// Note that the symbols for [`Dimension`]s are prefixed with underscores. This
///     is to distinguish them from symbols for [`Unit`] types. Mathematically,
///     these are essentially the same thing, but within this library, they are
///     quite different.
///
/// ## Combined Form
///
/// It is also possible to combine the two forms:
/// ```
/// # use dana::{prelude::*, symbols::*};
/// #
/// # let unit_anon = UnitAnon::from_unit(Power::KiloWatt * Time::Hour);
/// #
/// let _: UnitAnon<dim!(<2,1,-3> * _T)> = unit_anon;
/// ```
///
/// The first form must be placed before the second form, and the two must be
///     separated by a `*` operator.
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
/// use dana::{qty, symbols::basic::*};
///
/// let gravity = qty![9.81 m/s^2];
/// ```
///
/// Using braces, it is also possible to supply an expression for the value:
/// ```
/// # use dana::{qty, symbols::basic::*};
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
/// # use dana::{qty, symbols::{electrical::*, physics::*}};
/// #
/// let heat_energy = qty![73.0 W*s as E];
/// let speed_limit = qty![45.0 mph in m/s];
/// ```
///
/// Because these conversion operators evaluate to [`Quantity::convert`] and
///     [`Quantity::convert_to`] calls, they are still subject to dimensional
///     compatibility requirements:
/// ```compile_fail
/// # use dana::{qty, symbols::basic::*};
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
/// # use dana::{qty, symbols::physics::*};
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
/// # use dana::{qty, symbols::physics::*};
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
/// # use dana::{qty, symbols::basic::*};
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
/// # use dana::{qty, symbols::electrical::*};
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
/// use dana::{prelude::*, symbols::basic::*};
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
/// # use dana::{prelude::*, symbols::time::*};
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
///
/// **Note:** This macro expands to an *expression*. For a version that does the
///     same thing, but instead expands to a *type*, see [`utype!`].
///
/// This macro expects to be invoked with one or more [`Unit`] types, separated
///     by binary mathematical operators, and possibly grouped by parentheses.
///     Each binary operator represents a specific [`UnitCompound`] generic type
///     that will be applied to the two sides of the operator. The following
///     operators are supported:
/// - `unit^N`: [`UnitPow`]
/// - `1 / unit`: [`PerUnit`]
/// - `lhs / rhs`: [`UnitDiv`]
/// - `lhs * rhs`: [`UnitMul`]
///
/// # Examples
///
/// The most basic way to use this macro, while still benefitting from operator
///     evaluation, is with fully-qualified units:
/// ```
/// use dana::prelude::*;
///
/// let _: PerUnit<Time> = unit!(1 / Time::Second);
/// let _: UnitDiv<Length, Time> = unit!(Length::Meter / Time::Second);
/// let _: UnitMul<Force, Length> = unit!(Force::Newton * Length::Meter);
/// let _: UnitSquared<Length> = unit!(Length::Meter^2);
///
/// //  This gets quite verbose for large units.
/// let _: UnitDiv<GravParam, Mass>
///     = unit!(Length::Meter^3 / Time::Second^2 / Mass::KiloGram);
/// ```
///
/// It is highly recommended to import from [`symbols`] as needed:
/// ```
/// use dana::{prelude::*, symbols::physics::*};
///
/// let _: PerUnit<Time> = unit!(1/s);
/// let _: UnitDiv<Length, Time> = unit!(m/s);
/// let _: UnitMul<Length, Force> = unit!(m*N);
/// let _: UnitSquared<Length> = unit!(m^2);
///
/// let _: UnitDiv<GravParam, Mass> = unit!(m^3 / s^2 / kg);
/// ```
///
/// Note that `^` is the "strongest" operator, and will always be applied as
///     soon as possible. Other operators will be applied in order from left to
///     right. To change this order, parentheses can be used to form groups:
/// ```
/// # use dana::{prelude::*, symbols::common::*};
/// #
/// let _: UnitDiv<Length, UnitSquared<Time>> = unit!(m / s^2);
/// let _: UnitSquared<UnitDiv<Length, Time>> = unit!((m/s)^2);
///
/// let _: UnitDiv<UnitMul<Mass, Length>, Time> = unit!(kg * m / s);
/// let _: UnitMul<UnitDiv<Length, Time>, Mass> = unit!(m / s * kg);
/// let _: UnitDiv<Length, UnitMul<Time, Mass>> = unit!(m / (s*kg));
/// ```
#[macro_export]
macro_rules! unit {($($t:tt)*) => {$crate::macros::proc::unit!($($t)*)}}


/// Macro to simplify compound unit type definitions.
///
/// # Examples
///
/// Syntax is identical to [`unit!`], but this macro resolves to a type, rather
///     than an expression:
/// ```
/// use dana::{prelude::*, symbols::basic::*};
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
