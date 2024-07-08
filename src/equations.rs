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

    #[test]
    fn test_gravity_earth() {
        let m_stone: Quantity<Mass> = qty![90.0 kg];
        let m_earth: Quantity<Mass> = qty![5.9722e24 kg];
        let r_earth: Quantity<Length> = qty![6.3781e6 m];

        let f: Quantity<Force> = gravity(m_stone, m_earth, r_earth);
        let a: Quantity<Accel> = qty![(f / m_stone) as _];

        assert!((GFORCE - a).abs() < qty![1.0 cm/s/s]);
    }
}
