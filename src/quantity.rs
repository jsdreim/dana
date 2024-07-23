use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign};
use num_traits::{Float, Inv, NumCast, Pow, real::Real, Zero};
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

    pub fn value_as_base(self) -> V {
        self.value_as(U::default())
    }

    pub fn normalize(self) -> Self where
        U: UnitScale,
        V: Float,
    {
        if self.value.is_zero() {
            self.with_base()
        } else {
            let limit = V::from_u8(3).unwrap();
            let mut log: V = self.value.log10();
            let mut unit: U = self.unit;

            if log.is_sign_positive() {
                while log >= limit {
                    let Some(next) = unit.next_up() else { break };

                    let log_rel = unit.scale_factor(next).log10();
                    let log_new = log + V::from_f64(log_rel).unwrap();

                    if log_new >= V::zero() {
                        log = log_new;
                        unit = next;
                    } else {
                        break;
                    }
                }
            } else {
                while log < V::zero() {
                    let Some(next) = unit.next_down() else { break };

                    let log_rel = unit.scale_factor(next).log10();
                    let log_new = log + V::from_f64(log_rel).unwrap();

                    //  TODO: `<=` or `<`?
                    if log_new <= limit {
                        log = log_new;
                        unit = next;
                    } else {
                        break;
                    }
                }
            }

            self.with_unit(unit)
        }
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

    pub fn pow<const E: i32>(self) -> Quantity<
        U::Output,
        <V as Pow<V>>::Output,
    > where
        U: CanPow<E>,
        V: Pow<V>,
        <V as Pow<V>>::Output: Scalar,
    {
        Quantity {
            value: self.value.pow(V::from_i32(E).unwrap()),
            unit: self.unit.pow(),
        }
    }
    //endregion

    //region Roots.
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

    pub fn root<const D: i32>(self) -> Quantity<
        U::Output,
        <V as Pow<<V as Inv>::Output>>::Output,
    > where
        U: CanRoot<D>,
        V: Inv + Pow<<V as Inv>::Output>,
        <V as Pow<<V as Inv>::Output>>::Output: Scalar,
    {
        Quantity {
            value: self.value.pow(V::from_i32(D).unwrap().inv()),
            unit: self.unit.root(),
        }
    }
    //endregion
}
//endregion


//region Methods for scalar operations.
impl<U: Unit, V: Scalar> Quantity<U, V> {
    pub fn scalar_into<X>(self) -> Quantity<U, X> where
        V: Into<X>,
        X: Scalar,
    {
        Quantity::new(self.unit, self.value.into())
    }

    pub fn scalar_try_into<X>(self) -> Result<Quantity<U, X>, V::Error> where
        V: TryInto<X>,
        X: Scalar,
    {
        Ok(Quantity::new(self.unit, self.value.try_into()?))
    }
}
//endregion


//region Methods for unit operations.
/// Unit conversion.
impl<U: Unit, V: Scalar> Quantity<U, V> {
    /// Perform trait-based unit conversion to the default of a unit type. This
    ///     kind of conversion can cross between [`Unit`] types.
    pub fn convert<W: Unit>(self) -> Quantity<W, V> where
        U: ConvertInto<W>,
    {
        self.convert_to(W::default())
    }

    /// Perform trait-based unit conversion to a specific unit. This kind of
    ///     conversion can cross between [`Unit`] types.
    pub fn convert_to<W: Unit>(self, unit: W) -> Quantity<W, V> where
        U: ConvertInto<W>,
    {
        self.unit.conversion_into(unit).quantity(self.value)
    }

    /// Cancel out units entirely, returning a scalar.
    pub fn cancel(self) -> V where
        U: Cancel,
    {
        self.value * self.unit.cancel()
    }
}
//endregion


//region Standard library operators.
//region Negation.
impl<U: Unit, V: Scalar> Neg for Quantity<U, V> where
    V: Neg, <V as Neg>::Output: Scalar,
{
    type Output = Quantity<U, <V as Neg>::Output>;

    fn neg(self) -> Self::Output {
        Quantity {
            value: self.value.neg(),
            unit: self.unit,
        }
    }
}
//endregion

//region Addition/subtraction between same-unit quantities.
impl<U: Unit, V: Scalar, W: Unit, X: Scalar> Add<Quantity<W, X>> for Quantity<U, V> where
    W: ConvertInto<U>,
    V: Add<X>, <V as Add<X>>::Output: Scalar,
{
    type Output = Quantity<U, <V as Add<X>>::Output>;

    fn add(self, rhs: Quantity<W, X>) -> Self::Output {
        Quantity {
            value: self.value + rhs.convert_to(self.unit).value,
            unit: self.unit,
        }
    }
}

impl<U: Unit, V: Scalar, W: Unit, X: Scalar> Sub<Quantity<W, X>> for Quantity<U, V> where
    W: ConvertInto<U>,
    V: Sub<X>, <V as Sub<X>>::Output: Scalar,
{
    type Output = Quantity<U, <V as Sub<X>>::Output>;

    fn sub(self, rhs: Quantity<W, X>) -> Self::Output {
        Quantity {
            value: self.value - rhs.convert_to(self.unit).value,
            unit: self.unit,
        }
    }
}

impl<U: Unit, V: Scalar, W: Unit, X: Scalar> AddAssign<Quantity<W, X>> for Quantity<U, V> where
    W: ConvertInto<U>,
    V: AddAssign<X>,
{
    fn add_assign(&mut self, rhs: Quantity<W, X>) {
        self.value += rhs.convert_to(self.unit).value;
    }
}

impl<U: Unit, V: Scalar, W: Unit, X: Scalar> SubAssign<Quantity<W, X>> for Quantity<U, V> where
    W: ConvertInto<U>,
    V: SubAssign<X>,
{
    fn sub_assign(&mut self, rhs: Quantity<W, X>) {
        self.value -= rhs.convert_to(self.unit).value;
    }
}
//endregion

//region Division/multiplication between quantities.
impl<U: Unit, V: Scalar, W: Unit, X: Scalar> Div<Quantity<W, X>> for Quantity<U, V> where
    U: Div<W>, <U as Div<W>>::Output: Unit,
    V: Div<X>, <V as Div<X>>::Output: Scalar,
{
    type Output = Quantity<U::Output, <V as Div<X>>::Output>;

    fn div(self, rhs: Quantity<W, X>) -> Self::Output {
        Quantity {
            value: self.value / rhs.value,
            unit: self.unit / rhs.unit,
        }
    }
}

impl<U: Unit, V: Scalar, W: Unit, X: Scalar> Mul<Quantity<W, X>> for Quantity<U, V> where
    U: Mul<W>, <U as Mul<W>>::Output: Unit,
    V: Mul<X>, <V as Mul<X>>::Output: Scalar,
{
    type Output = Quantity<U::Output, <V as Mul<X>>::Output>;

    fn mul(self, rhs: Quantity<W, X>) -> Self::Output {
        Quantity {
            value: self.value * rhs.value,
            unit: self.unit * rhs.unit,
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

//region Comparison between quantities.
//region Equivalence.
impl<U: Unit, V: Scalar, W: Unit, X: Scalar> PartialEq<Quantity<W, X>>
for Quantity<U, V> where
    W: ConvertInto<U>,
    V: PartialEq<X>,
    X: Clone,
{
    fn eq(&self, other: &Quantity<W, X>) -> bool {
        let comp = other.clone().convert_to(self.unit);
        self.value.eq(&comp.value)
    }
}

impl<U: Unit, V: Scalar + Eq> Eq for Quantity<U, V> where
    Quantity<U, V>: PartialEq,
{}
//endregion

//region Ordering.
impl<U: Unit, V: Scalar, W: Unit, X: Scalar> PartialOrd<Quantity<W, X>>
for Quantity<U, V> where
    W: ConvertInto<U>,
    V: PartialOrd<X>,
    X: Clone,
{
    fn partial_cmp(&self, other: &Quantity<W, X>) -> Option<std::cmp::Ordering> {
        let comp = other.clone().convert_to(self.unit);
        self.value.partial_cmp(&comp.value)
    }
}

impl<U: Unit, V: Scalar + Ord> Ord for Quantity<U, V> where
    U: ConvertInto<U>,
    Quantity<U, V>: PartialOrd,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let comp = other.clone().convert_to(self.unit);
        self.value.cmp(&comp.value)
    }
}
//endregion
//endregion
//endregion


//region Traits from `num_traits`.
impl<U: Unit, V: Scalar + Float> Quantity<U, V> {
    /// Cast the scalar to another type through the [`NumCast`] trait.
    pub fn scalar_cast<X: Scalar + NumCast>(self) -> Option<Quantity<U, X>> {
        Some(Quantity::new(self.unit, X::from(self.value)?))
    }

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


// impl<U: Unit, V: Scalar + From<X>, X: Scalar> From<Quantity<U, X>> for Quantity<U, V> {
//     fn from(qty: Quantity<U, X>) -> Self {
//         Self::new(qty.unit, qty.value.into())
//     }
// }
//
// impl<U: Unit, V: Scalar + TryFrom<X>, X: Scalar> TryFrom<Quantity<U, X>> for Quantity<U, V> {
//     type Error = V::Error;
//
//     fn try_from(qty: Quantity<U, X>) -> Result<Self, Self::Error> {
//         Ok(Self::new(qty.unit, qty.value.try_into()?))
//     }
// }


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
