#[macro_export]
macro_rules! utype {
    //  Internal: Exponents.
    // (@@ $e:tt) => { $crate::units::exp::TypeFrac<$e, 1> };
    (@@ $e:tt) => { <$crate::units::dim::ExpHack<$e> as $crate::units::dim::HasTypenum>::Typenum };

    //  Internal: Unpack a group directly.
    ((@ $($t:tt)+)) => { $($t)+ };

    //  Unpack braces.
    (($($t:tt)+)) => { $crate::utype!($($t)+) };
    ({$($t:tt)+}) => { $crate::utype!($($t)+) };

    //  Pass through a single path.
    ($p:path) => { $p };

    //  Exponents.
    ($a:tt $(::$b:tt)* ^ 0)     => { compile_error!("Cannot define unit of power zero.")      };
    ($a:tt $(::$b:tt)* ^ 1)     => {                             $crate::utype!($a $(::$b)*)  };
    ($a:tt $(::$b:tt)* ^ $e:tt) => {
        $crate::units::UnitPow<$crate::utype!($a $(::$b)*), $crate::utype!(@@ $e)>
    };
    //  Signed exponents.
    ($a:tt $(::$b:tt)* ^-$e:tt) => { $crate::units::PerUnit     <$crate::utype!($a $(::$b)* ^ $e)> };
    ($a:tt $(::$b:tt)* ^+$e:tt) => {                             $crate::utype!($a $(::$b)* ^ $e)  };

    //  Inverted term.
    (1 / $a:tt $(::$b:tt)* ^ $e:tt $($z:tt)*) => { $crate::unit!(
        (@ $crate::units::PerUnit<$crate::utype!($a $(::$b)* ^ $e)>) $($z)*
    )};
    (1 / $a:tt $(::$b:tt)* $($z:tt)*) => { $crate::unit!(
        (@ $crate::units::PerUnit<$crate::utype!($a $(::$b)*)>) $($z)*
    )};

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

    //  Pass through a single path.
    ($p:path) => { $p };

    //  Exponents.
    ($a:tt $(::$b:tt)* ^ 0)     => { compile_error!("Cannot define unit of power zero.")     };
    ($a:tt $(::$b:tt)* ^ 1)     => {                             $crate::unit!($a $(::$b)*)  };
    ($a:tt $(::$b:tt)* ^ $e:tt) => {
        $crate::units::UnitPow::<_, $crate::utype!(@@ $e)>::new($crate::unit!($a $(::$b)*))
    };
    //  Signed exponents.
    ($a:tt $(::$b:tt)* ^-$e:tt) => { $crate::units::PerUnit     ($crate::unit!($a $(::$b)* ^ $e)) };
    ($a:tt $(::$b:tt)* ^+$e:tt) => {                             $crate::unit!($a $(::$b)* ^ $e)  };

    //  Inverted term.
    (1 / $a:tt $(::$b:tt)* ^ $e:tt $($z:tt)*) => { $crate::unit!(
        (@ $crate::units::PerUnit($crate::unit!($a $(::$b)* ^ $e))) $($z)*
    )};
    (1 / $a:tt $(::$b:tt)* $($z:tt)*) => { $crate::unit!(
        (@ $crate::units::PerUnit($crate::unit!($a $(::$b)*))) $($z)*
    )};

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
    (<$ty:ty> $($t:tt)*) => { $crate::Quantity<$crate::utype!($($t)*), $ty> };
    (         $($t:tt)*) => { $crate::Quantity<$crate::utype!($($t)*)     > };
}


macro_rules! impl_unit_concrete {
    ($unit:ident) => {
        impl ::std::fmt::Display for $unit {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                <str as std::fmt::Display>::fmt(self.symbol(), f)
            }
        }

        impl Default for $unit {
            fn default() -> Self { Self::BASE }
        }

        //  TODO
        // ::static_assertions::const_assert!($unit::BASE.scale() == 1.0);
    };
    ($($unit:ident),+$(,)?) => {
        $(impl_unit_concrete!($unit);)+
    };
}
