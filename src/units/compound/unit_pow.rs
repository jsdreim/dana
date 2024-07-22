use std::{marker::PhantomData, ops::Mul};
use typenum::Integer;
use crate::{/*scalar::Scalar,*/ units::{dim::*, traits::*}};

pub type UnitPow2<U> = UnitPow<U, typenum::P2>;
pub type UnitPow3<U> = UnitPow<U, typenum::P3>;
pub type UnitPow4<U> = UnitPow<U, typenum::P4>;
pub type UnitPow5<U> = UnitPow<U, typenum::P5>;

pub type UnitSquared<U> = UnitPow2<U>;
pub type UnitCubed<U> = UnitPow3<U>;


/// A unit raised to an arbitrary power.
#[derive(Clone, Copy, /*Debug,*/ Default, //Deserialize, Serialize,
Eq, Ord, PartialOrd)]
pub struct UnitPow<U: Unit, E: Integer>(pub U, pub PhantomData<E>) where
    U::Dim: DimPowType<E>,
;

impl<U: Unit, E: Integer> UnitPow<U, E> where
    U::Dim: DimPowType<E>,
{
    pub const fn new(unit: U) -> Self { Self(unit, PhantomData) }
}

impl<U: Unit, E: Integer> PartialEq for UnitPow<U, E> where
    U::Dim: DimPowType<E>,
{
    fn eq(&self, other: &Self) -> bool {
        self.scale().eq(&other.scale())
    }
}

impl<U: Unit, E: Integer> Unit for UnitPow<U, E> where
    U::Dim: DimPowType<E>,
{
    type Dim = <U::Dim as DimPowType<E>>::Output;
    // type ScaleType = f64;

    fn scale(&self) -> f64 {
        self.0.scale().powi(E::I32)
    }
}

impl<U: Unit + std::fmt::Debug, E: Integer> std::fmt::Debug for UnitPow<U, E> where
    U::Dim: DimPowType<E>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "UnitPow({:?}, {:?})", self.0, E::I32)
    }
}

impl<U: Unit, E: Integer> std::fmt::Display for UnitPow<U, E> where
    U::Dim: DimPowType<E>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#}^{}", self.0, E::I32)
    }
}

// impl<U1: ConvertInto<U2>, U2: Unit, E: Exp> ConvertFrom<UnitPow<U1, E>> for UnitPow<U2, E> {}


//region Exponential traits.
impl<U: Unit, E: Integer> UnitExp for UnitPow<U, E> where
    U::Dim: DimPowType<E>,
{}

impl<U: Unit, E1, E2> CanPow<E2> for UnitPow<U, E1> where
    E1: Integer + Mul<E2>,
    <E1 as Mul<E2>>::Output: Integer,
    U::Dim: DimPowType<E1>,
    U::Dim: DimPowType<<E1 as Mul<E2>>::Output>,
{
    type Output = UnitPow<U, E1::Output>;
    fn pow(self) -> Self::Output { UnitPow::new(self.0) }
}

//region Roots.
// impl<U: Unit, E: Integer> CanRoot<E> for UnitPow<U, E> {
//     type Output = U;
//     fn root(self) -> Self::Output { self.0 }
// }

//region Nearly-manual root implementations.
// macro_rules! impl_roots {
//     ($($e_num:ident / $e_den:ident = $e_quo:ident;)*) => {
//         $(
//         impl<U: Unit> CanRoot<$e_den> for UnitPow<U, $e_num> {
//             type Output = UnitPow<U, $e_quo>;
//             fn root(self) -> Self::Output { UnitPow::new(self.0) }
//         }
//         )*
//     };
// }
//
// impl_roots! {
//     E4 / E2 = E2;
//     E6 / E2 = E3;
//     E8 / E2 = E4;
//     E10 / E2 = E5;
//     E12 / E2 = E6;
//     E14 / E2 = E7;
//     E16 / E2 = E8;
//
//     E6 / E3 = E2;
//     E9 / E3 = E3;
//     E12 / E3 = E4;
//     E15 / E3 = E5;
//
//     E8 / E4 = E2;
//     E12 / E4 = E3;
//     E16 / E4 = E4;
//
//     E10 / E5 = E2;
//     E15 / E5 = E3;
//
//     E12 / E6 = E2;
// }
//endregion


/*//  TODO: Use this when specialization is stable enough.
impl<U: Unit, E: CanDiv<D>, D> CanRoot<D> for UnitPow<U, E> {
    default type Output = UnitPow<U, E::Quotient>;
    default fn root(self) -> Self::Output { UnitPow::new(self.0) }
}

macro_rules! impl_cancelling_roots {($($e:ident),*$(,)?) => {
    $(impl<U: Unit> CanRoot<$e> for UnitPow<U, $e> {
        type Output = U;
        fn root(self) -> Self::Output { self.0 }
    })*
}}

impl_cancelling_roots!(
    E1,  E2,  E3,  E4,  E5,  E6,  E7,  E8,
    E9,  E10, E11, E12, E13, E14, E15, E16,
);*/
//endregion
//endregion


// impl<U: Unit> Cancel for UnitPow<U, E0> {
//     fn cancel(&self) -> f64 { 1.0 }
// }

// impl<U: UnitNonExp> Simplify<U> for UnitPow<U, E1> {
//     fn simplify<S: Scalar>(self) -> Conversion<U, S> { Conversion::units(self.0) }
// }
