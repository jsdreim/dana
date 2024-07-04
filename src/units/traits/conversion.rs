use std::ops::Mul;
use crate::Scalar;
use super::{Unit, UnitBinary};


pub enum Transform<S: Scalar + 'static> {
    None,
    Scale(S),
    Func(&'static fn(S) -> S),
}

impl<S: Scalar + Clone + Mul> Transform<S> {
    pub fn apply(&self, value: S) -> S {
        match self {
            Self::None => value,
            Self::Scale(s) => value * s.clone(),
            Self::Func(f) => f(value),
        }
    }
}


pub struct Conversion<U: Unit, S: Scalar + 'static> {
    target: U,
    transform: Transform<S>,
}

impl<U: Unit, S: Scalar> Conversion<U, S> {
    pub fn basic() -> Self {
        Self::units(U::default())
    }

    pub const fn units(target: U) -> Self {
        Self { target, transform: Transform::None }
    }

    pub const fn scale(target: U, scale: S) -> Self {
        Self { target, transform: Transform::Scale(scale) }
    }

    pub const fn func(target: U, f: &'static fn(S) -> S) -> Self {
        Self { target, transform: Transform::Func(f) }
    }

    pub fn apply(&self, value: S) -> crate::Quantity<U, S> where
        S: Clone,
    {
        self.target.quantity(self.transform.apply(value))
    }

    pub fn map_unit<V: Unit>(self, f: impl FnOnce(U) -> V) -> Conversion<V, S> {
        Conversion {
            target: f(self.target),
            transform: self.transform,
        }
    }
}


pub trait ConvertTo<U: Unit>: Unit {
    /// Given another unit, return the multiplication factor needed to convert
    ///     from this unit to the other unit.
    fn conversion_factor(&self, unit: U) -> f64;

    fn conversion<S: Scalar>(self, unit: U) -> Conversion<U, S> {
        let factor = self.conversion_factor(unit);
        Conversion::scale(unit, S::from_f64(factor).unwrap())
    }
}

pub trait ConvertLeft<U: Unit>: UnitBinary {
    type WithLeftConverted: UnitBinary;
    fn convert_left<S: Scalar>(&self, unit: U)
        -> Conversion<Self::WithLeftConverted, S>;
}

pub trait ConvertRight<U: Unit>: UnitBinary {
    type WithRightConverted: UnitBinary;
    fn convert_right<S: Scalar>(&self, unit: U)
        -> Conversion<Self::WithRightConverted, S>;
}


pub trait Cancel: Unit {
    fn cancel(&self) -> f64;
    fn cancel_to<S: Scalar>(&self) -> S { S::from_f64(self.cancel()).unwrap() }
}

pub trait CancelLeft: UnitBinary {
    type WithLeftCancelled: Unit;
    fn cancel_left<S: Scalar>(&self) -> Conversion<Self::WithLeftCancelled, S>;
}

pub trait CancelRight: UnitBinary {
    type WithRightCancelled: Unit;
    fn cancel_right<S: Scalar>(&self) -> Conversion<Self::WithRightCancelled, S>;
}


pub trait Simplify<U: Unit>: Unit {
    fn simplify<S: Scalar>(self) -> Conversion<U, S>;
}

pub trait SimplifyLeft<U: Unit>: UnitBinary {
    type WithLeftSimplified: UnitBinary;
    fn simplify_left<S: Scalar>(&self) -> Conversion<Self::WithLeftSimplified, S>;
}

pub trait SimplifyRight<U: Unit>: UnitBinary {
    type WithRightSimplified: UnitBinary;
    fn simplify_right<S: Scalar>(&self) -> Conversion<Self::WithRightSimplified, S>;
}
