use crate::{dimension::DimType, Quantity, Scalar, units::unit_anon::UnitAnon};

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

    fn anonymous(&self) -> UnitAnon<Self::Dim> {
        UnitAnon::new(self.scale())
    }

    fn dimension(&self) -> Self::Dim { DimType::dimension() }

    fn quantity<V: Scalar>(self, value: V) -> Quantity<Self, V> {
        Quantity::new(self, value)
    }

    #[cfg(feature = "simd")]
    fn quantity_simd<V, const N: usize, S>(self, scalars: [V; N])
        -> crate::simd::QtySimd<Self, V, N, S> where
        std::simd::LaneCount<N>: std::simd::SupportedLaneCount,
        V: crate::simd::QtySimdValue,
        S: crate::simd::QtySimdScale,
    {
        crate::simd::QtySimd::new([self; N], scalars)
    }

    fn default_quantity<V: Scalar>(value: V) -> Quantity<Self, V> {
        Self::default().quantity(value)
    }

    fn convert_from<U, V>(self, qty: Quantity<U, V>) -> Quantity<Self, V> where
        U: ConvertInto<Self>,
        V: Scalar,
    {
        qty.convert_to(self)
    }
}


pub trait UnitCompound: Unit {}

/// A "concrete" unit is irreducible, and typically corresponds directly to a
///     physical property. It typically has multiple variants with different
///     scales.
pub trait UnitConcrete: Unit {
    /// The SI base unit has a scale of 1.
    const BASE: Self;

    /// Return a textual representation of this unit. Usually a base symbol with
    ///     an optional SI scaling prefix.
    fn symbol(&self) -> &'static str;
}

/// Concrete units by definition have an exponent of 1.
impl<U: UnitConcrete> UnitNonExp for U {}

// /// Any concrete unit can be freely converted to another of its own type.
// impl<U: UnitConcrete> ConvertFrom<U> for U {}


pub trait UnitScale: Unit {
    fn next_down(&self) -> Option<Self>;
    fn next_up(&self) -> Option<Self>;
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
    type Left: Unit;
    type Right: Unit;

    fn left(&self) -> Self::Left;
    fn right(&self) -> Self::Right;

    fn new(left: Self::Left, right: Self::Right) -> Self;

    fn modify_left<F, L, V>(&self, f: F) -> V where
        F: FnOnce(Self::Left) -> L,
        V: UnitBinary<Left=L, Right=Self::Right>,
    {
        V::new(f(self.left()), self.right())
    }

    fn modify_right<F, R, V>(&self, f: F) -> V where
        F: FnOnce(Self::Right) -> R,
        V: UnitBinary<Left=Self::Left, Right=R>,
    {
        V::new(self.left(), f(self.right()))
    }
}
