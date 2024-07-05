#[macro_export]
macro_rules! utype {
    //  Unpack braces.
    (($($t:tt)+)) => { $crate::utype!($($t)+) };
    ({$($t:tt)+}) => { $crate::utype!($($t)+) };

    //  Pass through a single path.
    ($p:path) => { $p };

    //  Exponents.
    ($a:tt $(::$b:tt)* ^ -1)    => { $crate::units::PerUnit     <$crate::utype!($a $(::$b)*)> };
    ($a:tt $(::$b:tt)* ^ 2)     => { $crate::units::UnitSquared <$crate::utype!($a $(::$b)*)> };
    ($a:tt $(::$b:tt)* ^ 3)     => { $crate::units::UnitCubed   <$crate::utype!($a $(::$b)*)> };
    ($a:tt $(::$b:tt)* ^ $e:tt) => { $crate::units::UnitPow     <$crate::utype!($a $(::$b)*)> };

    //  Simpler operations.
    ($a:tt $(::$b:tt)* $(^ $e:tt)? / $($z:tt)*) => {
        $crate::units::UnitDiv<$crate::utype!($a $(::$b)* $(^ $e)?), $crate::utype!($($z)*)>
    };
    ($a:tt $(::$b:tt)* $(^ $e:tt)? * $($z:tt)*) => {
        $crate::units::UnitMul<$crate::utype!($a $(::$b)* $(^ $e)?), $crate::utype!($($z)*)>
    };
}

#[macro_export]
macro_rules! unit {
    //  Unpack braces.
    (($($t:tt)+)) => { $crate::unit!($($t)+) };
    ({$($t:tt)+}) => { $crate::unit!($($t)+) };

    //  Pass through a single token or path.
    ($u:tt) => { $crate::unit_from_symbol!($u) };
    ($p:path) => { $p };

    //  Exponents.
    ($a:tt $(::$b:tt)* ^ -1)    => { $crate::units::PerUnit     ($crate::unit!($a $(::$b)*)) };
    ($a:tt $(::$b:tt)* ^ 2)     => { $crate::units::UnitSquared ($crate::unit!($a $(::$b)*)) };
    ($a:tt $(::$b:tt)* ^ 3)     => { $crate::units::UnitCubed   ($crate::unit!($a $(::$b)*)) };
    ($a:tt $(::$b:tt)* ^ $e:tt) => { $crate::units::UnitPow     ($crate::unit!($a $(::$b)*), $e) };

    //  Simpler operations.
    ($a:tt $(::$b:tt)* $(^ $e:tt)? / $($z:tt)*) => {
        $crate::units::UnitDiv($crate::unit!($a $(::$b)* $(^ $e)?), $crate::unit!($($z)*))
    };
    ($a:tt $(::$b:tt)* $(^ $e:tt)? * $($z:tt)*) => {
        $crate::units::UnitMul($crate::unit!($a $(::$b)* $(^ $e)?), $crate::unit!($($z)*))
    };
}

#[macro_export]
macro_rules! qtype {
    ($ty:ty: $($t:tt)*) => {$crate::Quantity<$crate::utype!($($t)*), $ty>};
    ($($t:tt)*) => {$crate::Quantity<$crate::utype!($($t)*)>};
}

#[macro_export]
macro_rules! quantity {
    ($val:tt $($t:tt)*) => {
        $crate::Quantity {
            value: $val,
            unit: $crate::unit!($($t)*),
        }
    };
}


macro_rules! impl_unit_concrete {
    ($unit:ident) => {
        impl ::std::fmt::Display for $unit {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                <str as std::fmt::Display>::fmt(self.symbol(), f)
            }
        }
    };
    ($($unit:ident),+$(,)?) => {
        $(impl_unit_concrete!($unit);)+
    };
}


macro_rules! impl_unit_ops {
    ($unit:ident $(<$($tv:ident: $t0:ident $(+ $t1:ident)*),+>)?) => {
        /*/// Unit multiplication by scalar.
        impl<_V: $crate::Scalar $($(, $tv: $t0 $(+ $t1)*)+)?> ::std::ops::Mul<_V>
        for $unit$(<$($tv),+>)?
        {
            type Output = $crate::Quantity<$unit$(<$($tv),+>)?, _V>;

            fn mul(self, rhs: _V) -> Self::Output {
                Self::Output::new(self, rhs)
            }
        }*/

        /// Unit division.
        impl<_U: $crate::Unit $($(, $tv: $t0 $(+ $t1)*)+)?> ::std::ops::Div<_U>
        for $unit$(<$($tv),+>)?
        {
            type Output = $crate::units::UnitDiv<Self, _U>;

            fn div(self, rhs: _U) -> Self::Output {
                $crate::units::UnitDiv(self, rhs)
            }
        }

        /// Unit multiplication.
        impl<_U: $crate::Unit $($(, $tv: $t0 $(+ $t1)*)+)?> ::std::ops::Mul<_U>
        for $unit$(<$($tv),+>)?
        {
            type Output = $crate::units::UnitMul<Self, _U>;

            fn mul(self, rhs: _U) -> Self::Output {
                $crate::units::UnitMul(self, rhs)
            }
        }
    };
    ($($unit:ident $(<$($tv:ident: $t0:ident $(+ $t1:ident)*),+>)?),+$(,)?) => {
        $(impl_unit_ops!($unit $(<$($tv: $t0 $(+ $t1)*),+>)?);)+
    };
}

macro_rules! impl_unit_pow {
    ($unit:ident $(<$($tv:ident: $t0:ident $(+ $t1:ident)*),+>)?) => {
        impl$(<$($tv: $t0 $(+ $t1)*),+>)?
        $crate::units::traits::CanSquare
        for $unit$(<$($tv),+>)?
        {
            type Output = $crate::units::UnitSquared<Self>;
            fn squared(self) -> Self::Output { Self::Output::new(self) }
        }

        impl$(<$($tv: $t0 $(+ $t1)*),+>)?
        $crate::units::traits::CanCube
        for $unit$(<$($tv),+>)?
        {
            type Output = $crate::units::UnitCubed<Self>;
            fn cubed(self) -> Self::Output { Self::Output::new(self) }
        }
    };
    ($($unit:ident $(<$($tv:ident: $t0:ident $(+ $t1:ident)*),+>)?),+$(,)?) => {
        $(impl_unit_pow!($unit $(<$($tv: $t0 $(+ $t1)*),+>)?);)+
    };
}

macro_rules! impl_unit_pow_n {
    ($unit:ident $(<$($tv:ident: $t0:ident $(+ $t1:ident)*),+>)?) => {
        impl<_P: $crate::units::compound::unit_pow_n::Exp $($(, $tv: $t0 $(+ $t1)*)+)?>
        $crate::units::traits::CanPow<_P>
        for $unit$(<$($tv),+>)? where
            f64: ::num_traits::Pow<_P, Output=f64>,
        {
            type Output = $crate::units::UnitPow<Self, _P>;
            fn pow(self, exp: _P) -> Self::Output { Self::Output::new(self, exp) }
        }
    };
    ($($unit:ident $(<$($tv:ident: $t0:ident $(+ $t1:ident)*),+>)?),+$(,)?) => {
        $(impl_unit_pow_n!($unit $(<$($tv: $t0 $(+ $t1)*),+>)?);)+
    };
}
