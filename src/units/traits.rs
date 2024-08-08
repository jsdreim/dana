//! Module for traits describing aspects of units.

use crate::{
    dimension::DimType,
    Quantity,
    units::{unit_anon::UnitAnon, unit_rescale::{Rescale, UnitRescale}},
    Value,
};

mod exponential;
pub use exponential::*;

pub mod transform;
pub use transform::*;

mod unit_step;
pub use unit_step::*;

mod unit_binary;
pub use unit_binary::*;


/// Trait for a type that represents a dimensional "unit".
pub trait Unit: Copy + Default + core::fmt::Debug + core::fmt::Display + PartialEq {
    /// The [`Dimension`](crate::dimension::Dimension) represented by units of
    ///     this type.
    type Dim: DimType;
    // type ScaleType: crate::Scalar;

    //region Unit scale methods.
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
    //endregion

    /// Return the base unit of this type, with a scale of 1.
    fn base() -> Self { Default::default() }

    /// Return an [anonymous unit](UnitAnon) with the same dimension and scale
    ///     as this one.
    fn anonymous(&self) -> UnitAnon<Self::Dim> {
        UnitAnon::new(self.scale())
    }

    /// Return a [`UnitRescale`] of this unit.
    fn rescale<S: Rescale>(self, factor: S) -> UnitRescale<Self, S> {
        UnitRescale::new(self, factor)
    }

    /// Return a runtime representation of the dimension of this unit.
    fn dimension(&self) -> Self::Dim { DimType::dimension() }

    //region Quantity creation.
    //region With `self` as unit.
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

    /// Return a given [`Quantity`], converted to this unit.
    fn convert_from<U, V>(self, qty: Quantity<U, V>) -> Quantity<Self, V> where
        U: Unit<Dim=Self::Dim>,
        V: Value,
    {
        qty.convert_to(self)
    }
    //endregion

    //region With base unit.
    //  NOTE: These are provided, rather than simply recommending use of the
    //      more explicit `<_>::base().*()`, because that breaks inference when
    //      called from the trait. Using `Unit::base_*()` allows rustc to infer
    //      the output type.

    /// Return a [`Quantity`] with the base of this unit type and the given value.
    fn base_qty<V: Value>(value: V) -> Quantity<Self, V> {
        Self::base().quantity(value)
    }

    /// Return a given [`Quantity`], converted to the base of this unit type.
    fn base_from<U, V>(qty: Quantity<U, V>) -> Quantity<Self, V> where
        U: Unit<Dim=Self::Dim>,
        V: Value,
    {
        qty.convert_to(Self::base())
    }
    //endregion
    //endregion

    //region Feature-gated methods.
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
        core::simd::LaneCount<N>: core::simd::SupportedLaneCount,
        V: crate::simd::QtySimdValue,
        S: crate::simd::QtySimdScale,
    {
        crate::simd::QtySimd::from_scales(
            values,
            [crate::_conv_f64(self.scale()); N],
        )
    }
    //endregion
}


/// A mixed unit type has variants that are SI units and variants that are not.
pub trait UnitMixed: Unit {
    /// Return the SI unit nearest to this one.
    fn to_si(&self) -> Self;

    /// Return `true` if this unit is an SI unit.
    fn is_si(&self) -> bool {
        self.to_si().eq(self)
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
