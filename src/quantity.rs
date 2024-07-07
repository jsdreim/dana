use std::ops::{Add, Div, Mul, Sub};
use num_traits::{Float, Inv, Pow, real::Real, Zero};
use crate::{Scalar, units::{compound::*, traits::*}};


type ScalarDefault = f64;


/// Dimensionless [`Scalar`] value paired with a dimensional [`Unit`].
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Quantity<U: Unit, V: Scalar = ScalarDefault> {
    pub value: V,
    pub unit: U,
}

impl<U: Unit, V: Scalar, W: Unit, X: Scalar + 'static> PartialEq<Quantity<W, X>>
for Quantity<U, V> where
    W: ConvertInto<U>,
    V: PartialEq<X>,
    X: Clone,
{
    fn eq(&self, other: &Quantity<W, X>) -> bool {
        let comp = other.clone().convert(self.unit);
        self.value.eq(&comp.value)
    }
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

    pub fn value_as_base(self) -> V {
        self.value_as(U::default())
    }
}


//region Methods for mathematical operations.
impl<U: Unit, V: Scalar> Quantity<U, V> {
    //region Positive exponents.
    pub fn squared(self) -> Quantity<<U as CanSquare>::Output, <V as Mul<V>>::Output> where
        U: CanSquare,
        V: Mul<V> + Clone,
    {
        Quantity {
            value: self.value.clone() * self.value,
            unit: self.unit.squared(),
        }
    }

    pub fn cubed(self) -> Quantity<<U as CanCube>::Output, <V as Mul<V>>::Output> where
        U: CanCube,
        V: Mul<V> + Clone,
    {
        Quantity {
            value: self.value.clone() * self.value.clone() * self.value,
            unit: self.unit.cubed(),
        }
    }

    pub fn pow<E>(self, exp: E) -> Quantity<<U as CanPow<E>>::Output, <V as Pow<E>>::Output> where
        E: unit_pow_n::Exp,
        U: CanPow<E>,
        V: Pow<E>,
        <V as Pow<E>>::Output: Scalar,
    {
        Quantity {
            value: self.value.pow(exp),
            unit: self.unit.pow(exp),
        }
    }
    //endregion

    //region Negative exponents.
    pub fn sqrt(self) -> Quantity<<U as CanSquareRoot>::Output, V> where
        U: CanSquareRoot,
        V: Real,
    {
        Quantity {
            value: self.value.sqrt(),
            unit: self.unit.sqrt(),
        }
    }

    pub fn cbrt(self) -> Quantity<<U as CanCubeRoot>::Output, V> where
        U: CanCubeRoot,
        V: Real,
    {
        Quantity {
            value: self.value.cbrt(),
            unit: self.unit.cbrt(),
        }
    }

    pub fn root<D>(self, degree: D) -> Quantity<
        <U as CanRoot<D>>::Output,
        <V as Pow<<D as Inv>::Output>>::Output,
    > where
        D: unit_pow_n::Exp + Inv,
        U: CanRoot<D>,
        V: Pow<<D as Inv>::Output>,
        <V as Pow<<D as Inv>::Output>>::Output: Scalar,
    {
        Quantity {
            value: self.value.pow(degree.inv()),
            unit: self.unit.root(degree),
        }
    }
    //endregion
}
//endregion


//region Methods for unit operations.
/// Unit conversion.
impl<U: Unit, V: Scalar + 'static> Quantity<U, V> {
    /// Perform trait-based unit conversion. This kind of conversion can cross
    ///     between [`Unit`] types.
    pub fn convert<W: Unit>(self, unit: W) -> Quantity<W, V> where
        U: ConvertInto<W>,
    {
        self.unit.conversion_into(unit).quantity(self.value)
    }

    pub fn convert_left_to<W: Unit>(self, unit: W)
        -> Quantity<U::WithLeftConverted, V> where
        U: ConvertLeft<W>,
        U::Left: ConvertInto<W>,
    {
        self.unit.convert_left(unit).quantity(self.value)
    }

    pub fn convert_right_to<W: Unit>(self, unit: W)
        -> Quantity<U::WithRightConverted, V> where
        U: ConvertRight<W>,
        U::Right: ConvertInto<W>,
    {
        self.unit.convert_right(unit).quantity(self.value)
    }

    /// Simplify redundant units.
    pub fn simplify<W: Unit>(self) -> Quantity<W, V> where
        U: Simplify<W>,
    {
        self.unit.simplify::<V>().quantity(self.value)
    }
}
//endregion


//region Methods for value-equivalent unit reorganization.
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
//endregion


//region Standard library operators.
//region Addition/subtraction between same-unit quantities.
impl<U: Unit, V: Scalar, X: Scalar> Add<Quantity<U, X>> for Quantity<U, V> where
    V: Add<X>, <V as Add<X>>::Output: Scalar,
{
    type Output = Quantity<U, <V as Add<X>>::Output>;

    fn add(self, rhs: Quantity<U, X>) -> Self::Output {
        Quantity {
            value: self.value + rhs.value_as(self.unit),
            unit: self.unit,
        }
    }
}

impl<U: Unit, V: Scalar, X: Scalar> Sub<Quantity<U, X>> for Quantity<U, V> where
    V: Sub<X>, <V as Sub<X>>::Output: Scalar,
{
    type Output = Quantity<U, <V as Sub<X>>::Output>;

    fn sub(self, rhs: Quantity<U, X>) -> Self::Output {
        Quantity {
            value: self.value - rhs.value_as(self.unit),
            unit: self.unit,
        }
    }
}
//endregion

//region Division/multiplication between quantities.
impl<U: Unit, V: Scalar, W: Unit, X: Scalar> Div<Quantity<W, X>> for Quantity<U, V> where
    V: Div<X>, <V as Div<X>>::Output: Scalar,
{
    type Output = Quantity<UnitDiv<U, W>, <V as Div<X>>::Output>;

    fn div(self, rhs: Quantity<W, X>) -> Self::Output {
        Quantity {
            value: self.value / rhs.value,
            unit: UnitDiv(self.unit, rhs.unit),
        }
    }
}

impl<U: Unit, V: Scalar, W: Unit, X: Scalar> Mul<Quantity<W, X>> for Quantity<U, V> where
    V: Mul<X>, <V as Mul<X>>::Output: Scalar,
{
    type Output = Quantity<UnitMul<U, W>, <V as Mul<X>>::Output>;

    fn mul(self, rhs: Quantity<W, X>) -> Self::Output {
        Quantity {
            value: self.value * rhs.value,
            unit: UnitMul(self.unit, rhs.unit),
        }
    }
}
//endregion

//region Division/multiplication between quantities and scalars.
impl<U: Unit, V: Scalar, X: Scalar> Div<X> for Quantity<U, V> where
    V: Div<X>, <V as Div<X>>::Output: Scalar,
{
    type Output = Quantity<U, <V as Div<X>>::Output>;

    fn div(self, rhs: X) -> Self::Output {
        Quantity {
            value: self.value / rhs,
            unit: self.unit,
        }
    }
}

impl<U: Unit, V: Scalar, X: Scalar> Mul<X> for Quantity<U, V> where
    V: Mul<X>, <V as Mul<X>>::Output: Scalar,
{
    type Output = Quantity<U, <V as Mul<X>>::Output>;

    fn mul(self, rhs: X) -> Self::Output {
        Quantity {
            value: self.value * rhs,
            unit: self.unit,
        }
    }
}
//endregion

//region Division/multiplication between quantities and pure units.
impl<U: Unit, V: Scalar> Quantity<U, V> {
    pub fn div_unit<W: Unit>(self, rhs: W) -> Quantity<<U as Div<W>>::Output, V> where
        U: Div<W>, <U as Div<W>>::Output: Unit,
    {
        Quantity {
            value: self.value,
            unit: self.unit / rhs,
        }
    }

    pub fn mul_unit<W: Unit>(self, rhs: W) -> Quantity<<U as Mul<W>>::Output, V> where
        U: Mul<W>, <U as Mul<W>>::Output: Unit,
    {
        Quantity {
            value: self.value,
            unit: self.unit * rhs,
        }
    }
}
//endregion
//endregion


//region Traits from `num_traits`.
impl<U: Unit, V: Scalar + Float> Quantity<U, V> {
    pub fn abs(self) -> Self {
        Self::new(self.unit, self.value.abs())
    }

    pub fn ceil(self) -> Self {
        Self::new(self.unit, self.value.ceil())
    }

    pub fn floor(self) -> Self {
        Self::new(self.unit, self.value.floor())
    }

    pub fn round(self) -> Self {
        Self::new(self.unit, self.value.round())
    }

    pub fn trunc(self) -> Self {
        Self::new(self.unit, self.value.trunc())
    }
}

// impl<U: Unit, V: Scalar> num_traits:: for Quantity<U, V> {}

impl<U: Unit, V: Scalar> Inv for Quantity<U, V> where
    U: Inv, <U as Inv>::Output: Unit,
    V: Inv, <V as Inv>::Output: Scalar,
{
    type Output = Quantity<<U as Inv>::Output, <V as Inv>::Output>;

    fn inv(self) -> Self::Output {
        Quantity::new(self.unit.inv(), self.value.inv())
    }
}


impl<U: Unit, V: Scalar> Zero for Quantity<U, V> {
    fn zero() -> Self {
        Self { value: V::zero(), unit: U::default() }
    }

    fn is_zero(&self) -> bool {
        self.value.is_zero()
    }
}
//endregion


macro_rules! impl_fmt {
    ($($fmt:path),+$(,)?) => {$(
    impl<U: Unit, V: Scalar> $fmt for Quantity<U, V> where V: $fmt {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            <V as $fmt>::fmt(&self.value, f)?;
            write!(f, " {}", self.unit)
        }
    }
    )+};
}

impl_fmt!(
    std::fmt::Display,
    std::fmt::Octal,
    std::fmt::LowerHex,
    std::fmt::UpperHex,
    std::fmt::Pointer,
    std::fmt::Binary,
    std::fmt::LowerExp,
    std::fmt::UpperExp,
);
