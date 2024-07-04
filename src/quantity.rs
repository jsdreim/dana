use std::ops::{Add, Sub};
use crate::{Scalar, units::traits::*};


type ScalarDefault = f64;


/// Dimensionless [`Scalar`] value paired with a dimensional [`Unit`].
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Quantity<U: Unit, V: Scalar = ScalarDefault> {
    pub value: V,
    pub unit: U,
}

impl<U: Unit, V: Scalar> Quantity<U, V> {
    pub const fn new(unit: U, value: V) -> Self {
        Self { value, unit }
    }

    // pub fn set_base(&mut self) { self.set_unit(U::default()) }

    pub fn set_unit(&mut self, unit: U) where
        V: std::ops::MulAssign,
    {
        let factor = self.unit.scale_factor(unit);

        self.value *= V::from_f64(factor).unwrap();
        self.unit = unit;
    }

    pub fn with_base(self) -> Self { self.with_unit(U::default()) }

    pub fn with_unit(self, unit: U) -> Self {
        if unit == self.unit {
            self
        } else {
            Self {
                value: self.value_as(unit),
                unit,
            }
        }
    }

    /// Return the value of this quantity, scaled to another unit.
    pub fn value_as(self, unit: U) -> V {
        let factor = self.unit.scale_factor(unit);
        self.value * V::from_f64(factor).unwrap()
    }
}


/// Unit conversion.
impl<U: Unit, V: Scalar + 'static> Quantity<U, V> {
    /// Perform trait-based unit conversion. This kind of conversion can cross
    ///     between [`Unit`] types.
    pub fn convert_to<W: Unit>(self, unit: W) -> Quantity<W, V> where
        U: ConvertTo<W>,
        V: Clone,
    {
        self.unit.conversion(unit).apply(self.value)
    }

    pub fn convert_left_to<W: Unit>(self, unit: W)
        -> Quantity<U::WithLeftConverted, V> where
        U: ConvertLeft<W>,
        U::Left: ConvertTo<W>,
        V: Clone,
    {
        self.unit.convert_left(unit).apply(self.value)
    }

    pub fn convert_right_to<W: Unit>(self, unit: W)
        -> Quantity<U::WithRightConverted, V> where
        U: ConvertRight<W>,
        U::Right: ConvertTo<W>,
        V: Clone,
    {
        self.unit.convert_right(unit).apply(self.value)
    }

    /// Simplify redundant units.
    pub fn simplify<W: Unit>(self) -> Quantity<W, V> where
        U: Simplify<W>,
        V: Clone,
    {
        self.unit.simplify::<V>().apply(self.value)
    }
}


/// Associative Property.
impl<U: Unit, V: Scalar> Quantity<U, V> {
    pub fn reassociate<W: Unit>(self) -> Quantity<W, V> where
        U: Associative<W>,
    { Quantity::new(self.unit.reassociate(), self.value) }

    pub fn reassociate_left<W: Unit>(self) -> Quantity<U::WithLeftReassociated, V> where
        U: AssociativeLeft<W>,
        U::Left: Associative<W>,
    { Quantity::new(self.unit.reassociate_left(), self.value) }

    pub fn reassociate_right<W: Unit>(self) -> Quantity<U::WithRightReassociated, V> where
        U: AssociativeRight<W>,
        U::Right: Associative<W>,
    { Quantity::new(self.unit.reassociate_right(), self.value) }
}

/// Commutative Property.
impl<U: Unit, V: Scalar> Quantity<U, V> {
    pub fn commute(self) -> Quantity<U::Commuted, V> where
        U: Commutative,
    { Quantity::new(self.unit.commute(), self.value) }

    pub fn commute_left(self) -> Quantity<U::WithLeftCommuted, V> where
        U: CommutativeLeft,
        U::Left: Commutative,
    { Quantity::new(self.unit.commute_left(), self.value) }

    pub fn commute_right(self) -> Quantity<U::WithRightCommuted, V> where
        U: CommutativeRight,
        U::Right: Commutative,
    { Quantity::new(self.unit.commute_right(), self.value) }
}


impl<U: Unit, V: Scalar, W: Scalar> Add<Quantity<U, W>> for Quantity<U, V> where
    V: Add<W>, <V as Add<W>>::Output: Scalar,
{
    type Output = Quantity<U, <V as Add<W>>::Output>;

    fn add(self, rhs: Quantity<U, W>) -> Self::Output {
        Quantity {
            value: self.value + rhs.value_as(self.unit),
            unit: self.unit,
        }
    }
}

impl<U: Unit, V: Scalar, W: Scalar> Sub<Quantity<U, W>> for Quantity<U, V> where
    V: Sub<W>, <V as Sub<W>>::Output: Scalar,
{
    type Output = Quantity<U, <V as Sub<W>>::Output>;

    fn sub(self, rhs: Quantity<U, W>) -> Self::Output {
        Quantity {
            value: self.value - rhs.value_as(self.unit),
            unit: self.unit,
        }
    }
}


// impl<U: Unit, V: Scalar> num_traits:: for Quantity<U, V> {}
// impl<U: Unit, V: Scalar> num_traits::Signed for Quantity<U, V> {}


impl<U: Unit, V: Scalar> num_traits::Zero for Quantity<U, V> {
    fn zero() -> Self {
        Self { value: V::zero(), unit: U::default() }
    }

    fn is_zero(&self) -> bool {
        self.value.is_zero()
    }
}
