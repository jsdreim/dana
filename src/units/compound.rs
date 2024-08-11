//! Module for [compound unit](UnitCompound) types.
//!
//! Compound units are defined in terms of other units. Each compound unit type
//!     is generic over one or more [`Unit`] types (which may themselves also be
//!     compound units), and represents a relationship between them. A compound
//!     unit defines its [`Dim`](Unit::Dim) type and [scale](Unit::scale) based
//!     on those of its components.

use super::traits::*;

mod serde;

mod unit_div;
mod unit_inv;
mod unit_mul;
mod unit_pow;

pub use unit_div::UnitDiv;
pub use unit_inv::UnitInv;
pub use unit_mul::UnitMul;
pub use unit_pow::*;


/// Legacy alias for [`UnitInv`].
//  TODO: Remove for `0.6.0`.
#[deprecated(since = "0.5.0", note = "renamed to `UnitInv`")]
pub type PerUnit<U> = UnitInv<U>;


// /// Module to re-export all compound unit types.
// pub mod types {
//     pub use super::{UnitDiv, UnitInv, UnitMul, unit_pow::*};
// }


/// Operator implementations grouped together, to compare them more easily.
mod impl_ops {
    use core::ops::{Div, Mul};
    use num_traits::Inv;
    use typenum::Integer;
    use crate::{dimension::*, units::compound::*};

    //region `Div`/`Mul` impls.
    macro_rules! impl_div_mul {
        ($($name:ident $(<$(
        $param:ident $(: $bound:ident)?
        ),*>)?),* $(,)?) => {
            $(impl<$($($param $(: $bound)?,)*)? __W: Unit>
            Div<__W> for $name$(<$($param),*>)? where Self: CanUnitDiv<__W> {
                type Output = UnitDiv<Self, __W>;

                fn div(self, rhs: __W) -> Self::Output {
                    Self::Output::new(self, rhs)
                }
            })*

            $(impl<$($($param $(: $bound)?,)*)? __W: Unit>
            Mul<__W> for $name$(<$($param),*>)? where Self: CanUnitMul<__W> {
                type Output = UnitMul<Self, __W>;

                fn mul(self, rhs: __W) -> Self::Output {
                    Self::Output::new(self, rhs)
                }
            })*
        };
    }

    impl_div_mul!(
        UnitInv<U: Unit>,
        UnitDiv<A: Unit, B: Unit>,
        UnitMul<A: Unit, B: Unit>,
        UnitPow<U: Unit, E: Integer>,
    );
    //endregion

    //region `Inv` impls.
    macro_rules! impl_inv {
        ($($name:ident $(<$(
        $param:ident $(: $bound:ident)?
        ),*>)?),* $(,)?) => {
            $(impl$(<$($param $(: $bound)?),*>)?
            Inv for $name$(<$($param),*>)? where Self: CanUnitInv {
                type Output = UnitInv<Self>;

                fn inv(self) -> Self::Output {
                    Self::Output::new(self)
                }
            })*
        };
    }

    impl_inv!(
        UnitInv<U: Unit>,
        // UnitDiv<A: Unit, B: Unit>,
        UnitMul<A: Unit, B: Unit>,
        UnitPow<U: Unit, E: Integer>,
    );

    impl<A: Unit, B: Unit> Inv for UnitDiv<A, B> {
        type Output = UnitDiv<B, A>;

        fn inv(self) -> Self::Output {
            Self::Output::new(self.1, self.0)
        }
    }
    //endregion

    //region `CanPow` impls.
    macro_rules! impl_pow {
        ($($name:ident $(<$(
        $param:ident $(: $bound:ident)?
        ),*>)?),* $(,)?) => {
            $(impl<$($($param $(: $bound)?,)*)? const __E: i32>
            CanPow<__E> for $name$(<$($param),*>)? where
                Exponent<__E>: HasTypenum,
                Self: Unit,
                Self::Dim: CanDimPowType<<Exponent<__E> as HasTypenum>::Typenum>,
            {
                type Output = UnitPowN<Self, __E>;

                fn pow(self) -> Self::Output {
                    Self::Output::new(self)
                }
            })*
        };
    }

    impl_pow!(
        UnitInv<U: Unit>,
        UnitDiv<A: Unit, B: Unit>,
        UnitMul<A: Unit, B: Unit>,
        // UnitPow<U: Unit, E: Integer>,
    );
    //endregion
}
