//! Module for base dimensions underlying the unit system.

use core::{marker::PhantomData, ops::{Add, Div, Mul, Neg, Sub}};
use num_traits::Inv;
use typenum::{
    marker_traits::{Integer, NonZero},
    PartialDiv,
};


dummy!(
    /// Trait bound for [`Dimension`] type parameters.
    pub trait Int: Integer
);


/// Integer type used for dimension exponents.
pub type Exp = i32;

/// Number of fundamental quantities.
pub const LEN: usize = 7;


/// Scalar "dimension", representing no dimension.
pub type One          = dim!(< 0, 0, 0, 0, 0, 0, 0>);
//                             L  M  T  I  Θ  N  J
pub type Length       = dim!(< 1, 0, 0, 0, 0, 0, 0>);
pub type Mass         = dim!(< 0, 1, 0, 0, 0, 0, 0>);
pub type Time         = dim!(< 0, 0, 1, 0, 0, 0, 0>);
pub type Current      = dim!(< 0, 0, 0, 1, 0, 0, 0>);
pub type Temp         = dim!(< 0, 0, 0, 0, 1, 0, 0>);
pub type Amount       = dim!(< 0, 0, 0, 0, 0, 1, 0>);
pub type Intensity    = dim!(< 0, 0, 0, 0, 0, 0, 1>);
//                             L  M  T  I  Θ  N  J
pub type Frequency    = dim!(< 0, 0,-1, 0, 0, 0, 0>);
pub type Velocity     = dim!(< 1, 0,-1, 0, 0, 0, 0>);
pub type Accel        = dim!(< 1, 0,-2, 0, 0, 0, 0>);
pub type Force        = dim!(< 1, 1,-2, 0, 0, 0, 0>);
pub type Pressure     = dim!(<-1, 1,-2, 0, 0, 0, 0>);
pub type Area         = dim!(< 2, 0, 0, 0, 0, 0, 0>);
pub type Volume       = dim!(< 3, 0, 0, 0, 0, 0, 0>);
pub type Density      = dim!(<-3, 1, 0, 0, 0, 0, 0>);
//                             L  M  T  I  Θ  N  J
pub type Charge       = dim!(< 0, 0, 1, 1, 0, 0, 0>);
pub type Torque       = dim!(< 2, 1,-2, 0, 0, 0, 0>);
pub type Energy       = dim!(< 2, 1,-2, 0, 0, 0, 0>);
pub type Power        = dim!(< 2, 1,-3, 0, 0, 0, 0>);
pub type Voltage      = dim!(< 2, 1,-3,-1, 0, 0, 0>);
pub type Resistance   = dim!(< 2, 1,-3,-2, 0, 0, 0>);
pub type Capacitance  = dim!(<-2,-1, 4, 2, 0, 0, 0>);
//                             L  M  T  I  Θ  N  J


/// Zero-size type that serves as a type-level array of exponents.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Dimension<L: Int, M: Int, T: Int, I: Int, K: Int, N: Int, J: Int> {
    _l: PhantomData<L>, _m: PhantomData<M>, _t: PhantomData<T>,
    _i: PhantomData<I>, _k: PhantomData<K>, _n: PhantomData<N>,
    _j: PhantomData<J>,
}

impl<L: Int, M: Int, T: Int, I: Int, K: Int, N: Int, J: Int>
Dimension<L, M, T, I, K, N, J> {
    pub const fn new() -> Self { Self {
        _l: PhantomData, _m: PhantomData, _t: PhantomData,
        _i: PhantomData, _k: PhantomData, _n: PhantomData,
        _j: PhantomData,
    }}
}


use private::Sealed;
mod private {
    pub trait Sealed {}
}


impl<L: Int, M: Int, T: Int, I: Int, Θ: Int, N: Int, J: Int>
Sealed for Dimension<L, M, T, I, Θ, N, J> {}


/// Trait specifying a type to be a [`Dimension`] with arbitrary exponents.
pub trait DimType: Sealed + Copy + core::fmt::Display {
    //region Definitions.
    /// Exponent typenum for Length.
    type ExpLen: Int;
    /// Exponent typenum for Mass.
    type ExpMass: Int;
    /// Exponent typenum for Time.
    type ExpTime: Int;
    /// Exponent typenum for Electrical Current.
    type ExpCurr: Int;
    /// Exponent typenum for Temperature.
    type ExpTemp: Int;
    /// Exponent typenum for Substance Amount.
    type ExpAmt: Int;
    /// Exponent typenum for Luminous Intensity.
    type ExpLum: Int;

    /// Exponent constant for Length.
    const EXP_LEN:  Exp = <Self::ExpLen as Integer>::I32;
    /// Exponent constant for Mass.
    const EXP_MASS: Exp = <Self::ExpMass as Integer>::I32;
    /// Exponent constant for Time.
    const EXP_TIME: Exp = <Self::ExpTime as Integer>::I32;
    /// Exponent constant for Electrical Current.
    const EXP_CURR: Exp = <Self::ExpCurr as Integer>::I32;
    /// Exponent constant for Temperature.
    const EXP_TEMP: Exp = <Self::ExpTemp as Integer>::I32;
    /// Exponent constant for Substance Amount.
    const EXP_AMT:  Exp = <Self::ExpAmt as Integer>::I32;
    /// Exponent constant for Luminous Intensity.
    const EXP_LUM:  Exp = <Self::ExpLum as Integer>::I32;
    //endregion

    //region Arrays.
    /// Exponents of the seven fundamental quantities.
    const ARRAY: [Exp; LEN] = [
        Self::EXP_LEN,
        Self::EXP_MASS,
        Self::EXP_TIME,
        Self::EXP_CURR,
        Self::EXP_TEMP,
        Self::EXP_AMT,
        Self::EXP_LUM,
    ];

    /// Labels of the seven fundamental quantities, paired with their exponents.
    const CHARS: [(char, Exp); LEN] = [
        ('L', Self::EXP_LEN),
        ('M', Self::EXP_MASS),
        ('T', Self::EXP_TIME),
        ('I', Self::EXP_CURR),
        ('Θ', Self::EXP_TEMP),
        ('N', Self::EXP_AMT),
        ('J', Self::EXP_LUM),
    ];
    //endregion

    /// Return a runtime representation of this dimension.
    fn dimension() -> Self;
}

impl<L: Int, M: Int, T: Int, I: Int, K: Int, N: Int, J: Int> DimType
for Dimension<L, M, T, I, K, N, J> {
    type ExpLen = L;
    type ExpMass = M;
    type ExpTime = T;
    type ExpCurr = I;
    type ExpTemp = K;
    type ExpAmt = N;
    type ExpLum = J;

    fn dimension() -> Self { Self::new() }
}


impl<L: Int, M: Int, T: Int, I: Int, K: Int, N: Int, J: Int> core::fmt::Display
for Dimension<L, M, T, I, K, N, J> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        use core::fmt::Write;

        let mut any = false;

        for (char, exp) in Self::CHARS {
            if exp != 0 {
                if any {
                    f.write_char('*')?;
                } else {
                    any = true;
                }

                f.write_char(char)?;

                if exp != 1 {
                    write!(f, "^{}", exp)?;
                }
            }
        }

        Ok(())
    }
}


/// Division.
impl<
    L1: Int, M1: Int, T1: Int, I1: Int, K1: Int, N1: Int, J1: Int,
    L2: Int, M2: Int, T2: Int, I2: Int, K2: Int, N2: Int, J2: Int,
> Div<Dimension<L2, M2, T2, I2, K2, N2, J2>>
for Dimension<L1, M1, T1, I1, K1, N1, J1> where
    L1: Sub<L2>, L1::Output: Int,
    M1: Sub<M2>, M1::Output: Int,
    T1: Sub<T2>, T1::Output: Int,
    I1: Sub<I2>, I1::Output: Int,
    K1: Sub<K2>, K1::Output: Int,
    N1: Sub<N2>, N1::Output: Int,
    J1: Sub<J2>, J1::Output: Int,
{
    type Output = Dimension<
        L1::Output, M1::Output, T1::Output,
        I1::Output, K1::Output, N1::Output,
        J1::Output,
    >;

    fn div(self, _: Dimension<L2, M2, T2, I2, K2, N2, J2>) -> Self::Output {
        Default::default()
    }
}


/// Multiplication.
impl<
    L1: Int, M1: Int, T1: Int, I1: Int, K1: Int, N1: Int, J1: Int,
    L2: Int, M2: Int, T2: Int, I2: Int, K2: Int, N2: Int, J2: Int,
> Mul<Dimension<L2, M2, T2, I2, K2, N2, J2>>
for Dimension<L1, M1, T1, I1, K1, N1, J1> where
    L1: Add<L2>, L1::Output: Int,
    M1: Add<M2>, M1::Output: Int,
    T1: Add<T2>, T1::Output: Int,
    I1: Add<I2>, I1::Output: Int,
    K1: Add<K2>, K1::Output: Int,
    N1: Add<N2>, N1::Output: Int,
    J1: Add<J2>, J1::Output: Int,
{
    type Output = Dimension<
        L1::Output, M1::Output, T1::Output,
        I1::Output, K1::Output, N1::Output,
        J1::Output,
    >;

    fn mul(self, _: Dimension<L2, M2, T2, I2, K2, N2, J2>) -> Self::Output {
        Default::default()
    }
}


/// Inversion.
impl<
    L: Int + Neg, M: Int + Neg, T: Int + Neg,
    I: Int + Neg, K: Int + Neg, N: Int + Neg,
    J: Int + Neg,
> Inv for Dimension<L, M, T, I, K, N, J> where
    L::Output: Int, M::Output: Int, T::Output: Int,
    I::Output: Int, K::Output: Int, N::Output: Int,
    J::Output: Int,
{
    type Output = Dimension<
        L::Output, M::Output, T::Output,
        I::Output, K::Output, N::Output,
        J::Output,
    >;

    fn inv(self) -> Self::Output { Default::default() }
}


/// Indicates that a [`Dimension`] may be raised to an [`Integer`] power.
pub trait DimPowType<E: Int>: DimType {
    /// The output of the operation.
    type Output: DimType;
}

impl<
    L: Int + Mul<E>, M: Int + Mul<E>, T: Int + Mul<E>,
    I: Int + Mul<E>, K: Int + Mul<E>, N: Int + Mul<E>,
    J: Int + Mul<E>,
    E: Int,
> DimPowType<E> for Dimension<L, M, T, I, K, N, J> where
    L::Output: Int, M::Output: Int, T::Output: Int,
    I::Output: Int, K::Output: Int, N::Output: Int,
    J::Output: Int,
{
    type Output = Dimension<
        L::Output, M::Output, T::Output,
        I::Output, K::Output, N::Output,
        J::Output,
    >;
}


/// Indicates that a [`Dimension`] may be taken to a [`NonZero`] [`Integer`] root.
pub trait DimRootType<D: Int + NonZero>: DimType {
    /// The output of the operation.
    type Output: DimType;
}

impl<
    L: Int + PartialDiv<D>, M: Int + PartialDiv<D>, T: Int + PartialDiv<D>,
    I: Int + PartialDiv<D>, K: Int + PartialDiv<D>, N: Int + PartialDiv<D>,
    J: Int + PartialDiv<D>,
    D: Int + NonZero,
> DimRootType<D> for Dimension<L, M, T, I, K, N, J> where
    L::Output: Int, M::Output: Int, T::Output: Int,
    I::Output: Int, K::Output: Int, N::Output: Int,
    J::Output: Int,
{
    type Output = Dimension<
        L::Output, M::Output, T::Output,
        I::Output, K::Output, N::Output,
        J::Output,
    >;
}


/// Zero-size generic representing a specific `i32` at the type level.
pub struct ExpHack<const E: Exp>;

/// Trait for associating a type with a specific [`typenum`] [`Integer`].
pub trait HasTypenum {
    /// The [`Integer`] equivalent of this type.
    type Typenum: Integer;
}

dana_macros::impl_typenums!();


/// Indicates that a [`Dimension`] may be raised to an arbitrary `i32` power.
pub trait DimPow<const E: Exp>: DimType {
    /// The output of the operation.
    type Output: DimType;
}

impl<Dim: DimType, const E: Exp> DimPow<E> for Dim where
    ExpHack<E>: HasTypenum,
    Dim: DimPowType<<ExpHack<E> as HasTypenum>::Typenum>,
{
    type Output = Dim::Output;
}


/// Indicates that a [`Dimension`] may be taken to an arbitrary `i32` root.
pub trait DimRoot<const D: Exp>: DimType {
    /// The output of the operation.
    type Output: DimType;
}

impl<Dim: DimType, const D: Exp> DimRoot<D> for Dim where
    ExpHack<D>: HasTypenum,
    <ExpHack<D> as HasTypenum>::Typenum: NonZero,
    Dim: DimRootType<<ExpHack<D> as HasTypenum>::Typenum>,
{
    type Output = Dim::Output;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dimensions() {
        //  TODO: Find a way to test this without needing stdlib.
        // #[cfg(feature = "std")] {
        //     assert_eq!(format!("{}", Length::dimension()), "L");
        //     assert_eq!(format!("{}", Velocity::dimension()), "L*T^-1");
        //     assert_eq!(format!("{}", Accel::dimension()), "L*T^-2");
        // }

        let _: Accel = Velocity::dimension() / Time::dimension();
        let _: Accel = Velocity::dimension() * Time::dimension().inv();
        let _: Length = Velocity::dimension() * Time::dimension();
        let _: Torque = Length::dimension() * Force::dimension();
    }
}
