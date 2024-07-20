use std::ops::{Div, Mul};
use const_assert::*;
use num_traits::Inv;


/// Integer type used for dimension exponents.
pub type Exp = i32;


pub type None         = Dim< 0, 0, 0, 0, 0, 0, 0 >;
//                           L  M  T  I  Θ  N  J
pub type Length       = Dim< 1, 0, 0, 0, 0, 0, 0 >;
pub type Mass         = Dim< 0, 1, 0, 0, 0, 0, 0 >;
pub type Time         = Dim< 0, 0, 1, 0, 0, 0, 0 >;
pub type Current      = Dim< 0, 0, 0, 1, 0, 0, 0 >;
pub type Temp         = Dim< 0, 0, 0, 0, 1, 0, 0 >;
pub type Amount       = Dim< 0, 0, 0, 0, 0, 1, 0 >;
pub type Lum          = Dim< 0, 0, 0, 0, 0, 0, 1 >;
//                           L  M  T  I  Θ  N  J
pub type Frequency    = Dim< 0, 0,-1, 0, 0, 0, 0 >;
pub type Velocity     = Dim< 1, 0,-1, 0, 0, 0, 0 >;
pub type Accel        = Dim< 1, 0,-2, 0, 0, 0, 0 >;
pub type Force        = Dim< 1, 1,-2, 0, 0, 0, 0 >;
pub type Pressure     = Dim<-1, 1,-2, 0, 0, 0, 0 >;
pub type Area         = Dim< 2, 0, 0, 0, 0, 0, 0 >;
pub type Volume       = Dim< 3, 0, 0, 0, 0, 0, 0 >;
pub type Density      = Dim<-3, 1, 0, 0, 0, 0, 0 >;
//                           L  M  T  I  Θ  N  J
pub type Charge       = Dim< 0, 0, 1, 1, 0, 0, 0 >;
pub type Torque       = Dim< 2, 1,-2, 0, 0, 0, 0 >;
pub type Energy       = Dim< 2, 1,-2, 0, 0, 0, 0 >;
pub type Power        = Dim< 2, 1,-3, 0, 0, 0, 0 >;
pub type Voltage      = Dim< 2, 1,-3,-1, 0, 0, 0 >;
pub type Resistance   = Dim< 2, 1,-3,-2, 0, 0, 0 >;
pub type Capacitance  = Dim<-2,-1, 4, 2, 0, 0, 0 >;
//                           L  M  T  I  Θ  N  J


pub mod symbols {
    pub type L = super::Length;
    pub type M = super::Mass;
    pub type T = super::Time;
    pub type I = super::Current;
    pub type Θ = super::Temp;
    pub type N = super::Amount;
    pub type J = super::Lum;
}


#[derive(Clone, Copy, Debug)]
pub struct Dim<
    const L: Exp, const M: Exp, const T: Exp,
    const I: Exp, const Θ: Exp, const N: Exp,
    const J: Exp,
>;

impl<
    const L: Exp, const M: Exp, const T: Exp,
    const I: Exp, const Θ: Exp, const N: Exp,
    const J: Exp,
> Dim<L, M, T, I, Θ, N, J> {
    pub const EXP_LEN: Exp = L;
    pub const EXP_MASS: Exp = M;
    pub const EXP_TIME: Exp = T;
    pub const EXP_CURR: Exp = I;
    pub const EXP_TEMP: Exp = Θ;
    pub const EXP_AMT: Exp = N;
    pub const EXP_LUM: Exp = J;

    pub const ARRAY: [(char, Exp); 7] = [
        ('L', L), ('M', M), ('T', T),
        ('I', I), ('Θ', Θ), ('N', N),
        ('J', J),
    ];

    pub const fn dim() -> Self { Self }

    pub fn powi<const E: Exp>() {}
}


impl<
    const L: Exp, const M: Exp, const T: Exp,
    const I: Exp, const Θ: Exp, const N: Exp,
    const J: Exp,
> std::fmt::Display for Dim<L, M, T, I, Θ, N, J> {
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

impl<
    const L1: Exp, const M1: Exp, const T1: Exp,
    const I1: Exp, const Θ1: Exp, const N1: Exp,
    const J1: Exp,
    const L2: Exp, const M2: Exp, const T2: Exp,
    const I2: Exp, const Θ2: Exp, const N2: Exp,
    const J2: Exp,
> Div<Dim<L2, M2, T2, I2, Θ2, N2, J2>> for Dim<L1, M1, T1, I1, Θ1, N1, J1> where
    Dim<{L1-L2}, {M1-M2}, {T1-T2}, {I1-I2}, {Θ1-Θ2}, {N1-N2}, {J1-J2}>:
{
    type Output = Dim<
        { L1 - L2 }, { M1 - M2 }, { T1 - T2 },
        { I1 - I2 }, { Θ1 - Θ2 }, { N1 - N2 },
        { J1 - J2 },
    >;

    fn div(self, _rhs: Dim<L2, M2, T2, I2, Θ2, N2, J2>) -> Self::Output { Dim }
}

impl<
    const L1: Exp, const M1: Exp, const T1: Exp,
    const I1: Exp, const Θ1: Exp, const N1: Exp,
    const J1: Exp,
    const L2: Exp, const M2: Exp, const T2: Exp,
    const I2: Exp, const Θ2: Exp, const N2: Exp,
    const J2: Exp,
> Mul<Dim<L2, M2, T2, I2, Θ2, N2, J2>> for Dim<L1, M1, T1, I1, Θ1, N1, J1> where
    Dim<{L1+L2}, {M1+M2}, {T1+T2}, {I1+I2}, {Θ1+Θ2}, {N1+N2}, {J1+J2}>:
{
    type Output = Dim<
        { L1 + L2 }, { M1 + M2 }, { T1 + T2 },
        { I1 + I2 }, { Θ1 + Θ2 }, { N1 + N2 },
        { J1 + J2 },
    >;

    fn mul(self, _rhs: Dim<L2, M2, T2, I2, Θ2, N2, J2>) -> Self::Output { Dim }
}

impl<
    const L: Exp, const M: Exp, const T: Exp,
    const I: Exp, const Θ: Exp, const N: Exp,
    const J: Exp,
> Inv for Dim<L, M, T, I, Θ, N, J> where
    Dim<{-L}, {-M}, {-T}, {-I}, {-Θ}, {-N}, {-J}>:
{
    type Output = Dim<{-L}, {-M}, {-T}, {-I}, {-Θ}, {-N}, {-J}>;
    fn inv(self) -> Self::Output { Dim }
}


pub trait DimPow<const E: Exp> {
    type Output;
}

impl<
    const L: Exp, const M: Exp, const T: Exp,
    const I: Exp, const Θ: Exp, const N: Exp,
    const J: Exp,
    const E: Exp,
> DimPow<E> for Dim<L, M, T, I, Θ, N, J> where
    Dim<{L*E}, {M*E}, {T*E}, {I*E}, {Θ*E}, {N*E}, {J*E}>:
{
    type Output = Dim<
        { L * E }, { M * E }, { T * E },
        { I * E }, { Θ * E }, { N * E },
        { J * E },
    >;
}


pub trait DimRoot<const E: Exp> {
    type Output;
}

impl<
    const L: Exp, const M: Exp, const T: Exp,
    const I: Exp, const Θ: Exp, const N: Exp,
    const J: Exp,
    const D: Exp,
> DimRoot<D> for Dim<L, M, T, I, Θ, N, J> where
    Assert<{ L % D == 0 }>: IsTrue, Assert<{ M % D == 0 }>: IsTrue, Assert<{ T % D == 0 }>: IsTrue,
    Assert<{ I % D == 0 }>: IsTrue, Assert<{ Θ % D == 0 }>: IsTrue, Assert<{ N % D == 0 }>: IsTrue,
    Assert<{ J % D == 0 }>: IsTrue,
    Assert<{     D != 0 }>: IsTrue,
    Dim<{L/D}, {M/D}, {T/D}, {I/D}, {Θ/D}, {N/D}, {J/D}>:
{
    type Output = Dim<
        { L / D }, { M / D }, { T / D },
        { I / D }, { Θ / D }, { N / D },
        { J / D },
    >;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dimensions() {
        assert_eq!(format!("{}", Length::dim()), "L");
        assert_eq!(format!("{}", Velocity::dim()), "L*T^-1");
        assert_eq!(format!("{}", Accel::dim()), "L*T^-2");
    }
}
