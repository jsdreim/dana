use crate::{
    constants::*,
    Quantity,
    Scalar,
    units::{*, symbols::physical::*},
};


pub fn gravitational_parameter(mass: Quantity<Mass>) -> Quantity<GravParam> {
    (mass * G).simplify()
}


pub fn gravity(
    mass_1: Quantity<Mass>,
    mass_2: Quantity<Mass>,
    dist: Quantity<Length>,
) -> Quantity<Force> {
    //  F = G(M₁M₂ / r²)

    let force: qtype!((l^3/t^2/m) * (m*m/l^2)) = G * (
        (mass_1 * mass_2)
        / dist.squared()
    );

    force.convert()
}


pub fn photon_energy(freq: Quantity<Frequency>) -> Quantity<Energy> {
    (freq * H).simplify()
}


pub fn photon_frequency(energy: Quantity<Energy>) -> Quantity<Frequency> {
    (energy / H).simplify()
}


pub fn frequency_to_wavelength<V: Scalar + 'static>(
    freq: Quantity<Frequency, V>,
    speed: Quantity<Speed, V>,
) -> Quantity<Length, V> {
    qty![speed / [freq as 1/Time] -> Length]
}


pub fn wavelength_to_frequency<V: Scalar + 'static>(
    length: Quantity<Length, V>,
    speed: Quantity<Speed, V>,
) -> Quantity<Frequency, V> {
    qty![speed / length -> 1/Time as Frequency]
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

    #[test]
    fn test_photon() {
        fn test(wave: Quantity<L>, energy_expected: Quantity<E>) {
            //  Check the two stages of conversion separately.
            let freq = wavelength_to_frequency(wave, C);
            let energy = photon_energy(freq);

            //  Confirm that the functions are symmetric.
            assert_eq!(wave, frequency_to_wavelength(freq, C));
            assert_eq!(freq, photon_frequency(energy));

            //  Confirm that the results are correct.
            assert_eq!(
                format!("{:.3e}", energy),
                format!("{:.3e}", energy_expected),
            );
        }

        test(qty![685.0 nm], qty![1.810 eV]);
        test(qty![535.0 nm], qty![2.317 eV]);
        test(qty![440.0 nm], qty![2.818 eV]);
    }
}
