//! The link between dimensionless [`Value`]s and dimensional [`Unit`]s.

#[cfg(feature = "rand")]
pub mod rand;

mod conversions;

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use num_traits::{Inv, MulAdd, NumCast, Pow, real::Real, Signed, Zero};
use crate::{units::{traits::*, UnitAnon}, Value};


type ValueDefault = f64;


/// A [`Quantity`] with an [anonymous unit](UnitAnon).
pub type QuantityAnon<D, V = ValueDefault> = Quantity<UnitAnon<D>, V>;


/// Dimensionless [`Value`] value paired with a dimensional [`Unit`].
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Quantity<U: Unit, V: Value = ValueDefault> {
    /// Dimensionless value. This defines *how much* of the unit is represented
    ///     by the quantity.
    pub value: V,
    /// Dimensional unit. This defines *what* is represented by the quantity.
    pub unit: U,
}

impl<U: Unit, V: Value> Quantity<U, V> {
    /// Construct a new [`Quantity`] from [`Unit`] and [`Value`].
    pub const fn new(unit: U, value: V) -> Self {
        Self { value, unit }
    }

    // pub fn set_base(&mut self) { self.set_unit(U::default()) }

    /// Change the unit of this quantity in-place.
    pub fn set_unit(&mut self, unit: U) where
        V: MulAssign,
    {
        self.value *= self.unit.scale_factor_v(unit).unwrap();
        self.unit = unit;
    }

    /// Return `true` if another quantity is within the given limit of this one.
    pub fn almost_eq<W>(self, rhs: Quantity<W, V>, limit: V) -> bool where
        V: Signed + PartialOrd,
        W: Unit<Dim=U::Dim>,
    {
        (self - rhs).abs().value <= limit
    }

    /// Return an equivalent quantity with its unit [anonymized](UnitAnon).
    pub fn with_anonymous(self) -> QuantityAnon<U::Dim, V> {
        self.unit.anonymous().quantity(self.value)
    }

    /// Return an equivalent quantity with the base unit of the same type.
    pub fn with_base(self) -> Self { self.with_unit(U::base()) }

    /// Return an equivalent quantity with the given unit of the same type.
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
    pub fn value_as<W: Unit<Dim=U::Dim>>(self, unit: W) -> V {
        self.value * self.unit.scale_factor_v(unit).unwrap()
    }

    /// Return the value of this quantity, scaled to the base unit of its type.
    pub fn value_as_base(self) -> V {
        self.value_as(U::base())
    }

    /// Return an equivalent quantity with its value as close as possible to
    ///     being within the range `[1,1000)`.
    ///
    /// This is done by repeatedly "stepping" the unit up or down. As such, it
    ///     may be quite expensive for more complex compound units.
    pub fn normalize(self) -> Self where
        U: UnitStep,
        V: Real,
    {
        if self.value.is_zero() {
            self.with_base()
        } else {
            let limit = V::from_u8(3).unwrap();
            let mut log: V = self.value.log10();
            let mut unit: U = self.unit;

            if log.is_sign_positive() {
                while log >= limit {
                    let Some(next) = unit.step_up() else { break };

                    let log_rel = unit.scale_factor_v::<U, V>(next).unwrap().log10();
                    let log_new = log + log_rel;

                    if log_new >= V::zero() {
                        log = log_new;
                        unit = next;
                    } else {
                        break;
                    }
                }
            } else {
                while log < V::zero() {
                    let Some(next) = unit.step_down() else { break };

                    let log_rel = unit.scale_factor_v::<U, V>(next).unwrap().log10();
                    let log_new = log + log_rel;

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

    /// Return a [`QtySimd`](crate::simd::QtySimd) array, for SIMD operations,
    ///     populated by this quantity.
    #[cfg(feature = "simd")]
    pub fn to_simd<const N: usize, S>(self) -> crate::simd::QtySimd<U, V, N, S> where
        std::simd::LaneCount<N>: std::simd::SupportedLaneCount,
        V: crate::simd::QtySimdValue,
        S: crate::simd::QtySimdScale,
        f64: num_traits::AsPrimitive<S>,
    {
        crate::simd::QtySimd::from_qty(self)
    }
}


//region Methods for mathematical operations.
impl<U: Unit, V: Value> Quantity<U, V> {
    //region Positive exponents.
    /// Return the square of this quantity.
    pub fn squared(self) -> Quantity<<U as CanSquare>::Output, <V as Mul<V>>::Output> where
        U: CanSquare,
    {
        Quantity {
            value: self.value.clone() * self.value,
            unit: self.unit.squared(),
        }
    }

    /// Return the cube of this quantity.
    pub fn cubed(self) -> Quantity<<U as CanCube>::Output, <V as Mul<V>>::Output> where
        U: CanCube,
    {
        Quantity {
            value: self.value.clone() * self.value.clone() * self.value,
            unit: self.unit.cubed(),
        }
    }

    /// Return an arbitrary integer power of this quantity.
    pub fn pow<const E: i32>(self) -> Quantity<
        U::Output,
        <V as Pow<V>>::Output,
    > where
        U: CanPow<E>,
        V: Pow<V>,
        <V as Pow<V>>::Output: Value,
    {
        Quantity {
            value: self.value.pow(V::from_i32(E).unwrap()),
            unit: self.unit.pow(),
        }
    }
    //endregion

    //region Roots.
    /// Return the square root of this quantity.
    pub fn sqrt(self) -> Quantity<<U as CanSquareRoot>::Output, V> where
        U: CanSquareRoot,
        V: Real,
    {
        Quantity {
            value: self.value.sqrt(),
            unit: self.unit.sqrt(),
        }
    }

    /// Return the cube root of this quantity.
    pub fn cbrt(self) -> Quantity<<U as CanCubeRoot>::Output, V> where
        U: CanCubeRoot,
        V: Real,
    {
        Quantity {
            value: self.value.cbrt(),
            unit: self.unit.cbrt(),
        }
    }

    /// Return the root to an arbitrary integer degree of this quantity.
    pub fn root<const D: i32>(self) -> Quantity<
        U::Output,
        <V as Pow<<V as Inv>::Output>>::Output,
    > where
        U: CanRoot<D>,
        V: Inv + Pow<<V as Inv>::Output>,
        <V as Pow<<V as Inv>::Output>>::Output: Value,
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
impl<U: Unit, V: Value> Quantity<U, V> {
    /// Return an equivalent quantity, with the value converted to another type
    ///     via [`Into::into`].
    pub fn value_into<X>(self) -> Quantity<U, X> where
        V: Into<X>,
        X: Value,
    {
        Quantity::new(self.unit, self.value.into())
    }

    /// Return an equivalent quantity, with the value converted to another type
    ///     via [`TryInto::try_into`].
    pub fn value_try_into<X>(self) -> Result<Quantity<U, X>, V::Error> where
        V: TryInto<X>,
        X: Value,
    {
        Ok(Quantity::new(self.unit, self.value.try_into()?))
    }
}
//endregion


//region Methods for unit operations.
/// Unit conversion.
impl<U: Unit, V: Value> Quantity<U, V> {
    /// Perform trait-based unit conversion to the default of a unit type. This
    ///     kind of conversion can cross between [`Unit`] types.
    pub fn convert<W: Unit>(self) -> Quantity<W, V> where
        U: ConvertInto<W>,
    {
        self.convert_to(W::base())
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
impl<U: Unit, V: Value> Neg for Quantity<U, V> where
    V: Neg, <V as Neg>::Output: Value,
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
impl<U: Unit, V: Value, W: Unit, X: Value> Add<Quantity<W, X>> for Quantity<U, V> where
    W: ConvertInto<U>,
    V: Add<X>, <V as Add<X>>::Output: Value,
{
    type Output = Quantity<U, <V as Add<X>>::Output>;

    fn add(self, rhs: Quantity<W, X>) -> Self::Output {
        Quantity {
            value: self.value + rhs.convert_to(self.unit).value,
            unit: self.unit,
        }
    }
}

impl<U: Unit, V: Value, W: Unit, X: Value> Sub<Quantity<W, X>> for Quantity<U, V> where
    W: ConvertInto<U>,
    V: Sub<X>, <V as Sub<X>>::Output: Value,
{
    type Output = Quantity<U, <V as Sub<X>>::Output>;

    fn sub(self, rhs: Quantity<W, X>) -> Self::Output {
        Quantity {
            value: self.value - rhs.convert_to(self.unit).value,
            unit: self.unit,
        }
    }
}

impl<U: Unit, V: Value, W: Unit, X: Value> AddAssign<Quantity<W, X>> for Quantity<U, V> where
    W: ConvertInto<U>,
    V: AddAssign<X>,
{
    fn add_assign(&mut self, rhs: Quantity<W, X>) {
        self.value += rhs.convert_to(self.unit).value;
    }
}

impl<U: Unit, V: Value, W: Unit, X: Value> SubAssign<Quantity<W, X>> for Quantity<U, V> where
    W: ConvertInto<U>,
    V: SubAssign<X>,
{
    fn sub_assign(&mut self, rhs: Quantity<W, X>) {
        self.value -= rhs.convert_to(self.unit).value;
    }
}
//endregion

//region Division/multiplication between quantities.
impl<U: Unit, V: Value, W: Unit, X: Value> Div<Quantity<W, X>> for Quantity<U, V> where
    U: Div<W>, <U as Div<W>>::Output: Unit,
    V: Div<X>, <V as Div<X>>::Output: Value,
{
    type Output = Quantity<U::Output, <V as Div<X>>::Output>;

    fn div(self, rhs: Quantity<W, X>) -> Self::Output {
        Quantity {
            value: self.value / rhs.value,
            unit: self.unit / rhs.unit,
        }
    }
}

impl<U: Unit, V: Value, W: Unit, X: Value> Mul<Quantity<W, X>> for Quantity<U, V> where
    U: Mul<W>, <U as Mul<W>>::Output: Unit,
    V: Mul<X>, <V as Mul<X>>::Output: Value,
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
impl<U: Unit, V: Value, X: Value> Div<X> for Quantity<U, V> where
    V: Div<X>, <V as Div<X>>::Output: Value,
{
    type Output = Quantity<U, <V as Div<X>>::Output>;

    fn div(self, rhs: X) -> Self::Output {
        Quantity {
            value: self.value / rhs,
            unit: self.unit,
        }
    }
}

impl<U: Unit, V: Value, X: Value> Mul<X> for Quantity<U, V> where
    V: Mul<X>, <V as Mul<X>>::Output: Value,
{
    type Output = Quantity<U, <V as Mul<X>>::Output>;

    fn mul(self, rhs: X) -> Self::Output {
        Quantity {
            value: self.value * rhs,
            unit: self.unit,
        }
    }
}

impl<U: Unit, V: Value, X: Value> DivAssign<X> for Quantity<U, V> where
    V: DivAssign<X>,
{
    fn div_assign(&mut self, rhs: X) {
        self.value /= rhs;
    }
}

impl<U: Unit, V: Value, X: Value> MulAssign<X> for Quantity<U, V> where
    V: MulAssign<X>,
{
    fn mul_assign(&mut self, rhs: X) {
        self.value *= rhs;
    }
}
//endregion

//region Division/multiplication between quantities and pure units.
/*impl<U: Unit, V: Value> Quantity<U, V> {
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
}*/
//endregion

//region Comparison between quantities.
//region Equivalence.
impl<U: Unit, V: Value, W: Unit, X: Value> PartialEq<Quantity<W, X>>
for Quantity<U, V> where
    W: ConvertInto<U>,
    V: PartialEq<X>,
{
    fn eq(&self, other: &Quantity<W, X>) -> bool {
        let comp = other.clone().convert_to(self.unit);
        self.value.eq(&comp.value)
    }
}

impl<U: Unit, V: Value + Eq> Eq for Quantity<U, V> where
    Quantity<U, V>: PartialEq,
{}
//endregion

//region Ordering.
impl<U: Unit, V: Value, W: Unit, X: Value> PartialOrd<Quantity<W, X>>
for Quantity<U, V> where
    W: ConvertInto<U>,
    V: PartialOrd<X>,
{
    fn partial_cmp(&self, other: &Quantity<W, X>) -> Option<std::cmp::Ordering> {
        let comp = other.clone().convert_to(self.unit);
        self.value.partial_cmp(&comp.value)
    }
}

impl<U: Unit, V: Value + Ord> Ord for Quantity<U, V> where
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
impl<U: Unit, V: Value + Signed> Quantity<U, V> {
    /// Return the absolute value of this quantity.
    pub fn abs(self) -> Self {
        Self::new(self.unit, self.value.abs())
    }
}

impl<U: Unit, V: Value + Real> Quantity<U, V> {
    /// Cast the value to another type through the [`NumCast`] trait.
    pub fn value_cast<X: Value>(self) -> Option<Quantity<U, X>> {
        Some(Quantity::new(self.unit, NumCast::from(self.value)?))
    }

    /// Return the smallest quantity with an integer value greater than or equal
    ///     to this one.
    pub fn ceil(self) -> Self {
        Self::new(self.unit, self.value.ceil())
    }

    /// Return the largest quantity with an integer value less than or equal to
    ///     this one.
    pub fn floor(self) -> Self {
        Self::new(self.unit, self.value.floor())
    }

    /// Return the largest quantity with the integer value nearest to this one.
    pub fn round(self) -> Self {
        Self::new(self.unit, self.value.round())
    }

    /// Return a quantity with only the integer part of the value of this one.
    pub fn trunc(self) -> Self {
        Self::new(self.unit, self.value.trunc())
    }
}

// impl<U: Unit, V: Scalar> num_traits:: for Quantity<U, V> {}

impl<U: Unit, V: Value> Inv for Quantity<U, V> where
    U: Inv, <U as Inv>::Output: Unit,
    V: Inv, <V as Inv>::Output: Value,
{
    type Output = Quantity<<U as Inv>::Output, <V as Inv>::Output>;

    fn inv(self) -> Self::Output {
        Quantity::new(self.unit.inv(), self.value.inv())
    }
}


impl<U: Unit, V: Value> Zero for Quantity<U, V> {
    fn zero() -> Self {
        U::base().zero()
    }

    fn is_zero(&self) -> bool {
        self.value.is_zero()
    }
}


impl<U, V, UMul, VMul, UAdd, VAdd> MulAdd<
    Quantity<UMul, VMul>,
    Quantity<UAdd, VAdd>,
> for Quantity<U, V> where
    U: Unit, UMul: Unit, UAdd: Unit,
    V: Value, VMul: Value, VAdd: Value,
    U: Mul<UMul>, <U as Mul<UMul>>::Output: Unit<Dim=UAdd::Dim>,
    V: MulAdd<VMul, VAdd>, <V as MulAdd<VMul, VAdd>>::Output: Value,
{
    type Output = Quantity<
        <U as Mul<UMul>>::Output,
        <V as MulAdd<VMul, VAdd>>::Output,
    >;

    fn mul_add(
        self,
        qty_mul: Quantity<UMul, VMul>,
        qty_add: Quantity<UAdd, VAdd>,
    ) -> Self::Output {
        let u_out = self.unit * qty_mul.unit;

        let v_mul = qty_mul.value;
        let v_add = qty_add.convert_to(u_out).value;
        let v_out = self.value.mul_add(v_mul, v_add);

        u_out.quantity(v_out)
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
    impl<U: Unit, V: Value> $fmt for Quantity<U, V> where V: $fmt {
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
