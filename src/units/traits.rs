use crate::{Quantity, Scalar};

pub mod associative;
pub mod commutative;
pub mod transform;

pub use associative::*;
pub use commutative::*;
pub use transform::*;


pub trait Unit: Copy + Default + std::fmt::Display + PartialEq {
    // type ScaleType: crate::Scalar;

    fn scale(&self) -> f64;

    /// Given another unit of the same type, return the multiplication factor
    ///     needed to scale from this unit to the other unit.
    fn scale_factor(self, target: Self) -> f64 {
        let have = self.scale();
        let want = target.scale();

        have / want
    }

    fn quantity<V: Scalar>(self, value: V) -> Quantity<Self, V> {
        Quantity::new(self, value)
    }

    fn default_quantity<V: Scalar>(value: V) -> Quantity<Self, V> {
        Self::default().quantity(value)
    }

    fn convert_from<U, V>(self, qty: Quantity<U, V>) -> Quantity<Self, V> where
        U: ConvertInto<Self>,
        V: Scalar + 'static,
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

/// Any concrete unit can be freely converted to another of its own type.
impl<U: UnitConcrete> ConvertFrom<U> for U {}


pub trait UnitScale: Unit {
    fn next_down(&self) -> Option<Self>;
    fn next_up(&self) -> Option<Self>;
}


//region Exponential traits.
pub trait UnitExp: Unit {}
pub trait UnitNonExp: Unit {}

//region Positive exponents.
/// Trait for a type that can be raised to a power.
pub trait CanPow<E>: Unit {
    type Output: Unit;
    fn pow(self) -> Self::Output;
}

/// Trait for a type that can be raised to the second power.
pub trait CanSquare: Unit {
    type Output: Unit;
    fn squared(self) -> Self::Output;
}

impl<U: CanPow<super::exp::E2>> CanSquare for U {
    type Output = U::Output;
    fn squared(self) -> Self::Output { self.pow() }
}

/// Trait for a type that can be raised to the third power.
pub trait CanCube: Unit {
    type Output: Unit;
    fn cubed(self) -> Self::Output;
}

impl<U: CanPow<super::exp::E3>> CanCube for U {
    type Output = U::Output;
    fn cubed(self) -> Self::Output { self.pow() }
}
//endregion

//region Roots.
pub trait CanRoot<E>: Unit {
    type Output: Unit;
    fn root(self) -> Self::Output;
}

pub trait CanSquareRoot: Unit {
    type Output: Unit;
    fn sqrt(self) -> Self::Output;
}

impl<U: CanRoot<super::exp::E2>> CanSquareRoot for U {
    type Output = U::Output;
    fn sqrt(self) -> Self::Output { self.root() }
}

pub trait CanCubeRoot: Unit {
    type Output: Unit;
    fn cbrt(self) -> Self::Output;
}

impl<U: CanRoot<super::exp::E3>> CanCubeRoot for U {
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
