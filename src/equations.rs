use crate::{constants::*, quantity::Quantity, units::*};


impl Quantity<Mass> {
    pub fn grav_param(self) -> Quantity<GravParam> {
        (self * G).simplify()
    }
}


pub fn gravity(
    mass_1: Quantity<Mass>,
    mass_2: Quantity<Mass>,
    distance: Quantity<Distance>,
) -> Quantity<Force> {
    //  F = G(M₁M₂ / r²)

    let force: qtype!((l^3/t^2/m) * (m*m/l^2)) = G * (
        (mass_1 * mass_2)
        / distance.squared()
    );

    force.convert()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gravity() {
        let m1 = qty!(5.0 kg);
        let m2 = qty!(5.0 kg);
        let d = qty!(20.0 mm);
        let f = gravity(m1, m2, d);

        assert_eq!(
            format!("{:.3e}", f),
            format!("{:.3e}", qty!(4.171e-6 N)),
        );
    }
}
