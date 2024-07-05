use crate::{constants::*, units::{concrete::*, traits::*}};


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
    let kg = quantity!(1.0 Mass::Kilogram);
    let e = kg.convert_to(Energy::Joule);

    assert!((89.8e15..89.9e15).contains(&e.value));
}
