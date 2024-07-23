use std::marker::PhantomData;
use num_traits::AsPrimitive;
use crate::{dimension::*, Scalar, units::traits::*};


#[derive(Clone, Copy, Debug, Eq, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct UnitAnon<D: DimType, S: Scalar + AsPrimitive<f64> + Copy = f64>(pub S, PhantomData<D>);

impl<D: DimType, S: Scalar + AsPrimitive<f64> + Copy> UnitAnon<D, S> {
    pub const fn new(s: S) -> Self { Self(s, PhantomData) }
}

impl<D: DimType, S: Scalar + AsPrimitive<f64> + Copy> Default for UnitAnon<D, S> {
    fn default() -> Self { Self(S::one(), PhantomData) }
}

impl<D: DimType, S: Scalar + AsPrimitive<f64> + Copy> PartialEq for UnitAnon<D, S> {
    fn eq(&self, other: &Self) -> bool { self.0.eq(&other.0) }
}

impl<D: DimType, S: Scalar + AsPrimitive<f64> + Copy> Unit for UnitAnon<D, S> {
    type Dim = D;
    // type ScaleType = S;

    fn scale(&self) -> f64 {
        self.0.as_()
    }
}

impl<D: DimType, S: Scalar + AsPrimitive<f64> + Copy> std::fmt::Display for UnitAnon<D, S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "*{}", self.0)
    }
}
