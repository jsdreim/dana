//! Module for unit conversion machinery.

//  TODO: Can all of this be removed in favor of doing it in Quantity methods?
#![allow(missing_docs)]

use crate::{Unit, Value};


pub struct Conversion<U: Unit, S: Value> {
    pub target: U,
    pub factor: S,
}

impl<U: Unit, S: Value> Conversion<U, S> {
    pub const fn new(target: U, factor: S) -> Self {
        Self { target, factor }
    }

    pub fn scalar(&self, value: S) -> S {
        value * self.factor.clone()
    }

    pub fn quantity(&self, value: S) -> crate::Quantity<U, S> {
        self.target.quantity(self.scalar(value))
    }

    pub fn map_unit<V: Unit>(self, f: impl FnOnce(U) -> V) -> Conversion<V, S> {
        Conversion {
            target: f(self.target),
            factor: self.factor,
        }
    }

    pub fn map_factor<T: Value>(self, f: impl FnOnce(S) -> T) -> Conversion<U, T> {
        Conversion {
            target: self.target,
            factor: f(self.factor),
        }
    }
}


pub trait ConvertFrom<U: Unit>: Unit {
    /// Given another unit, return the multiplication factor needed to convert
    ///     to this unit from the other unit.
    fn conversion_factor_from(&self, unit: U) -> f64 {
        unit.scale() / self.scale()
    }

    fn conversion_from<S: Value>(self, unit: U) -> Conversion<U, S> {
        let factor = self.conversion_factor_from(unit);
        Conversion::new(unit, crate::_conv_f64(factor))
    }
}


impl<U, W> ConvertFrom<U> for W where
    U: Unit, W: Unit<Dim=U::Dim>,
{}


pub trait ConvertInto<U: Unit>: Unit {
    /// Given another unit, return the multiplication factor needed to convert
    ///     from this unit to the other unit.
    fn conversion_factor_into(&self, unit: U) -> f64;

    fn conversion_into<S: Value>(self, unit: U) -> Conversion<U, S> {
        let factor = self.conversion_factor_into(unit);
        Conversion::new(unit, crate::_conv_f64(factor))
    }
}

impl<U: Unit, V: ConvertFrom<U>> ConvertInto<V> for U {
    fn conversion_factor_into(&self, unit: V) -> f64 {
        unit.conversion_factor_from(*self)
    }
}


pub trait Cancel: Unit {
    fn cancel_factor(&self) -> f64 { self.scale() }

    fn cancel<S: Value>(&self) -> S {
        crate::_conv_f64(self.cancel_factor())
    }
}

impl<U> Cancel for U where
    U: Unit<Dim=crate::dimension::One>,
{}
