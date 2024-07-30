//! Module for traits describing aspects of units.

use crate::{dimension::DimType, Quantity, units::UnitAnon, Value};

pub mod transform;
pub use transform::*;


/// Trait for a type that represents a dimensional "unit".
pub trait Unit: Copy + Default + std::fmt::Display + PartialEq {
    /// The [`Dimension`](crate::dimension::Dimension) represented by units of
    ///     this type.
    type Dim: DimType;
    // type ScaleType: crate::Scalar;

    /// Return the scale of this unit, relative to the base unit of this type.
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

    /// Return an [anonymous unit](UnitAnon) with the same dimension and scale
    ///     as this one.
    fn anonymous(&self) -> UnitAnon<Self::Dim> {
        UnitAnon::new(self.scale())
    }

    /// Return a runtime representation of the dimension of this unit.
    fn dimension(&self) -> Self::Dim { DimType::dimension() }

    /// Return a [`Quantity`] with this unit and the given value.
    fn quantity<V: Value>(self, value: V) -> Quantity<Self, V> {
        Quantity::new(self, value)
    }

    /// Return a [`Quantity`] with this unit and a value of one.
    fn one<V: Value>(self) -> Quantity<Self, V> {
        self.quantity(num_traits::One::one())
    }

    /// Return a [`Quantity`] with this unit and a value of zero.
    fn zero<V: Value>(self) -> Quantity<Self, V> {
        self.quantity(num_traits::Zero::zero())
    }

    /// Return a [`Quantity`] with this unit and a random value.
    #[cfg(feature = "rand")]
    fn random<V, R>(self, rng: &mut R) -> Quantity<Self, V> where
        rand::distributions::Standard: rand::prelude::Distribution<V>,
        R: rand::Rng,
        V: Value,
    {
        self.random_in(rng, rand::distributions::Standard)
    }

    /// Return a [`Quantity`] with this unit and a random value, using the
    ///     random distribution provided..
    #[cfg(feature = "rand")]
    fn random_in<V, R, D>(self, rng: &mut R, dist: D) -> Quantity<Self, V> where
        D: rand::prelude::Distribution<V>,
        R: rand::Rng,
        V: Value,
    {
        self.quantity(dist.sample(rng))
    }

    /// Return a [`QtySimd`](crate::simd::QtySimd) array, populated from the
    ///     array of values, with this unit.
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
}


/// A "compound" unit is defined in terms of other units, and represents a
///     mathematical relationship between them.
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
//region Whole exponents.
/// Trait for a type that can be raised to a power.
pub trait CanPow<const E: i32>: Unit {
    /// The result of the operation.
    type Output: Unit;

    /// Raise this unit to the power of the const parameter `E`.
    fn pow(self) -> Self::Output;
}

/// Trait for a type that can be raised to the second power.
pub trait CanSquare: Unit {
    /// The result of the operation.
    type Output: Unit;

    /// Return the square of this unit.
    fn squared(self) -> Self::Output;
}

impl<U: CanPow<2>> CanSquare for U {
    type Output = U::Output;
    fn squared(self) -> Self::Output { self.pow() }
}

/// Trait for a type that can be raised to the third power.
pub trait CanCube: Unit {
    /// The result of the operation.
    type Output: Unit;

    /// Return the cube of this unit.
    fn cubed(self) -> Self::Output;
}

impl<U: CanPow<3>> CanCube for U {
    type Output = U::Output;
    fn cubed(self) -> Self::Output { self.pow() }
}
//endregion

//region Roots.
/// Trait for a type that can be taken to an exponential root.
pub trait CanRoot<const D: i32>: Unit {
    /// The result of the operation.
    type Output: Unit;

    /// Take the root of this unit to the degree of the const parameter `D`.
    fn root(self) -> Self::Output;
}

/// Trait for a type that can be taken to the second root.
pub trait CanSquareRoot: Unit {
    /// The result of the operation.
    type Output: Unit;

    /// Return the square root of this unit.
    fn sqrt(self) -> Self::Output;
}

impl<U: CanRoot<2>> CanSquareRoot for U {
    type Output = U::Output;
    fn sqrt(self) -> Self::Output { self.root() }
}

/// Trait for a type that can be taken to the third root.
pub trait CanCubeRoot: Unit {
    /// The result of the operation.
    type Output: Unit;

    /// Return the cube root of this unit.
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
    /// The unit on the left-hand side of the binary operator.
    type Lhs: Unit;
    /// The unit on the right-hand side of the binary operator.
    type Rhs: Unit;

    /// Construct a new binary unit around the inputs.
    fn binary(lhs: Self::Lhs, rhs: Self::Rhs) -> Self;

    /// Return the left-hand unit.
    fn lhs(&self) -> Self::Lhs;
    /// Return the right-hand unit.
    fn rhs(&self) -> Self::Rhs;

    /// Return a new binary unit, based on this one, where a given function has
    ///     been run on the left-hand unit.
    fn modify_lhs<F, L, V>(&self, f: F) -> V where
        F: FnOnce(Self::Lhs) -> L,
        V: UnitBinary<Lhs=L, Rhs=Self::Rhs>,
    {
        V::binary(f(self.lhs()), self.rhs())
    }

    /// Return a new binary unit, based on this one, where a given function has
    ///     been run on the right-hand unit.
    fn modify_rhs<F, R, V>(&self, f: F) -> V where
        F: FnOnce(Self::Rhs) -> R,
        V: UnitBinary<Lhs=Self::Lhs, Rhs=R>,
    {
        V::binary(self.lhs(), f(self.rhs()))
    }

    /// Return a version of this unit with the left-hand unit stepped down
    ///     according to [`UnitStep::step_down`], or `None` if it cannot be.
    fn step_lhs_down(&self) -> Option<Self> where Self::Lhs: UnitStep {
        Some(Self::binary(self.lhs().step_down()?, self.rhs()))
    }

    /// Return a version of this unit with the left-hand unit stepped up
    ///     according to [`UnitStep::step_up`], or `None` if it cannot be.
    fn step_lhs_up(&self) -> Option<Self> where Self::Lhs: UnitStep {
        Some(Self::binary(self.lhs().step_up()?, self.rhs()))
    }

    /// Return a version of this unit with the right-hand unit stepped down
    ///     according to [`UnitStep::step_down`], or `None` if it cannot be.
    fn step_rhs_down(&self) -> Option<Self> where Self::Rhs: UnitStep {
        Some(Self::binary(self.lhs(), self.rhs().step_down()?))
    }

    /// Return a version of this unit with the right-hand unit stepped up
    ///     according to [`UnitStep::step_up`], or `None` if it cannot be.
    fn step_rhs_up(&self) -> Option<Self> where Self::Rhs: UnitStep {
        Some(Self::binary(self.lhs(), self.rhs().step_up()?))
    }
}
