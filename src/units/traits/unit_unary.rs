use super::*;


/// A compound Unit type that wraps a single inner unit.
pub trait UnitUnary: UnitCompound {
    /// The wrapped unit type.
    type Inner: Unit;

    /// Construct a new unary unit around the input.
    fn unary(inner: Self::Inner) -> Self;

    /// Return the wrapped unit.
    fn inner(&self) -> Self::Inner;

    /// Return a new unary unit, based on this one, where a given function has
    ///     been run on the inner unit.
    fn modify_inner<F, I, V>(&self, f: F) -> V where
        F: FnOnce(Self::Inner) -> I,
        V: UnitUnary<Inner=I>,
    {
        V::unary(f(self.inner()))
    }

    /// Return a version of this unit with the inner unit stepped down according
    ///     to [`UnitStep::step_down`], or `None` if it cannot be.
    fn step_inner_down(&self) -> Option<Self> where Self::Inner: UnitStep {
        Some(Self::unary(self.inner().step_down()?))
    }

    /// Return a version of this unit with the inner unit stepped up according
    ///     to [`UnitStep::step_up`], or `None` if it cannot be.
    fn step_inner_up(&self) -> Option<Self> where Self::Inner: UnitStep {
        Some(Self::unary(self.inner().step_up()?))
    }
}
