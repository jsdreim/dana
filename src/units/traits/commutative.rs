use super::UnitBinary;


/// For a function `f` where it is applicable, the Commutative Property states
///     that `f(a, b)` is equivalent to `f(b, a)`. This trait allows a compound
///     unit type to switch between these forms.
pub trait Commutative: UnitBinary {
    type Commuted: UnitBinary<Left=Self::Right, Right=Self::Left>;

    fn commute(&self) -> Self::Commuted {
        UnitBinary::new(self.right(), self.left())
    }
}


/// A binary unit type where the left side is [`Commutative`].
pub trait CommutativeLeft: UnitBinary where Self::Left: Commutative {
    type WithLeftCommuted: UnitBinary<
        Left=<Self::Left as Commutative>::Commuted,
        Right=Self::Right,
    >;

    fn commute_left(&self) -> Self::WithLeftCommuted {
        self.modify_left(|u| u.commute())
    }
}


/// A binary unit type where the right side is [`Commutative`].
pub trait CommutativeRight: UnitBinary where Self::Right: Commutative {
    type WithRightCommuted: UnitBinary<
        Left=Self::Left,
        Right=<Self::Right as Commutative>::Commuted,
    >;

    fn commute_right(&self) -> Self::WithRightCommuted {
        self.modify_right(|u| u.commute())
    }
}
