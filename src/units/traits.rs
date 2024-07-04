use crate::Scalar;

pub mod associative;
pub mod commutative;
pub mod conversion;

pub use associative::*;
pub use commutative::*;
pub use conversion::*;


pub trait Unit: Copy + Default + PartialEq {
    // type ScaleType: crate::Scalar;

    fn scale(&self) -> f64;

    /// Given another unit of the same type, return the multiplication factor
    ///     needed to scale from this unit to the other unit.
    fn scale_factor(self, target: Self) -> f64 {
        let have = self.scale();
        let want = target.scale();

        have / want
    }

    fn quantity<V: Scalar>(self, value: V) -> crate::Quantity<Self, V> {
        crate::Quantity::new(self, value)
    }

    fn default_quantity<V: Scalar>(value: V) -> crate::Quantity<Self, V> {
        Self::default().quantity(value)
    }
}


pub trait UnitCompound: Unit {}
pub trait UnitConcrete: Unit {}


pub trait UnitNonExp: Unit {}


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
