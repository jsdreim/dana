use std::{marker::PhantomData, ops::{Add, Div, Mul, Sub}};
use std::ops::Neg;
use num_traits::Inv;
use typenum::{consts::{N1, N2, N3, P1, P2, P3, P4, Z0}, Integer, NonZero, PartialDiv};


/// Integer type used for dimension exponents.
pub type Exp = i32;


pub type None         = Dimension<Z0, Z0, Z0, Z0, Z0, Z0, Z0>;
//                                 L   M   T   I   Θ   N   J
pub type Length       = Dimension<P1, Z0, Z0, Z0, Z0, Z0, Z0>;
pub type Mass         = Dimension<Z0, P1, Z0, Z0, Z0, Z0, Z0>;
pub type Time         = Dimension<Z0, Z0, P1, Z0, Z0, Z0, Z0>;
pub type Current      = Dimension<Z0, Z0, Z0, P1, Z0, Z0, Z0>;
pub type Temp         = Dimension<Z0, Z0, Z0, Z0, P1, Z0, Z0>;
pub type Amount       = Dimension<Z0, Z0, Z0, Z0, Z0, P1, Z0>;
pub type Lum          = Dimension<Z0, Z0, Z0, Z0, Z0, Z0, P1>;
//                                 L   M   T   I   Θ   N   J
pub type Frequency    = Dimension<Z0, Z0, N1, Z0, Z0, Z0, Z0>;
pub type Velocity     = Dimension<P1, Z0, N1, Z0, Z0, Z0, Z0>;
pub type Accel        = Dimension<P1, Z0, N2, Z0, Z0, Z0, Z0>;
pub type Force        = Dimension<P1, P1, N2, Z0, Z0, Z0, Z0>;
pub type Pressure     = Dimension<N1, P1, N2, Z0, Z0, Z0, Z0>;
pub type Area         = Dimension<P2, Z0, Z0, Z0, Z0, Z0, Z0>;
pub type Volume       = Dimension<P3, Z0, Z0, Z0, Z0, Z0, Z0>;
pub type Density      = Dimension<N3, P1, Z0, Z0, Z0, Z0, Z0>;
//                                 L   M   T   I   Θ   N   J
pub type Charge       = Dimension<Z0, Z0, P1, P1, Z0, Z0, Z0>;
pub type Torque       = Dimension<P2, P1, N2, Z0, Z0, Z0, Z0>;
pub type Energy       = Dimension<P2, P1, N2, Z0, Z0, Z0, Z0>;
pub type Power        = Dimension<P2, P1, N3, Z0, Z0, Z0, Z0>;
pub type Voltage      = Dimension<P2, P1, N3, N1, Z0, Z0, Z0>;
pub type Resistance   = Dimension<P2, P1, N3, N2, Z0, Z0, Z0>;
pub type Capacitance  = Dimension<N2, N1, P4, P2, Z0, Z0, Z0>;
//                                 L   M   T   I   Θ   N   J


pub mod symbols {
    pub type L = super::Length;
    pub type M = super::Mass;
    pub type T = super::Time;
    pub type I = super::Current;
    pub type Θ = super::Temp;
    pub type N = super::Amount;
    pub type J = super::Lum;
}


#[derive(Clone, Copy, Debug, Default)]
pub struct Dimension<
    L: Integer, M: Integer, T: Integer,
    I: Integer, Θ: Integer, N: Integer,
    J: Integer,
> {
    _l: PhantomData<L>, _m: PhantomData<M>, _t: PhantomData<T>,
    _i: PhantomData<I>, _θ: PhantomData<Θ>, _n: PhantomData<N>,
    _j: PhantomData<J>,
}


//  TODO: Seal this trait.
pub trait DimType: Copy + Default + std::fmt::Display {
    type ExpLen: Integer;
    type ExpMass: Integer;
    type ExpTime: Integer;
    type ExpCurr: Integer;
    type ExpTemp: Integer;
    type ExpAmt: Integer;
    type ExpLum: Integer;

    const EXP_LEN:  Exp = <Self::ExpLen as Integer>::I32;
    const EXP_MASS: Exp = <Self::ExpMass as Integer>::I32;
    const EXP_TIME: Exp = <Self::ExpTime as Integer>::I32;
    const EXP_CURR: Exp = <Self::ExpCurr as Integer>::I32;
    const EXP_TEMP: Exp = <Self::ExpTemp as Integer>::I32;
    const EXP_AMT:  Exp = <Self::ExpAmt as Integer>::I32;
    const EXP_LUM:  Exp = <Self::ExpLum as Integer>::I32;

    const ARRAY: [(char, Exp); 7] = [
        ('L', <Self::ExpLen as Integer>::I32),
        ('M', <Self::ExpMass as Integer>::I32),
        ('T', <Self::ExpTime as Integer>::I32),
        ('I', <Self::ExpCurr as Integer>::I32),
        ('Θ', <Self::ExpTemp as Integer>::I32),
        ('N', <Self::ExpAmt as Integer>::I32),
        ('J', <Self::ExpLum as Integer>::I32),
    ];

    // const ARRAY: [(char, Exp); 7] = [
    //     ('L', Self::EXP_LEN),  ('M', Self::EXP_MASS), ('T', Self::EXP_TIME),
    //     ('I', Self::EXP_CURR), ('Θ', Self::EXP_TEMP), ('N', Self::EXP_AMT),
    //     ('J', Self::EXP_LUM),
    // ];

    fn dim() -> Self { Self::default() }
}

impl<
    L: Integer, M: Integer, T: Integer,
    I: Integer, Θ: Integer, N: Integer,
    J: Integer,
> DimType for Dimension<L, M, T, I, Θ, N, J> where
    // L::Output: Integer, M::Output: Integer, T::Output: Integer,
    // I::Output: Integer, Θ::Output: Integer, N::Output: Integer,
    // J::Output: Integer,
{
    type ExpLen = L;
    type ExpMass = M;
    type ExpTime = T;
    type ExpCurr = I;
    type ExpTemp = Θ;
    type ExpAmt = N;
    type ExpLum = J;
}


impl<
    L: Integer, M: Integer, T: Integer,
    I: Integer, Θ: Integer, N: Integer,
    J: Integer,
> std::fmt::Display for Dimension<L, M, T, I, Θ, N, J> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Write;

        let mut any = false;

        for (char, exp) in Self::ARRAY {
            if exp != 0 {
                if any {
                    f.write_char('*')?;
                } else {
                    any = true;
                }

                f.write_char(char)?;

                if exp != 1 {
                    write!(f, "^{}", exp)?;
                }
            }
        }

        Ok(())
    }
}


macro_rules! impl_dim {
    (
        impl unary $trait_dim:ident::$out_dim:ident
        $(fn $func:ident($($bind:tt)*))?
        as $trait_int:ident::$out_int:ident
        $(where $($t:tt)*)?
    ) => {
        impl<
            L: Integer + $trait_int, M: Integer + $trait_int, T: Integer + $trait_int,
            I: Integer + $trait_int, Θ: Integer + $trait_int, N: Integer + $trait_int,
            J: Integer + $trait_int,
        > $trait_dim for Dimension<L, M, T, I, Θ, N, J> where
            <L as $trait_int>::$out_int: Integer,
            <M as $trait_int>::$out_int: Integer,
            <T as $trait_int>::$out_int: Integer,
            <I as $trait_int>::$out_int: Integer,
            <Θ as $trait_int>::$out_int: Integer,
            <N as $trait_int>::$out_int: Integer,
            <J as $trait_int>::$out_int: Integer,
            $($($t)*)?
        {
            type $out_dim = Dimension<
                <L as $trait_int>::$out_int,
                <M as $trait_int>::$out_int,
                <T as $trait_int>::$out_int,
                <I as $trait_int>::$out_int,
                <Θ as $trait_int>::$out_int,
                <N as $trait_int>::$out_int,
                <J as $trait_int>::$out_int,
            >;

            $(fn $func($($bind)*) -> Self::Output {
                Default::default()
            })?
        }
    };
    (
        impl binary $trait_dim:ident::$out_dim:ident
        $(fn $func:ident($($bind:tt)*))?
        as $trait_int:ident::$out_int:ident
        $(where $($t:tt)*)?
    ) => {
        impl<
            L1: Integer + $trait_int<L2>, M1: Integer + $trait_int<M2>, T1: Integer + $trait_int<T2>,
            I1: Integer + $trait_int<I2>, Θ1: Integer + $trait_int<Θ2>, N1: Integer + $trait_int<N2>,
            J1: Integer + $trait_int<J2>,
            L2: Integer, M2: Integer, T2: Integer,
            I2: Integer, Θ2: Integer, N2: Integer,
            J2: Integer,
        > $trait_dim<Dimension<L2, M2, T2, I2, Θ2, N2, J2>>
        for Dimension<L1, M1, T1, I1, Θ1, N1, J1> where
            <L1 as $trait_int<L2>>::$out_int: Integer,
            <M1 as $trait_int<M2>>::$out_int: Integer,
            <T1 as $trait_int<T2>>::$out_int: Integer,
            <I1 as $trait_int<I2>>::$out_int: Integer,
            <Θ1 as $trait_int<Θ2>>::$out_int: Integer,
            <N1 as $trait_int<N2>>::$out_int: Integer,
            <J1 as $trait_int<J2>>::$out_int: Integer,
            $($($t)*)?
        {
            type $out_dim = Dimension<
                <L1 as $trait_int<L2>>::$out_int,
                <M1 as $trait_int<M2>>::$out_int,
                <T1 as $trait_int<T2>>::$out_int,
                <I1 as $trait_int<I2>>::$out_int,
                <Θ1 as $trait_int<Θ2>>::$out_int,
                <N1 as $trait_int<N2>>::$out_int,
                <J1 as $trait_int<J2>>::$out_int,
            >;

            $(fn $func($($bind)*, _: Dimension<L2, M2, T2, I2, Θ2, N2, J2>) -> Self::Output {
                Default::default()
            })?
        }
    };
}


/// Division.
pub trait DimDiv<D: DimType>: DimType {
    type Output: DimType;
}


/// Multiplication.
pub trait DimMul<D: DimType>: DimType {
    type Output: DimType;
}

impl_dim!(impl binary Div::Output fn div(self) as Sub::Output);
impl_dim!(impl binary Mul::Output fn mul(self) as Add::Output);

impl_dim!(impl binary DimDiv::Output as Sub::Output);
impl_dim!(impl binary DimMul::Output as Add::Output);


/// Inversion.
pub trait DimInv: DimType {
    type Output: DimType;
}

impl<D: DimType> DimInv for D where
    D::ExpLen: Neg, <D::ExpLen as Neg>::Output: Integer,
    D::ExpMass: Neg, <D::ExpMass as Neg>::Output: Integer,
    D::ExpTime: Neg, <D::ExpTime as Neg>::Output: Integer,
    D::ExpCurr: Neg, <D::ExpCurr as Neg>::Output: Integer,
    D::ExpTemp: Neg, <D::ExpTemp as Neg>::Output: Integer,
    D::ExpAmt: Neg, <D::ExpAmt as Neg>::Output: Integer,
    D::ExpLum: Neg, <D::ExpLum as Neg>::Output: Integer,
{
    type Output = Dimension<
        <D::ExpLen as Neg>::Output,
        <D::ExpMass as Neg>::Output,
        <D::ExpTime as Neg>::Output,
        <D::ExpCurr as Neg>::Output,
        <D::ExpTemp as Neg>::Output,
        <D::ExpAmt as Neg>::Output,
        <D::ExpLum as Neg>::Output,
    >;
}

impl<
    L: Integer + Neg, M: Integer + Neg, T: Integer + Neg,
    I: Integer + Neg, Θ: Integer + Neg, N: Integer + Neg,
    J: Integer + Neg,
> Inv for Dimension<L, M, T, I, Θ, N, J> where
    L::Output: Integer, M::Output: Integer, T::Output: Integer,
    I::Output: Integer, Θ::Output: Integer, N::Output: Integer,
    J::Output: Integer,
{
    type Output = Dimension<
        L::Output, M::Output, T::Output,
        I::Output, Θ::Output, N::Output,
        J::Output,
    >;

    fn inv(self) -> Self::Output { Default::default() }
}


/// Integer powers.
pub trait DimPowType<E: Integer>: DimType {
    type Output: DimType;
}

impl<
    L: Integer + Mul<E>, M: Integer + Mul<E>, T: Integer + Mul<E>,
    I: Integer + Mul<E>, Θ: Integer + Mul<E>, N: Integer + Mul<E>,
    J: Integer + Mul<E>,
    E: Integer,
> DimPowType<E> for Dimension<L, M, T, I, Θ, N, J> where
    L::Output: Integer, M::Output: Integer, T::Output: Integer,
    I::Output: Integer, Θ::Output: Integer, N::Output: Integer,
    J::Output: Integer,
{
    type Output = Dimension<
        L::Output, M::Output, T::Output,
        I::Output, Θ::Output, N::Output,
        J::Output,
    >;
}


/// Fractional powers.
pub trait DimRootType<D: Integer + NonZero>: DimType {
    type Output: DimType;
}

impl<
    L: Integer + PartialDiv<D>, M: Integer + PartialDiv<D>, T: Integer + PartialDiv<D>,
    I: Integer + PartialDiv<D>, Θ: Integer + PartialDiv<D>, N: Integer + PartialDiv<D>,
    J: Integer + PartialDiv<D>,
    D: Integer + NonZero,
> DimRootType<D> for Dimension<L, M, T, I, Θ, N, J> where
    L::Output: Integer, M::Output: Integer, T::Output: Integer,
    I::Output: Integer, Θ::Output: Integer, N::Output: Integer,
    J::Output: Integer,
{
    type Output = Dimension<
        L::Output, M::Output, T::Output,
        I::Output, Θ::Output, N::Output,
        J::Output,
    >;
}


pub struct ExpHack<const E: Exp>;
pub trait HasTypenum {
    type Typenum: Integer;
}

impl_typenums!();


pub trait DimPow<const E: Exp>: DimType {
    type Output: DimType;
}

impl<Dim: DimType, const E: Exp> DimPow<E> for Dim where
    ExpHack<E>: HasTypenum,
    Dim: DimPowType<<ExpHack<E> as HasTypenum>::Typenum>,
{
    type Output = Dim::Output;
}


pub trait DimRoot<const D: Exp>: DimType {
    type Output: DimType;
}

impl<Dim: DimType, const D: Exp> DimRoot<D> for Dim where
    ExpHack<D>: HasTypenum,
    <ExpHack<D> as HasTypenum>::Typenum: NonZero,
    Dim: DimRootType<<ExpHack<D> as HasTypenum>::Typenum>,
{
    type Output = Dim::Output;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dimensions() {
        assert_eq!(format!("{}", Length::dim()), "L");
        assert_eq!(format!("{}", Velocity::dim()), "L*T^-1");
        assert_eq!(format!("{}", Accel::dim()), "L*T^-2");

        let _a: Accel = Velocity::dim() / Time::dim();
        let _a: Accel = Velocity::dim() * Time::dim().inv();
        let _a: Length = Velocity::dim() * Time::dim();
    }
}
