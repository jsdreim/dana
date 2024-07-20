use const_assert::*;


/// Integer type used for dimension exponents.
pub type Exp = i32;


pub type None         = Dimension< 0, 0, 0, 0, 0, 0, 0 >;
//                                 L  M  T  I  Θ  N  J
pub type Length       = Dimension< 1, 0, 0, 0, 0, 0, 0 >;
pub type Mass         = Dimension< 0, 1, 0, 0, 0, 0, 0 >;
pub type Time         = Dimension< 0, 0, 1, 0, 0, 0, 0 >;
pub type Current      = Dimension< 0, 0, 0, 1, 0, 0, 0 >;
pub type Temp         = Dimension< 0, 0, 0, 0, 1, 0, 0 >;
pub type Amount       = Dimension< 0, 0, 0, 0, 0, 1, 0 >;
pub type Lum          = Dimension< 0, 0, 0, 0, 0, 0, 1 >;
//                                 L  M  T  I  Θ  N  J
pub type Frequency    = Dimension< 0, 0,-1, 0, 0, 0, 0 >;
pub type Velocity     = Dimension< 1, 0,-1, 0, 0, 0, 0 >;
pub type Accel        = Dimension< 1, 0,-2, 0, 0, 0, 0 >;
pub type Force        = Dimension< 1, 1,-2, 0, 0, 0, 0 >;
pub type Pressure     = Dimension<-1, 1,-2, 0, 0, 0, 0 >;
pub type Area         = Dimension< 2, 0, 0, 0, 0, 0, 0 >;
pub type Volume       = Dimension< 3, 0, 0, 0, 0, 0, 0 >;
pub type Density      = Dimension<-3, 1, 0, 0, 0, 0, 0 >;
//                                 L  M  T  I  Θ  N  J
pub type Charge       = Dimension< 0, 0, 1, 1, 0, 0, 0 >;
pub type Torque       = Dimension< 2, 1,-2, 0, 0, 0, 0 >;
pub type Energy       = Dimension< 2, 1,-2, 0, 0, 0, 0 >;
pub type Power        = Dimension< 2, 1,-3, 0, 0, 0, 0 >;
pub type Voltage      = Dimension< 2, 1,-3,-1, 0, 0, 0 >;
pub type Resistance   = Dimension< 2, 1,-3,-2, 0, 0, 0 >;
pub type Capacitance  = Dimension<-2,-1, 4, 2, 0, 0, 0 >;
//                                 L  M  T  I  Θ  N  J


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
pub struct Dimension<
    const L: Exp, const M: Exp, const T: Exp,
    const I: Exp, const Θ: Exp, const N: Exp,
    const J: Exp,
>;


//  TODO: Seal this trait.
pub trait DimType: std::fmt::Display {
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
> DimType for Dimension<L, M, T, I, Θ, N, J> {
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


macro_rules! dim_op {
    (1: $dim:ident $($t:tt)*) => {Dimension<
        {<$dim as DimType>::EXP_LEN  $($t)*},
        {<$dim as DimType>::EXP_MASS $($t)*},
        {<$dim as DimType>::EXP_TIME $($t)*},
        {<$dim as DimType>::EXP_CURR $($t)*},
        {<$dim as DimType>::EXP_TEMP $($t)*},
        {<$dim as DimType>::EXP_AMT  $($t)*},
        {<$dim as DimType>::EXP_LUM  $($t)*},
    >};
    (2: $dim1:ident $op:tt $dim2:ident) => {Dimension<
        {<$dim1 as DimType>::EXP_LEN  $op <$dim2 as DimType>::EXP_LEN  },
        {<$dim1 as DimType>::EXP_MASS $op <$dim2 as DimType>::EXP_MASS },
        {<$dim1 as DimType>::EXP_TIME $op <$dim2 as DimType>::EXP_TIME },
        {<$dim1 as DimType>::EXP_CURR $op <$dim2 as DimType>::EXP_CURR },
        {<$dim1 as DimType>::EXP_TEMP $op <$dim2 as DimType>::EXP_TEMP },
        {<$dim1 as DimType>::EXP_AMT  $op <$dim2 as DimType>::EXP_AMT  },
        {<$dim1 as DimType>::EXP_LUM  $op <$dim2 as DimType>::EXP_LUM  },
    >};
}


/// Division.
pub trait DimDiv<D: DimType>: DimType {
    type Output;
}

impl<D1: DimType, D2: DimType> DimDiv<D2> for D1 where
    dim_op!(2: D1 / D2):
{
    type Output = dim_op!(2: D1 / D2);
}


/// Multiplication.
pub trait DimMul<D: DimType>: DimType {
    type Output;
}

impl<D1: DimType, D2: DimType> DimMul<D2> for D1 where
    dim_op!(2: D1 * D2):
{
    type Output = dim_op!(2: D1 * D2);
}


/// Inversion.
pub trait DimInv: DimType {
    type Output;
}

impl<D: DimType> DimInv for D where
    dim_op!(1: D * -1):
{
    type Output = dim_op!(1: D * -1);
}


/// Integer powers.
pub trait DimPow<const E: Exp>: DimType {
    type Output;
}

impl<const E: Exp, D: DimType> DimPow<E> for D where
    dim_op!(1: D * E):
{
    type Output = dim_op!(1: D * E);
}


/// Fractional powers.
pub trait DimRoot<const E: Exp>: DimType {
    type Output;
}

impl<const E: Exp, D: DimType> DimRoot<E> for D where
    Assert<{ <D as DimType>::EXP_LEN  % E == 0 }>: IsTrue,
    Assert<{ <D as DimType>::EXP_MASS % E == 0 }>: IsTrue,
    Assert<{ <D as DimType>::EXP_TIME % E == 0 }>: IsTrue,
    Assert<{ <D as DimType>::EXP_CURR % E == 0 }>: IsTrue,
    Assert<{ <D as DimType>::EXP_TEMP % E == 0 }>: IsTrue,
    Assert<{ <D as DimType>::EXP_AMT  % E == 0 }>: IsTrue,
    Assert<{ <D as DimType>::EXP_LUM  % E == 0 }>: IsTrue,
    Assert<{                            E != 0 }>: IsTrue,
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
