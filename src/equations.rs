use crate::{
    constants::*,
    Quantity,
    Scalar,
    symbols::physical::*,
    units::*,
};


impl<V: Scalar> Quantity<Mass, V> {
    /// Return the gravitational force experienced by an object of this mass at
    ///     the surface of Earth.
    pub fn mass_as_weight(self) -> Option<Quantity<Force, V>> {
        Some((self * GFORCE.scalar_cast()?).convert())
    }
}

impl<V: Scalar> Quantity<Force, V> {
    /// Return the mass that would experience this gravitational force at the
    ///     surface of Earth.
    pub fn weight_to_mass(self) -> Option<Quantity<Mass, V>> {
        Some((self / GFORCE.scalar_cast()?).convert())
    }
}


pub fn gravitational_parameter(mass: Quantity<Mass>) -> Quantity<GravParam> {
    (mass * CONST_G).convert()
}


pub fn gravity(
    mass_1: Quantity<Mass>,
    mass_2: Quantity<Mass>,
    dist: Quantity<Length>,
) -> Quantity<Force> {
    //  F = G(M₁M₂ / r²)

    let force: qtype!((L^3/T^2/M) * (M*M/L^2)) = CONST_G * (
        (mass_1 * mass_2)
        / dist.squared()
    );

    force.convert()
}


pub fn mass_to_energy(mass: Quantity<Mass>) -> Quantity<Energy> {
    qty![mass * {CONST_C.squared()} as Energy]
}


pub fn energy_to_mass(energy: Quantity<Energy>) -> Quantity<Mass> {
    qty![energy / {CONST_C.squared()} as Mass]
}


pub fn photon_energy(freq: Quantity<Frequency>) -> Quantity<Energy> {
    qty![freq * CONST_H in eV]
}


pub fn photon_frequency(energy: Quantity<Energy>) -> Quantity<Frequency> {
    qty![energy / CONST_H in Hz]
}


pub fn frequency_to_wavelength<V: Scalar>(
    freq: Quantity<Frequency, V>,
    speed: Quantity<Speed, V>,
) -> Quantity<Length, V> {
    qty![speed / [freq as 1/Time] -> Length]
}


pub fn wavelength_to_frequency<V: Scalar>(
    length: Quantity<Length, V>,
    speed: Quantity<Speed, V>,
) -> Quantity<Frequency, V> {
    qty![speed / length -> 1/Time as Frequency]
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_e_mc2() {
        let mass_0: Quantity<Mass> = qty![1.0 kg];
        let energy: Quantity<Energy> = mass_to_energy(mass_0);
        let mass_1: Quantity<Mass> = energy_to_mass(energy);

        assert_eq!(qty![*energy in kJ].floor(), 89_875_517_873_681.0);
        assert_eq!(mass_0, mass_1);
    }

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
            let freq = wavelength_to_frequency(wave, CONST_C);
            let energy = photon_energy(freq);

            //  Confirm that the functions are symmetric.
            assert_eq!(wave, frequency_to_wavelength(freq, CONST_C));
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
