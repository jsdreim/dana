use crate::{Scalar, units::{*, traits::*}};


macro_rules! impl_simplify {
    //region Internal: Automatic rearrangement (single implementation).
    (@
        $(where $($tvar:ident $(: $req:ident $(+ $reqs:ident)*)?),+;)?
        impl $from:tt => $target:tt;
    ) => {
        impl$(<$($tvar $(: $req $(+ $reqs)*)?),+>)?
        Simplify<$crate::utype!($target)>
        for $crate::utype!($from)
        {
            fn simplify<__S: $crate::scalar::Scalar>(self)
                -> $crate::units::traits::Conversion<$crate::utype!($target), __S>
            {
                #[allow(non_snake_case)]
                let unit_pat!($from): $crate::utype!($from) = self;
                Conversion::units(unit_pat!($target))
            }
        }
    };
    //endregion
    //region Direct form: Automatic rearrangement with no effect on value.
    (
        $(where $($tvar:ident $(: $req:ident $(+ $reqs:ident)*)?),+;)?
        impl $from:tt -> $target:tt;
    ) => {
        impl_simplify!(@
            $(where $($tvar$(:$req$(+$reqs)*)?),+;)?
            impl $from => $target;
        );
    };
    //endregion
    //region Two-way direct form: Automatic rearrangement with no effect on value.
    (
        $(where $($tvar:ident $(: $req:ident $(+ $reqs:ident)*)?),+;)?
        impl $from:tt <-> $target:tt;
    ) => {
        impl_simplify!(@
            $(where $($tvar$(:$req$(+$reqs)*)?),+;)?
            impl $from => $target;
        );
        impl_simplify!(@
            $(where $($tvar$(:$req$(+$reqs)*)?),+;)?
            impl $target => $from;
        );
    };
    //endregion
    //region Simple form: Rearrangement of bindings with no effect on value.
    (
        $(where $($tvar:ident $(: $req:ident $(+ $reqs:ident)*)?),+;)?

        for $from:tt -> $target:tt
        use $bind:tt => $output:tt
    ) => {
        impl$(<$($tvar $(: $req $(+ $reqs)*)?),+>)?
        Simplify<$crate::utype!($target)>
        for $crate::utype!($from)
        {
            fn simplify<__S: $crate::scalar::Scalar>(self)
                -> $crate::units::traits::Conversion<$crate::utype!($target), __S>
            {
                let unit_pat!($bind): $crate::utype!($from) = self;
                Conversion::units(unit_pat!($output))
            }
        }
    };
    //endregion
    //region Function form: Allow arbitrary code to produce `Conversion`.
    (
        // $(where $($tvar:ident $(: $req:ident $(+ $reqs:ident)*)?),+;)?
        $(where $($tvar:ident $(:
        $req:ident$([$($p_req:tt)*])?
        $(+ $reqs:ident$([$($p_reqs:tt)*])?)*
        )?),+;)?

        for $from:tt -> $target:tt
        use fn($sig:ident)
        // {$($code:tt)*}
        $code:block
    ) => {
        impl$(<$($tvar $(:
        $req$(<$($p_req)*>)?
        $(+ $reqs$(<$($p_reqs)*>)?)*
        )?),+>)? Simplify<$crate::utype!($target)>
        for $crate::utype!($from)
        {
            fn simplify<__S: $crate::scalar::Scalar>($sig)
                -> $crate::units::traits::Conversion<$crate::utype!($target), __S>
            {
                // $($code)*
                let (unit, float) = $code;
                let factor = __S::from_f64(float).unwrap();
                Conversion::scale(unit, factor)
            }
        }
    };
    //endregion
    //region Bound function form: Arbitrary code with variables bound to a pattern.
    (
        // $(where $($tvar:ident $(: $req:ident $(+ $reqs:ident)*)?),+;)?
        $(where $($tvar:ident $(:
        $req:ident$([$($p_req:tt)*])?
        $(+ $reqs:ident$([$($p_reqs:tt)*])?)*
        )?),+;)?

        for $from:tt -> $target:tt
        use $bind:tt in fn($sig:ident)
        $code:block
    ) => {
        impl_simplify!(
            $(where $($tvar $(:
            $req$([$($p_req)*])?
            $(+ $reqs$([$($p_reqs)*])?)*
            )?),+;)?

            for $from -> $target
            use fn($sig) {
                let unit_pat!($bind): $crate::utype!($from) = $sig;
                // $($code)*
                $code
            }
        );
    };
    //endregion
    //region Method form: Automatic rearrangements, implemented as Quantity methods.
    (
        $(where $($tvar:ident $(:
        $req:ident$([$($p_req:tt)*])?
        $(+ $reqs:ident$([$($p_reqs:tt)*])?)*
        )?),+;)?

        impl $from:tt {
            $($vis:vis fn $function:ident() -> $target:tt;)*
        }
    ) => {
        impl<__V: $crate::scalar::Scalar $($(, $tvar $(: $req $(+ $reqs)*)?)+)?>
        $crate::quantity::Quantity<$crate::utype!($from), __V> {
            $(
            $vis fn $function(self) -> $crate::quantity::Quantity<$crate::utype!($target), __V> {
                #[allow(non_snake_case)]
                let unit_pat!($from): $crate::utype!($from) = self.unit;
                $crate::quantity::Quantity {
                    value: self.value,
                    unit: unit_pat!($target),
                }
            }
            )*
        }
    };
    //endregion
}


/*dim_macros::impl_reorg! {
    where U: Unit;

    // impl U^-3 {}

    impl U^-2 {
        as (1/U)^2;
    }

    impl U^-1 {}

    // impl U^ 0 {}

    impl U^+1 {
        as 1/(1/U);

        // as U^2 / U;
        // as U^3 / U^2;
    }

    // impl U^+2 {
    //     as U * U;
    //     as U^3 / U;
    // }
    //
    // impl U^+3 {
    //     as (U*U) * U;
    //     as U * (U*U);
    //     as U * U^2;
    //     as U^2 * U;
    //     // as U^4 / U;
    // }

    // impl U^+4 {}
}*/


/*dim_macros::impl_reorg! {
    where A: Unit, B: Unit;

    // impl A * B {
    //     // as B * A;
    //     as A / B^-1;
    // }
    impl A / B {
        as A * B^-1;
        // as B^-1 * A;
    }
    // impl A^3 / B^2 {}
}*/


//  1/(1/x) = x
impl_simplify! {
    where U: Unit;
    impl (1/(1/U)) <-> U;
}

//  1/(a/b) = b/a
impl_simplify! {
    where A: Unit, B: Unit;
    impl (1/(A/B)) <-> (B/A);
}

//  a*b = b*a
impl_simplify! {
    where A: Unit, B: Unit;
    impl (A*B) -> (B*A);
}

//region Single fractions.
//  a * (1/b) = a/b
impl_simplify! {
    where A: UnitNonExp, B: Unit;
    impl (A * (1/B)) <-> (A/B);
}
impl_simplify! {
    where A: UnitNonExp, B: Unit;
    impl ((1/B) * A) <-> (A/B);
}
impl_simplify! {
    where A: UnitNonExp, B: Unit;
    impl (A / (1/B)) <-> (A*B);
}
// impl_simplify! {
//     where A: UnitNonExp, B: Unit;
//     impl (A / (1/B)) <-> (B*A);
// }

//  a / (b/c) = ac/b
impl_simplify! {
    where A: Unit, B: Unit, C: Unit;
    impl (A / (B/C)) <-> ((A*C) / B);
}

//  a / (b/c) = a * (c/b)
impl_simplify! {
    where A: Unit, B: Unit, C: Unit;
    impl (A / (B/C)) <-> (A * (C/B));
}


//  a / b² = a/b/b
impl_simplify! {
    where A: Unit, B: Unit;
    impl (A / B^2) -> (A/B/B);
}
//  a/b/b = a / b²
impl_simplify! {
    where A: Unit, B: Unit;
    for (A/B /B ) -> (A / B^2)
    use (a/b1/b2) in fn(self) { (UnitDiv(a, UnitSquared::new(b1)), b2.scale_factor(b1)) }
}

impl_simplify! {
    where A: UnitNonExp, B: Unit;
    for ((A /B) / A ) -> (1/B)
    use ((a1/b) / a2) in fn(self) { (PerUnit(b), a1.scale_factor(a2)) }
}

//  a / (a/b) = b
impl_simplify! {
    where A: Unit, B: Unit;
    for (A /(A /B)) -> B
    use (a1/(a2/b)) in fn(self) { (b, a1.scale_factor(a2)) }
}


//  TODO: Move elsewhere, probably.
impl_simplify! {
    where A: Unit, B: Unit, C: Unit;
    impl ((A/B) * C) {
        //  (a/b) * c = c / (b/a)
        pub fn invert_left() -> (C / (B/A));
    }
}
impl_simplify! {
    where A: Unit, B: Unit, C: Unit;
    impl (A * (B/C)) {
        //  a * (b/c) = a / (c/b)
        pub fn invert_right() -> (A / (C/B));
    }
}
//endregion

//region Pairs of fractions.
//  (1/a) * (1/b) = 1/ab
impl_simplify! {
    where A: Unit, B: Unit;
    impl ((1/A) * (1/B)) <-> (1/(A*B));
}

//  (a/b) * (1/b) = a/b²
impl_simplify! {
    where A: Unit, B: Unit;
    for ((A/B ) * (1/B )) -> (A/B^2)
    use ((a/b1) * (1/b2)) in fn(self) { (UnitDiv(a, UnitSquared::new(b1)), b2.scale_factor(b1)) }
}

//  (a/b) / (1/b) = a
impl_simplify! {
    where A: Unit, B: Unit;
    for ((A/B ) / (1/B )) -> A
    use ((a/b1) / (1/b2)) in fn(self) { (a, b2.scale_factor(b1)) }
}

//  (a/c) * (b/d) = ab/cd
impl_simplify! {
    where
    A: Unit, B: Unit,
    C: Unit, D: Unit;
    impl ((A/C) * (B/D)) <-> ((A*B) / (C*D));
}

//  (a/c) / (b/d) = ad/cb
impl_simplify! {
    where
    A: Unit, B: Unit, // a b    ad
    C: Unit, D: Unit; // c d    cb
    impl ((A/C) / (B/D)) <-> ((A*D) / (C*B));
}
//endregion

//region Exponents.
//  x * x = x²
impl_simplify! {
    where U: Unit;
    for (U * U) -> (U^2)
    use (a * b) in fn(self) { (UnitSquared::new(a), b.scale() / a.scale()) }
}

//  x² * x = x³
impl_simplify! {
    where U: Unit;
    for (U^2 * U) -> (U^3)
    use (a^2 * b) in fn(self) { (UnitCubed::new(a), b.scale() / a.scale()) }
}

//  x³ / x = x²
impl_simplify! {
    where U: Unit;
    for (U^3 / U) -> (U^2)
    use (a^3 / b) in fn(self) { (UnitSquared::new(a), a.scale() / b.scale()) }
}

//  x² / x = x¹
impl_simplify! {
    where U: UnitNonExp; // TODO
    for (U^2 / U) -> U
    use (a^2 / b) in fn(self) { (a, a.scale() / b.scale()) }
}
//endregion


//  (a/b)*b = a
impl_simplify! {
    where A: Unit, B: Unit;
    for ((A/B ) * B ) -> (A)
    use ((a/b1) * b2) in fn(self) { (a, b2.scale() / b1.scale()) }
}
impl_simplify! {
    where A: Unit, B: Unit;
    for (B  * (A/B )) -> (A)
    use (b2 * (a/b1)) in fn(self) { (a, b2.scale() / b1.scale()) }
}

//  ab/b = a
impl_simplify! {
    where A: Unit, B: Unit;
    for ((A*B ) / B ) -> (A)
    use ((a*b1) / b2) in fn(self) { (a, b1.scale() / b2.scale()) }
}
/*//  Conflicting impl?
impl_simplify! {
    where A: Unit, B: Unit;
    for ((B *A) / B ) -> (A)
    use ((b1*a) / b2) in fn(self) { (a, b1.scale() / b2.scale()) }
}*/


//region Distribution of powers in binary operations.
//region aⁿ * bⁿ == (a*b)ⁿ
impl<A: UnitExp, B: UnitExp, E: Exp>
Simplify<UnitPow<UnitMul<A::Output, B::Output>, E>>
for UnitMul<A, B> where
    A: CanRoot<E>,
    B: CanRoot<E>,
{
    fn simplify<S: Scalar>(self) -> Conversion<UnitPow<UnitMul<A::Output, B::Output>, E>, S> {
        Conversion::units(UnitPow::new(UnitMul::new(self.0.root(), self.1.root())))
    }
}

impl<A: Unit, B: Unit, E: Exp>
Simplify<UnitMul<A::Output, B::Output>>
for UnitPow<UnitMul<A, B>, E> where
    A: CanPow<E>,
    B: CanPow<E>,
{
    fn simplify<S: Scalar>(self) -> Conversion<UnitMul<A::Output, B::Output>, S> {
        Conversion::units(UnitMul::new(self.0.0.pow(), self.0.1.pow()))
    }
}
//endregion


//region aⁿ / bⁿ == (a/b)ⁿ
impl<A: UnitExp, B: UnitExp, E: Exp>
Simplify<UnitPow<UnitDiv<A::Output, B::Output>, E>>
for UnitDiv<A, B> where
    A: CanRoot<E>,
    B: CanRoot<E>,
{
    fn simplify<S: Scalar>(self) -> Conversion<UnitPow<UnitDiv<A::Output, B::Output>, E>, S> {
        Conversion::units(UnitPow::new(UnitDiv::new(self.0.root(), self.1.root())))
    }
}

impl<A: Unit, B: Unit, E: Exp>
Simplify<UnitDiv<A::Output, B::Output>>
for UnitPow<UnitDiv<A, B>, E> where
    A: CanPow<E>,
    B: CanPow<E>,
{
    fn simplify<S: Scalar>(self) -> Conversion<UnitDiv<A::Output, B::Output>, S> {
        Conversion::units(UnitDiv::new(self.0.0.pow(), self.0.1.pow()))
    }
}
//endregion
//endregion


#[cfg(test)]
mod tests {
    use crate::{Quantity, units::symbols::basic::*};
    use super::*;

    #[test]
    fn test_cancel() {
        let v: Quantity<Speed> = qty!(2.0 mm/ms);
        let t: Quantity<Time> = qty!(3.0 min);
        let d: Quantity<Length> = (v * t).simplify();
        assert_eq!(d, qty!(360.0 m));

        let dt: qtype!(l * t) = qty!(90.0 m*s);
        let t: Quantity<Time> = qty!(0.5 min);
        let d: Quantity<Length> = (dt / t).simplify();
        assert_eq!(d, qty!(3.0 m));
    }

    #[test]
    fn test_compounds() {
        let l = qty!(5.0 m);
        let t = qty!(2.0/s);

        let q: qtype!(l * (1/t)) = l * t;
        let v: qtype!(l / t) = q.simplify();

        assert_eq!(v, qty!(10.0 m/s));
    }

    #[test]
    fn test_powers() {
        //  Start with basic length.
        let x1: Quantity<Length> = qty!(2.0 m);

        //  Multiply and then simplify to square.
        let x1mul:  qtype!(l * l)   = x1*x1.with_unit(Length::MilliMeter);
        let x2:     qtype!(l^2)     = x1mul.simplify();

        //  Multiply and then simplify to cube.
        let x2mul:  qtype!(l^2 * l) = x2*x1.with_unit(Length::KiloMeter);
        let x3:     qtype!(l^3)     = x2mul.simplify();

        //  Ensure the results match.
        assert_eq!(x2, x1.squared());
        assert_eq!(x3, x1.cubed());

        //  Ensure the results are actually correct.
        assert_eq!(x2, qty!(4.0 m^2));
        assert_eq!(x3, qty!(8.0 m^3));

        //  Climb back down.

        //  Divide and then simplify back down to square.
        let x3div:  qtype!(l^3 / l) = x3/x1.with_unit(Length::MilliMeter);
        let x2:     qtype!(l^2)     = x3div.simplify();

        //  Divide and then simplify back down to square.
        let x2div:  qtype!(l^2 / l) = x2/x1.with_unit(Length::KiloMeter);
        let x1:     qtype!(l^1)     = x2div.simplify();

        //  Ensure the results are still correct.
        assert_eq!(x2, qty!(4.0 m^2));
        assert_eq!(x1, qty!(2.0 m^1));
    }
}
