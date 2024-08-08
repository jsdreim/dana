use super::*;


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
