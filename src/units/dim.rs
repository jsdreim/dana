use const_assert::*;


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


//  TODO: Seal this trait.
pub trait IsDim: std::fmt::Display {
    const EXP_LEN: Exp;
    const EXP_MASS: Exp;
    const EXP_TIME: Exp;
    const EXP_CURR: Exp;
    const EXP_TEMP: Exp;
    const EXP_AMT: Exp;
    const EXP_LUM: Exp;

    const ARRAY: [(char, Exp); 7] = [
        ('L', Self::EXP_LEN),  ('M', Self::EXP_MASS), ('T', Self::EXP_TIME),
        ('I', Self::EXP_CURR), ('Θ', Self::EXP_TEMP), ('N', Self::EXP_AMT),
        ('J', Self::EXP_LUM),
    ];

    fn dim() -> Self;
}

impl<
    const L: Exp, const M: Exp, const T: Exp,
    const I: Exp, const Θ: Exp, const N: Exp,
    const J: Exp,
> IsDim for Dim<L, M, T, I, Θ, N, J> {
    const EXP_LEN: Exp = L;
    const EXP_MASS: Exp = M;
    const EXP_TIME: Exp = T;
    const EXP_CURR: Exp = I;
    const EXP_TEMP: Exp = Θ;
    const EXP_AMT: Exp = N;
    const EXP_LUM: Exp = J;

    fn dim() -> Self { Self }
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


macro_rules! dim_op {
    (1: $dim:ident $($t:tt)*) => {Dim<
        {<$dim as IsDim>::EXP_LEN  $($t)*},
        {<$dim as IsDim>::EXP_MASS $($t)*},
        {<$dim as IsDim>::EXP_TIME $($t)*},
        {<$dim as IsDim>::EXP_CURR $($t)*},
        {<$dim as IsDim>::EXP_TEMP $($t)*},
        {<$dim as IsDim>::EXP_AMT  $($t)*},
        {<$dim as IsDim>::EXP_LUM  $($t)*},
    >};
    (2: $dim1:ident $op:tt $dim2:ident) => {Dim<
        {<$dim1 as IsDim>::EXP_LEN  $op <$dim2 as IsDim>::EXP_LEN  },
        {<$dim1 as IsDim>::EXP_MASS $op <$dim2 as IsDim>::EXP_MASS },
        {<$dim1 as IsDim>::EXP_TIME $op <$dim2 as IsDim>::EXP_TIME },
        {<$dim1 as IsDim>::EXP_CURR $op <$dim2 as IsDim>::EXP_CURR },
        {<$dim1 as IsDim>::EXP_TEMP $op <$dim2 as IsDim>::EXP_TEMP },
        {<$dim1 as IsDim>::EXP_AMT  $op <$dim2 as IsDim>::EXP_AMT  },
        {<$dim1 as IsDim>::EXP_LUM  $op <$dim2 as IsDim>::EXP_LUM  },
    >};
}


/// Division.
pub trait DimDiv<D: IsDim>: IsDim {
    type Output;
}

impl<D1: IsDim, D2: IsDim> DimDiv<D2> for D1 where
    dim_op!(2: D1 / D2):
{
    type Output = dim_op!(2: D1 / D2);
}


/// Multiplication.
pub trait DimMul<D: IsDim>: IsDim {
    type Output;
}

impl<D1: IsDim, D2: IsDim> DimMul<D2> for D1 where
    dim_op!(2: D1 * D2):
{
    type Output = dim_op!(2: D1 * D2);
}


/// Inversion.
pub trait DimInv: IsDim {
    type Output;
}

impl<D: IsDim> DimInv for D where
    dim_op!(1: D * -1):
{
    type Output = dim_op!(1: D * -1);
}


/// Integer powers.
pub trait DimPow<const E: Exp>: IsDim {
    type Output;
}

impl<const E: Exp, D: IsDim> DimPow<E> for D where
    dim_op!(1: D * E):
{
    type Output = dim_op!(1: D * E);
}


/// Fractional powers.
pub trait DimRoot<const E: Exp>: IsDim {
    type Output;
}

impl<const E: Exp, D: IsDim> DimRoot<E> for D where
    Assert<{ <D as IsDim>::EXP_LEN  % E == 0 }>: IsTrue,
    Assert<{ <D as IsDim>::EXP_MASS % E == 0 }>: IsTrue,
    Assert<{ <D as IsDim>::EXP_TIME % E == 0 }>: IsTrue,
    Assert<{ <D as IsDim>::EXP_CURR % E == 0 }>: IsTrue,
    Assert<{ <D as IsDim>::EXP_TEMP % E == 0 }>: IsTrue,
    Assert<{ <D as IsDim>::EXP_AMT  % E == 0 }>: IsTrue,
    Assert<{ <D as IsDim>::EXP_LUM  % E == 0 }>: IsTrue,
    Assert<{                          E != 0 }>: IsTrue,
    dim_op!(1: D / E):
{
    type Output = dim_op!(1: D / E);
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
