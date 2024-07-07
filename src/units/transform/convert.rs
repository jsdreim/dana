use crate::{constants::*, units::{*, traits::*}};


/// Implement conversion (both ways) between two [`Unit`] types, which can be
///     defined in mathematical form.
macro_rules! impl_conversion {
    ($left:tt = $right:tt $(* const $coeff:expr)?) => {
        impl ConvertFrom<$crate::utype!($right)> for $crate::utype!($left) {
            fn conversion_factor_from(&self, unit: $crate::utype!($right)) -> f64 {
                unit.scale() / self.scale() $(* $coeff.value_as_base())?
            }
        }

        impl ConvertFrom<$crate::utype!($left)> for $crate::utype!($right) {
            fn conversion_factor_from(&self, unit: $crate::utype!($left)) -> f64 {
                unit.scale() / self.scale() $(/ $coeff.value_as_base())?
            }
        }
    };
    //  If a relationship is defined with an inverse constant, rearrange it to
    //      work with the first case.
    ($left:tt = $right:tt / const $denom:expr) => {
        impl_conversion!($right = $left * const $denom);
    };
    ($left:tt = $($right:tt)*) => { impl_conversion!($left = ($($right)*)); };
}

/// Given a simple three-term relationship, implement two-way conversions for
///     each of the possible permutations.
macro_rules! impl_relationship {
    //  Define one unit as equivalent to another multiplied by a constant.
    ($a:ident = $b:tt $($op:tt const $coeff:expr)?) => {
        impl_conversion!($a = $b $($op const $coeff)?);
    };
    //  Define two units as inversely proportional.
    ($a:ident = 1 / $b:ident) => {
        impl_conversion!($a = 1 / $b); // A=1/B
        impl_conversion!($b = 1 / $a); // B=1/A
    };
    //  Define one unit as the product of two others.
    ($a:ident = $b:ident * $c:ident) => {
        impl_conversion!($a = $b * $c); // A=BC
        impl_conversion!($a = $c * $b); // A=CB
        impl_conversion!($b = $a / $c); // B=A/C
        impl_conversion!($c = $a / $b); // C=A/B
        impl_conversion!((1/$b) = $c / $a); // 1/B = C/A
        impl_conversion!((1/$c) = $b / $a); // 1/C = B/A
    };
    //  Define one unit as the quotient of two others.
    ($a:ident = $b:ident / $c:ident) => {
        impl_conversion!($a = $b / $c); // A=B/C
        impl_conversion!($c = $b / $a); // C=B/A
        impl_conversion!($b = $a * $c); // B=AC
        impl_conversion!($b = $c * $a); // B=CA
        impl_conversion!((1/$a) = $c / $b); // 1/A = C/B
        impl_conversion!((1/$c) = $a / $b); // 1/C = A/B
    };
    ($($a:ident = $b:tt $($op:tt $c:tt $($coeff:expr)?)?;)+) => {
        $(impl_relationship!($a = $b $($op $c $($coeff)?)?);)+
    };
}

impl_relationship! {
    Frequency = 1 / Time; // f=1/t
    Force = Mass * Accel; // F=ma
    Power = Energy / Time; // P=E/t
    Power = Current * Voltage; // P=IV
    Voltage = Current * Resistance; // V=IR

    Power = Energy * Frequency; // P = E/t = E(1/t) = Ef
    Energy = Mass * const C.squared(); // E=mc²
}


#[cfg(test)]
mod tests {
    use crate::Quantity;
    use super::*;

    #[test]
    fn test_e_mc2() {
        let m: Quantity<Mass> = quantity!(1.0 kg);
        let e: Quantity<Energy> = quantity!(m in J);
        assert_eq!(e.value_as(Energy::KiloJoule).floor(), 89_875_517_873_681.0);
    }

    #[test]
    fn test_f_ma() {
        let m: Quantity<Mass> = quantity!(2.0 kg);
        let a: Quantity<Accel> = quantity!(3.0 km/s/s);

        let f: Quantity<Force> = quantity!((m * a) as _);
        assert_eq!(f, quantity!(6.0 kN));
    }

    #[test]
    fn test_electrical() {
        //  3V3 across a 150Ω resistor.
        let v: Quantity<Voltage> = quantity!(3.3 V);
        let r: Quantity<Resistance> = quantity!(150.0 Ω);

        //  Should measure 22mA of current through the resistor.
        let i: Quantity<Current> = quantity!((v / r) in A);
        assert_eq!(i, quantity!(22.0 mA));

        //  Resistor should be dissipating 72.6mW as heat.
        let p: Quantity<Power> = quantity!((i * v) in W);
        assert_eq!(p, quantity!(72.6 mW));

        //  After 5 minutes, should have dissipated 21.78J in total.
        let t: Quantity<Time> = quantity!(300.0 s);
        let e: Quantity<Energy> = quantity!((p * t) in J);
        assert_eq!(e, quantity!(21.78 J));
    }
}
