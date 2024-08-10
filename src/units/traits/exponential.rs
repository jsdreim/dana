use super::*;


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
