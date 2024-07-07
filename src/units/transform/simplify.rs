use crate::units::{*, traits::*};


macro_rules! impl_simplify {
    //  Direct form: Automatic rearrangement with no effect on value.
    (
        $(where $($tvar:ident $(: $req:ident $(+ $reqs:ident)*)?),+;)?

        let $from:tt = $target:tt;
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
    //  Simple form: Rearrangement of bindings with no effect on value.
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
    //  Function form: Allow arbitrary code to produce `Conversion`.
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
                $code
            }
        }
    };
    //  Bound function form: Allow arbitrary code to produce `Conversion`, with
    //      variables bound to a pattern.
    (
        // $(where $($tvar:ident $(: $req:ident $(+ $reqs:ident)*)?),+;)?
        $(where $($tvar:ident $(:
        $req:ident$([$($p_req:tt)*])?
        $(+ $reqs:ident$([$($p_reqs:tt)*])?)*
        )?),+;)?

        for $from:tt -> $target:tt
        use $bind:tt in fn($sig:ident)
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
                let unit_pat!($bind): $crate::utype!($from) = $sig;
                // $($code)*
                $code
            }
        }
    };
    //  Method form: Automatic rearrangement, implemented as Quantity method.
    //  TODO: This should be its own macro.
    (
        $(where $($tvar:ident $(:
        $req:ident$([$($p_req:tt)*])?
        $(+ $reqs:ident$([$($p_reqs:tt)*])?)*
        )?),+;)?

        impl $from:tt -> $target:tt
        as $vis:vis fn $function:ident();
    ) => {
        impl<__V: $crate::scalar::Scalar $($(, $tvar $(: $req $(+ $reqs)*)?)+)?>
        $crate::quantity::Quantity<$crate::utype!($from), __V> {
            $vis fn $function(self) -> $crate::quantity::Quantity<$crate::utype!($target), __V> {
                #[allow(non_snake_case)]
                let unit_pat!($from): $crate::utype!($from) = self.unit;
                $crate::quantity::Quantity {
                    value: self.value,
                    unit: unit_pat!($target),
                }
            }
        }
    };
}


//  1/(1/x) = x
impl_simplify! {
    where U: Unit;
    let (1/(1/U)) = U;
}

//  1/(a/b) = b/a
impl_simplify! {
    where A: Unit, B: Unit;
    let (1/(A/B)) = (B/A);
}

//region Single fractions.
//  a * (1/b) = a/b
impl_simplify! {
    where A: UnitNonExp, B: Unit;
    let (A * (1/B)) = (A/B);
}
impl_simplify! {
    where A: UnitNonExp, B: Unit;
    let ((1/B) * A) = (A/B);
}

//  a / (b/c) = ac/b
impl_simplify! {
    where A: Unit, B: Unit, C: Unit;
    let (A/(B/C)) = ((A*C)/B);
}

//  a / (b/c) = a * (c/b)
impl_simplify! {
    where A: Unit, B: Unit, C: Unit;
    let (A / (B/C)) = (A * (C/B));
}

//  a * (b/c) = a / (c/b)
impl_simplify! {
    where A: Unit, B: Unit, C: Unit;
    let (A * (B/C)) = (A / (C/B));
}

//  TODO: Move elsewhere, probably.
// //  a / (b/c) = a * (c/b)
// impl_simplify! {
//     where A: Unit, B: Unit, C: Unit;
//     impl (A / (B/C)) -> (A * (C/B))
//     as pub fn invert_right();
// }
//
// //  a * (b/c) = a / (c/b)
// impl_simplify! {
//     where A: Unit, B: Unit, C: Unit;
//     impl (A * (B/C)) -> (A / (C/B))
//     as pub fn invert_right();
// }
//endregion

//region Pairs of fractions.
//  (1/a) * (1/b) = 1/ab
impl_simplify! {
    where A: Unit, B: Unit;
    let ((1/A) * (1/B)) = (1/(A*B));
}

//  (a/c) * (b/d) = ab/cd
impl_simplify! {
    where
    A: Unit, B: Unit,
    C: Unit, D: Unit;
    let ((A/C) * (B/D)) = ((A*B) / (C*D));
}

//  (a/c) / (b/d) = ad/cb
impl_simplify! {
    where
    A: Unit, B: Unit, // a b    ad
    C: Unit, D: Unit; // c d    cb
    let ((A/C) / (B/D)) = ((A*D) / (C*B));
}
//endregion

//region Exponents.
//  x * x = x²
impl_simplify! {
    where U: Unit;
    for (U * U) -> (U^2)
    use (a * b) in fn(self) {
        let want = a.scale();
        let have = b.scale();
        let factor = __S::from_f64(have / want).unwrap();
        Conversion::scale(UnitSquared(a), factor)
    }
}

//  x² * x = x³
impl_simplify! {
    where U: Unit;
    for (U^2 * U) -> (U^3)
    use (a^2 * b) in fn(self) {
        let want = a.scale();
        let have = b.scale();
        let factor = __S::from_f64(have / want).unwrap();
        Conversion::scale(UnitCubed(a), factor)
    }
}

//  x³ / x = x²
impl_simplify! {
    where U: Unit;
    for (U^3 / U) -> (U^2)
    use (a^3 / b) in fn(self) {
        let want = a.scale();
        let have = b.scale();
        let factor = __S::from_f64(have / want).unwrap();
        Conversion::scale(UnitSquared(a), factor)
    }
}

//  x² / x = x¹
impl_simplify! {
    where U: Unit;
    for (U^2 / U) -> U
    use (a^2 / b) in fn(self) {
        let want = a.scale();
        let have = b.scale();
        let factor = __S::from_f64(have / want).unwrap();
        Conversion::scale(a, factor)
    }
}
//endregion


#[cfg(test)]
mod tests {
    use crate::Quantity;
    use super::*;

    #[test]
    fn test_compounds() {
        let l = quantity!(5.0 m);
        let t = quantity!(2.0/s);

        let q: qtype!(l * (1/t)) = l * t;
        let v: qtype!(l / t) = q.simplify();

        assert_eq!(v, quantity!(10.0 m/s));
    }

    #[test]
    fn test_powers() {
        //  Start with basic length.
        let x1: Quantity<Distance> = quantity!(2.0 m);

        //  Multiply and then simplify to square.
        let x1_x1:  qtype!(l * l)   = x1*x1.with_unit(Length::Millimeter);
        let x2:     qtype!(l^2)     = x1_x1.simplify();

        //  Multiply and then simplify to cube.
        let x2_x1:  qtype!(l^2 * l) = x2*x1.with_unit(Length::Kilometer);
        let x3:     qtype!(l^3)     = x2_x1.simplify();

        //  Ensure the results match.
        assert_eq!(x2, x1.squared());
        assert_eq!(x3, x1.cubed());

        //  Ensure the results are actually correct.
        assert_eq!(x2, quantity!(4.0 m^2));
        assert_eq!(x3, quantity!(8.0 m^3));
    }
}
