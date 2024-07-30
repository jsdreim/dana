/// Asserts that one [`Quantity`](crate::Quantity) is *almost* equal to another,
///     by comparing their difference (in the units of the first quantity) to a
///     limit (default `1e-12`).
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


/// Equivalent to [`assert_qty_approx`], but only runs in debug builds.
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
