#[macro_export]
macro_rules! utype {
    //  Internal: Unpack a group directly.
    ((@ $($t:tt)+)) => { $($t)+ };

    //  Unpack braces.
    (($($t:tt)+)) => { $crate::utype!($($t)+) };
    ({$($t:tt)+}) => { $crate::utype!($($t)+) };

    //  Try to interpret a single token as a unit type symbol.
    ($u:tt) => { $crate::utype_from_symbol!($u) };
    //  Pass through a single path.
    ($p:path) => { $p };

    //  Exponents.
    ($a:tt $(::$b:tt)* ^ -1)    => { $crate::units::PerUnit     <$crate::utype!($a $(::$b)*)> };
    ($a:tt $(::$b:tt)* ^ 2)     => { $crate::units::UnitSquared <$crate::utype!($a $(::$b)*)> };
    ($a:tt $(::$b:tt)* ^ 3)     => { $crate::units::UnitCubed   <$crate::utype!($a $(::$b)*)> };
    ($a:tt $(::$b:tt)* ^ $e:tt) => { $crate::units::UnitPow     <$crate::utype!($a $(::$b)*)> };

    //  Div/mul where the second unit has an exponent.
    (
        $a1:tt $(::$b1:tt)* $(^ $e1:tt)? /
        $a2:tt $(::$b2:tt)*   ^ $e2:tt   $($z:tt)*
    ) => { $crate::utype!(
        (@ $crate::units::UnitDiv<
            $crate::utype!($a1 $(::$b1)* $(^ $e1)?),
            $crate::utype!($a2 $(::$b2)* ^ $e2),
        >)
        $($z)*
    )};
    (
        $a1:tt $(::$b1:tt)* $(^ $e1:tt)? *
        $a2:tt $(::$b2:tt)*   ^ $e2:tt   $($z:tt)*
    ) => { $crate::utype!(
        (@ $crate::units::UnitMul<
            $crate::utype!($a1 $(::$b1)* $(^ $e1)?),
            $crate::utype!($a2 $(::$b2)* ^ $e2),
        >)
        $($z)*
    )};

    //  Div/mul where the second unit has no exponent.
    (
        $a1:tt $(::$b1:tt)* $(^ $e1:tt)? /
        $a2:tt $(::$b2:tt)*              $($z:tt)*
    ) => { $crate::utype!(
        (@ $crate::units::UnitDiv<
            $crate::utype!($a1 $(::$b1)* $(^ $e1)?),
            $crate::utype!($a2 $(::$b2)*),
        >)
        $($z)*
    )};
    (
        $a1:tt $(::$b1:tt)* $(^ $e1:tt)? *
        $a2:tt $(::$b2:tt)*              $($z:tt)*
    ) => { $crate::utype!(
        (@ $crate::units::UnitMul<
            $crate::utype!($a1 $(::$b1)* $(^ $e1)?),
            $crate::utype!($a2 $(::$b2)*),
        >)
        $($z)*
    )};
}

#[macro_export]
macro_rules! unit {
    //  Internal: Unpack a group directly.
    ((@ $($t:tt)+)) => { $($t)+ };

    //  Unpack and interpret a group.
    (($($t:tt)+)) => { $crate::unit!($($t)+) };
    ({$($t:tt)+}) => { $crate::unit!($($t)+) };

    //  Try to interpret a single token as a unit symbol.
    ($u:tt) => { $crate::unit_from_symbol!($u) };
    //  Pass through a single path.
    ($p:path) => { $p };

    //  Exponents.
    ($a:tt $(::$b:tt)* ^ -1)    => { $crate::units::PerUnit     ($crate::unit!($a $(::$b)*)) };
    ($a:tt $(::$b:tt)* ^ 2)     => { $crate::units::UnitSquared ($crate::unit!($a $(::$b)*)) };
    ($a:tt $(::$b:tt)* ^ 3)     => { $crate::units::UnitCubed   ($crate::unit!($a $(::$b)*)) };
    ($a:tt $(::$b:tt)* ^ $e:tt) => { $crate::units::UnitPow     ($crate::unit!($a $(::$b)*), $e) };

    //  Div/mul where the second unit has an exponent.
    (
        $a1:tt $(::$b1:tt)* $(^ $e1:tt)? /
        $a2:tt $(::$b2:tt)*   ^ $e2:tt   $($z:tt)*
    ) => { $crate::unit!(
        (@ $crate::units::UnitDiv(
            $crate::unit!($a1 $(::$b1)* $(^ $e1)?),
            $crate::unit!($a2 $(::$b2)* ^ $e2),
        ))
        $($z)*
    )};
    (
        $a1:tt $(::$b1:tt)* $(^ $e1:tt)? *
        $a2:tt $(::$b2:tt)*   ^ $e2:tt   $($z:tt)*
    ) => { $crate::unit!(
        (@ $crate::units::UnitMul(
            $crate::unit!($a1 $(::$b1)* $(^ $e1)?),
            $crate::unit!($a2 $(::$b2)* ^ $e2),
        ))
        $($z)*
    )};

    //  Div/mul where the second unit has no exponent.
    (
        $a1:tt $(::$b1:tt)* $(^ $e1:tt)? /
        $a2:tt $(::$b2:tt)*              $($z:tt)*
    ) => { $crate::unit!(
        (@ $crate::units::UnitDiv(
            $crate::unit!($a1 $(::$b1)* $(^ $e1)?),
            $crate::unit!($a2 $(::$b2)*),
        ))
        $($z)*
    )};
    (
        $a1:tt $(::$b1:tt)* $(^ $e1:tt)? *
        $a2:tt $(::$b2:tt)*              $($z:tt)*
    ) => { $crate::unit!(
        (@ $crate::units::UnitMul(
            $crate::unit!($a1 $(::$b1)* $(^ $e1)?),
            $crate::unit!($a2 $(::$b2)*),
        ))
        $($z)*
    )};
}


#[macro_export]
macro_rules! qtype {
    ($ty:ty: $($t:tt)*) => {$crate::Quantity<$crate::utype!($($t)*), $ty>};
    ($($t:tt)*) => {$crate::Quantity<$crate::utype!($($t)*)>};
}

#[macro_export]
macro_rules! quantity {
    ($qty:tt as $($t:tt)*) => { $qty.convert_to($crate::unit!($($t)*)) };
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
