use crate::{constants::*, units::{*, traits::*}};


macro_rules! define_relationship {
    ($left:tt = $($right:tt)*) => {
        impl ConvertFrom<$crate::utype!($($right)*)> for $crate::utype!($left) {
            fn conversion_factor_from(&self, unit: $crate::utype!($($right)*)) -> f64 {
                unit.scale() / self.scale()
            }
        }

        impl ConvertFrom<$crate::utype!($left)> for $crate::utype!($($right)*) {
            fn conversion_factor_from(&self, unit: $crate::utype!($left)) -> f64 {
                unit.scale() / self.scale()
            }
        }
    };
}


//  F=ma
define_relationship!(Force = Mass * Accel); // F=ma
define_relationship!(Force = Accel * Mass); // F=am
define_relationship!(Mass = Force / Accel); // m=F/a
define_relationship!(Accel = Force / Mass); // a=F/m
define_relationship!((Mass^-1) = Accel / Force); // 1/m = a/F
define_relationship!((Accel^-1) = Mass / Force); // 1/a = m/F


// define_relationship!(Energy = Force * Distance);
// define_relationship!(Energy = Distance * Force);


impl ConvertFrom<Mass> for Energy {
    fn conversion_factor_from(&self, unit: Mass) -> f64 {
        (unit.scale() / self.scale()) * (C.value * C.value)
    }
}

impl ConvertFrom<Energy> for Mass {
    fn conversion_factor_from(&self, unit: Energy) -> f64 {
        (unit.scale() / self.scale()) / (C.value * C.value)
    }
}


#[test]
fn test_e_mc2() {
    let kg = quantity!(1.0 kg);
    let e = kg.convert_to(Energy::Joule);

    assert!((89.8e15..89.9e15).contains(&e.value));
}


#[test]
fn test_f_ma() {
    let m: qtype!(Mass) = quantity!(2.0 g);
    let a: qtype!(Accel) = quantity!(3.0 km/s/s);
    let f: qtype!(Force) = quantity!((m * a) as kN);

    assert_eq!(
        f.value_as(unit!(N)),
        m.value_as(unit!(kg)) * a.value_as(unit!(m/s/s))
    );
    assert_eq!(f.value_as(unit!(N)), 6.0);
}
