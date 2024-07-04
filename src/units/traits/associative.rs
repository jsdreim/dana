use super::{UnitBinary, UnitCompound};


/// For a function `f` where it is applicable, the Associative Property states
///     that `f(f(a, b), c)` is equivalent to `f(a, f(b, c))`. This trait allows
///     a compound unit type to switch between these forms.
pub trait Associative<U>: UnitCompound {
    fn reassociate(self) -> U;
}


/// A binary unit type where the left side is [`Associative`].
pub trait AssociativeLeft<U>: UnitBinary where Self::Left: Associative<U> {
    type WithLeftReassociated: UnitBinary<Left=U, Right=Self::Right>;

    fn reassociate_left(&self) -> Self::WithLeftReassociated {
        self.modify_left(|u| u.reassociate())
    }
}


/// A binary unit type where the right side is [`Associative`].
pub trait AssociativeRight<U>: UnitBinary where Self::Right: Associative<U> {
    type WithRightReassociated: UnitBinary<Left=Self::Left, Right=U>;

    fn reassociate_right(&self) -> Self::WithRightReassociated {
        self.modify_right(|u| u.reassociate())
    }
}
