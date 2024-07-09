pub trait Dimension: Copy {
    const SYMBOL: char;
    type BaseUnit: Unit<Self>;
}


pub trait Unit<D: Dimension> {
    type Symbol: std::fmt::Display;
    fn symbol(&self) -> Self::Symbol;
    fn scale(&self) -> f64 { 1.0 }
}


pub struct UnitScaled<U, const PRE: char, const MUL: usize, const DIV: usize>(U);

impl<D: Dimension, U: Unit<D>, const PRE: char, const MUL: usize, const DIV: usize>
Unit<D> for UnitScaled<U, PRE, MUL, DIV> {
    type Symbol = String;

    fn symbol(&self) -> Self::Symbol {
        format!("{PRE}{}", self.0.symbol())
    }

    fn scale(&self) -> f64 {
        self.0.scale() * (MUL as f64 / DIV as f64)
    }
}

pub type Nano<U> = UnitScaled<U, 'n', 1, 1_000_000_000>;
pub type Micro<U> = UnitScaled<U, 'μ', 1, 1_000_000>;
pub type Milli<U> = UnitScaled<U, 'm', 1, 1_000>;

pub type Kilo<U> = UnitScaled<U, 'k', 1_000, 1>;
pub type Mega<U> = UnitScaled<U, 'M', 1_000_000, 1>;
pub type Giga<U> = UnitScaled<U, 'G', 1_000_000_000, 1>;
pub type Tera<U> = UnitScaled<U, 'T', 1_000_000_000_000, 1>;
pub type Peta<U> = UnitScaled<U, 'P', 1_000_000_000_000_000, 1>;



pub mod length {
    use super::*;

    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct Length;

    impl Dimension for Length {
        const SYMBOL: char = 'L';
        type BaseUnit = Meter;
    }

    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct Meter;

    impl Unit<Length> for Meter {
        type Symbol = char;
        fn symbol(&self) -> Self::Symbol { 'm' }
    }

    pub type NanoMeter = Nano<Meter>;
    pub type MicroMeter = Micro<Meter>;
    pub type MilliMeter = Milli<Meter>;
    pub type KiloMeter = Kilo<Meter>;
    pub type MegaMeter = Mega<Meter>;
    pub type GigaMeter = Giga<Meter>;
    pub type TeraMeter = Tera<Meter>;
    pub type PetaMeter = Peta<Meter>;
}
