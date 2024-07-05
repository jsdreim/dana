use std::ops::Mul;
use crate::Scalar;
use super::{Unit, UnitBinary};


pub enum Transform<S: Scalar + 'static> {
    None,
    Scale(S),
    Func(&'static fn(S) -> S),
}

impl<S: Scalar + Mul> Transform<S> {
    pub fn apply(&self, value: S) -> S {
        match self {
            Self::None => value,
            Self::Scale(s) => value * s.clone(),
            Self::Func(f) => f(value),
        }
    }

    pub fn to_func(&self) -> Box<dyn Fn(S) -> S> {
        match self {
            Self::None => Box::new(|v| v),
            Self::Scale(s) => {
                let s: S = s.clone();
                Box::new(move |v| v * s.clone())
            }
            Self::Func(f) => Box::new(*f),
        }
    }
}


pub struct Conversion<U: Unit, S: Scalar + 'static> {
    target: U,
    transform: Transform<S>,
}

impl<U: Unit, S: Scalar> Conversion<U, S> {
    // pub fn basic() -> Self {
    //     Self::units(U::default())
    // }

    /// Direct conversion of units, with no effect on scalar value.
    pub const fn units(target: U) -> Self {
        Self { target, transform: Transform::None }
    }

    /// Conversion of units with simple scaling coefficient.
    pub const fn scale(target: U, scale: S) -> Self {
        Self { target, transform: Transform::Scale(scale) }
    }

    /// Conversion of units with an arbitrary transformation.
    pub const fn func(target: U, f: &'static fn(S) -> S) -> Self {
        Self { target, transform: Transform::Func(f) }
    }

    pub fn scalar(&self, value: S) -> S {
        self.transform.apply(value)
    }

    pub fn quantity(&self, value: S) -> crate::Quantity<U, S> {
        self.target.quantity(self.scalar(value))
    }

    pub fn map_unit<V: Unit>(self, f: impl FnOnce(U) -> V) -> Conversion<V, S> {
        Conversion {
            target: f(self.target),
            transform: self.transform,
        }
    }
}


pub trait ConvertFrom<U: Unit>: Unit {
    /// Given another unit, return the multiplication factor needed to convert
    ///     to this unit from the other unit.
    fn conversion_factor_from(&self, unit: U) -> f64;

    fn conversion_from<S: Scalar>(self, unit: U) -> Conversion<U, S> {
        let factor = self.conversion_factor_from(unit);
        Conversion::scale(unit, S::from_f64(factor).unwrap())
    }
}


impl<U: Unit, V: ConvertFrom<U>> ConvertInto<V> for U {
    fn conversion_factor_into(&self, unit: V) -> f64 {
        unit.conversion_factor_from(*self)
    }
}


pub trait ConvertInto<U: Unit>: Unit {
    /// Given another unit, return the multiplication factor needed to convert
    ///     from this unit to the other unit.
    fn conversion_factor_into(&self, unit: U) -> f64;

    fn conversion_into<S: Scalar>(self, unit: U) -> Conversion<U, S> {
        let factor = self.conversion_factor_into(unit);
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
