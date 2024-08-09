use super::*;


/// This trait defines the ability to "step" a [`Unit`] type up and down. This
///     allows a [`Quantity`] to automatically [normalize](Quantity::normalize)
///     its value.
pub trait UnitStep: Unit {
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
