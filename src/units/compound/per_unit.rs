use num_traits::Inv;
use crate::units::{dim::*, traits::*};


/// The reciprocal of a unit.
#[derive(Clone, Copy, Debug, Default,
Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct PerUnit<U: Unit>(pub U) where
    U::Dim: Inv,
    <U::Dim as Inv>::Output: DimType,
;

impl<U: Unit> PerUnit<U> where
    U::Dim: Inv,
    <U::Dim as Inv>::Output: DimType,
{
    pub const fn denominator(&self) -> U { self.0 }
}

impl<U: Unit> Unit for PerUnit<U> where
    U::Dim: Inv,
    <U::Dim as Inv>::Output: DimType,
{
    type Dim = <U::Dim as Inv>::Output;
    // type Dimension = None;
    // type ScaleType = f64;

    fn scale(&self) -> f64 {
        1.0 / self.0.scale()
    }
}

impl<U: Unit> std::fmt::Display for PerUnit<U> where
    U::Dim: Inv,
    <U::Dim as Inv>::Output: DimType,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // write!(f, "1/{:#}", self.0)
        // write!(f, "{:#}⁻¹", self.0)
        write!(f, "{:#}^-1", self.0)
    }
}


// impl<U1: ConvertInto<U2>, U2: Unit> ConvertFrom<PerUnit<U1>> for PerUnit<U2> {
//     fn conversion_factor_from(&self, unit: PerUnit<U1>) -> f64 {
//         1.0 / unit.0.conversion_factor_into(self.0)
//     }
// }

// impl<U: Unit> UnitNonExp for PerUnit<U> {}


// impl<U: Unit> Inv for PerUnit<U> where
//     <Self as Unit>::Dim: Inv<Output=U>,
// {
//     type Output = U;
//     fn inv(self) -> Self::Output { self.0 }
// }
