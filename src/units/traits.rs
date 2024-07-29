use crate::{dimension::DimType, Quantity, units::unit_anon::UnitAnon, Value};

pub mod transform;
pub use transform::*;


/// Trait for a type that represents a dimensional "unit".
pub trait Unit: Copy + Default + std::fmt::Display + PartialEq {
    type Dim: DimType;
    // type ScaleType: crate::Scalar;

    fn scale(&self) -> f64;

    /// Given a unit of the same dimension, return the multiplication factor
    ///     needed to scale from this unit to the other unit.
    fn scale_factor<U: Unit<Dim=Self::Dim>>(self, target: U) -> f64 {
        let have = self.scale();
        let want = target.scale();

        have / want
    }

    /// Given a unit of the same dimension, return the multiplication factor
    ///     needed to scale from this unit to the other unit, converted to a
    ///     [`Value`] type.
    fn scale_factor_v<U: Unit<Dim=Self::Dim>, V: Value>(self, target: U) -> Option<V> {
        V::from_f64(self.scale_factor(target))
    }

    /// Return the base unit of this type, with a scale of 1.
    fn base() -> Self { Default::default() }

    fn anonymous(&self) -> UnitAnon<Self::Dim> {
        UnitAnon::new(self.scale())
    }

    fn dimension(&self) -> Self::Dim { DimType::dimension() }

    fn quantity<V: Value>(self, value: V) -> Quantity<Self, V> {
        Quantity::new(self, value)
    }

    fn one<V: Value>(self) -> Quantity<Self, V> {
        self.quantity(num_traits::One::one())
    }

    fn zero<V: Value>(self) -> Quantity<Self, V> {
        self.quantity(num_traits::Zero::zero())
    }

    #[cfg(feature = "rand")]
    fn random<V, R>(self, rng: &mut R) -> Quantity<Self, V> where
        rand::distributions::Standard: rand::prelude::Distribution<V>,
        R: rand::Rng,
        V: Value,
    {
        self.random_in(rng, rand::distributions::Standard)
    }

    #[cfg(feature = "rand")]
    fn random_in<V, R, D>(self, rng: &mut R, dist: D) -> Quantity<Self, V> where
        D: rand::prelude::Distribution<V>,
        R: rand::Rng,
        V: Value,
    {
        self.quantity(dist.sample(rng))
    }

    #[cfg(feature = "simd")]
    fn quantity_simd<V, const N: usize, S>(self, values: [V; N])
        -> crate::simd::QtySimd<Self, V, N, S> where
        std::simd::LaneCount<N>: std::simd::SupportedLaneCount,
        V: crate::simd::QtySimdValue,
        S: crate::simd::QtySimdScale,
    {
        crate::simd::QtySimd::from_scales(
            values,
            [S::from_f64(self.scale()).unwrap(); N],
        )
    }

    fn convert_from<U, V>(self, qty: Quantity<U, V>) -> Quantity<Self, V> where
        U: ConvertInto<Self>,
        V: Value,
    {
        qty.convert_to(self)
    }
}


pub trait UnitCompound: Unit {}

/// A "concrete" unit is irreducible, and typically corresponds directly to a
///     physical property. It typically has multiple variants with different
///     scales.
pub trait UnitConcrete: Unit + UnitStep {
    /// The SI base unit has a scale of 1.
    const BASE: Self;

    /// Return a textual representation of this unit. Usually a base symbol with
    ///     an optional SI scaling prefix.
    fn symbol(&self) -> &'static str;
}

/// Concrete units by definition have an exponent of 1.
impl<U: UnitConcrete> UnitNonExp for U {}


/// This trait defines the ability to "step" a [`Unit`] type up and down. This
///     allows a [`Quantity`] to automatically [normalize](Quantity::normalize)
///     its value.
pub trait UnitStep: Unit {
    /// Return the next unit down in the scale, or `None` if this is already the
    ///     smallest variant.
    //  NOTE: It is an error for this method to return the same unit as `self`,
    //      or to form a loop.
    fn step_down(&self) -> Option<Self>;

    /// Return the next unit up in the scale, or `None` if this is already the
    ///     largest variant.
    //  NOTE: It is an error for this method to return the same unit as `self`,
    //      or to form a loop.
    fn step_up(&self) -> Option<Self>;

    /// Find the smallest unit in the scale by repeated stepping.
    fn step_to_bottom(&self) -> Self {
        let mut unit = *self;
        while let Some(next) = unit.step_down() { unit = next; }
        unit
    }

    /// Find the largest unit in the scale by repeated stepping.
    fn step_to_top(&self) -> Self {
        let mut unit = *self;
        while let Some(next) = unit.step_up() { unit = next; }
        unit
    }
}


//region Exponential traits.
pub trait UnitExp: Unit {}
pub trait UnitNonExp: Unit {}

//region Positive exponents.
/// Trait for a type that can be raised to a power.
pub trait CanPow<const E: i32>: Unit {
    type Output: Unit;
    fn pow(self) -> Self::Output;
}

/// Trait for a type that can be raised to the second power.
pub trait CanSquare: Unit {
    type Output: Unit;
    fn squared(self) -> Self::Output;
}

impl<U: CanPow<2>> CanSquare for U {
    type Output = U::Output;
    fn squared(self) -> Self::Output { self.pow() }
}

/// Trait for a type that can be raised to the third power.
pub trait CanCube: Unit {
    type Output: Unit;
    fn cubed(self) -> Self::Output;
}

impl<U: CanPow<3>> CanCube for U {
    type Output = U::Output;
    fn cubed(self) -> Self::Output { self.pow() }
}
//endregion

//region Roots.
pub trait CanRoot<const D: i32>: Unit {
    type Output: Unit;
    fn root(self) -> Self::Output;
}

pub trait CanSquareRoot: Unit {
    type Output: Unit;
    fn sqrt(self) -> Self::Output;
}

impl<U: CanRoot<2>> CanSquareRoot for U {
    type Output = U::Output;
    fn sqrt(self) -> Self::Output { self.root() }
}

pub trait CanCubeRoot: Unit {
    type Output: Unit;
    fn cbrt(self) -> Self::Output;
}

impl<U: CanRoot<3>> CanCubeRoot for U {
    type Output = U::Output;
    fn cbrt(self) -> Self::Output { self.root() }
}
//endregion
//endregion


/// A compound Unit type with two sides.
pub trait UnitBinary: UnitCompound {
    type Lhs: Unit;
    type Rhs: Unit;

    fn lhs(&self) -> Self::Lhs;
    fn rhs(&self) -> Self::Rhs;

    fn binary(lhs: Self::Lhs, rhs: Self::Rhs) -> Self;

    fn modify_lhs<F, L, V>(&self, f: F) -> V where
        F: FnOnce(Self::Lhs) -> L,
        V: UnitBinary<Lhs=L, Rhs=Self::Rhs>,
    {
        V::binary(f(self.lhs()), self.rhs())
    }

    fn modify_rhs<F, R, V>(&self, f: F) -> V where
        F: FnOnce(Self::Rhs) -> R,
        V: UnitBinary<Lhs=Self::Lhs, Rhs=R>,
    {
        V::binary(self.lhs(), f(self.rhs()))
    }

    fn step_lhs_down(&self) -> Option<Self> where Self::Lhs: UnitStep {
        Some(Self::binary(self.lhs().step_down()?, self.rhs()))
    }

    fn step_lhs_up(&self) -> Option<Self> where Self::Lhs: UnitStep {
        Some(Self::binary(self.lhs().step_up()?, self.rhs()))
    }

    fn step_rhs_down(&self) -> Option<Self> where Self::Rhs: UnitStep {
        Some(Self::binary(self.lhs(), self.rhs().step_down()?))
    }

    fn step_rhs_up(&self) -> Option<Self> where Self::Rhs: UnitStep {
        Some(Self::binary(self.lhs(), self.rhs().step_up()?))
    }
}
