use crate::{constants::*, units::{*, traits::*}};


impl ConvertTo<Mass> for Energy {
    fn conversion_factor(&self, unit: Mass) -> f64 {
        (self.scale() / unit.scale()) / (C.value * C.value)
    }
}

impl ConvertTo<Energy> for Mass {
    fn conversion_factor(&self, unit: Energy) -> f64 {
        (self.scale() / unit.scale()) * (C.value * C.value)
    }
}


#[test]
fn test_e_mc2() {
    let kg = quantity!(1.0 Mass::Kilogram);
    let e = kg.convert_to(Energy::Joule);

    dbg!(e);
}
