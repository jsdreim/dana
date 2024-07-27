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

    fn one<V: Scalar + num_traits::One>(self) -> Quantity<Self, V> {
        self.quantity(num_traits::One::one())
    }

    fn zero<V: Scalar + num_traits::Zero>(self) -> Quantity<Self, V> {
        self.quantity(num_traits::Zero::zero())
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
pub trait UnitConcrete: Unit + UnitScale {
    /// The SI base unit has a scale of 1.
    const BASE: Self;

    /// Return a textual representation of this unit. Usually a base symbol with
    ///     an optional SI scaling prefix.
    fn symbol(&self) -> &'static str;
}

/// Concrete units by definition have an exponent of 1.
impl<U: UnitConcrete> UnitNonExp for U {}


/// This trait allows a [`Unit`] type to define a scale for its variants, which
///     can be stepped up and down. This allows a [`Quantity`] to automatically
///     [normalize](Quantity::normalize) its value.
//  TODO: Find a word to distinguish between scale₁ (the scaling coefficient of
//      a unit) and scale₂ (the range between the variants with the smallest
//      scale₁ and the largest scale₁).
pub trait UnitScale: Unit {
    // const SMALLEST: Self;
    // const LARGEST: Self;

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
