//! Functions for mathematical relationships between quantities.

use crate::{
    constants::*,
    Quantity,
    symbols::physical::*,
    units::*,
    Value,
};


impl<V: Value> Quantity<Mass, V> {
    /// Return the gravitational force experienced by an object of this mass at
    ///     the surface of Earth.
    pub fn mass_as_weight(self) -> Option<Quantity<Force, V>> {
        Some((self * GFORCE.value_cast()?).convert())
    }
}

impl<V: Value> Quantity<Force, V> {
    /// Return the mass that would experience this gravitational force at the
    ///     surface of Earth.
    pub fn weight_to_mass(self) -> Option<Quantity<Mass, V>> {
        Some((self / GFORCE.value_cast()?).convert())
    }
}


impl<V: Value> Quantity<Temp, V> {
    /// Define an absolute temperature in [`Kelvin`](Temp::Kelvin) in terms of
    ///     degrees Celsius.
    pub fn from_celsius(c: V) -> Self {
        Temp::Kelvin.quantity(c + V::from_f64(273.15).unwrap())
    }

    /// Define an absolute temperature in [`Kelvin`](Temp::Kelvin) in terms of
    ///     degrees Fahrenheit.
    pub fn from_fahrenheit(f: V) -> Self {
        Self::from_celsius((f - V::from_f64(32.0).unwrap()) / V::from_f64(1.8).unwrap())
    }

    /// Define an absolute temperature in [`Kelvin`](Temp::Kelvin) in terms of
    ///     degrees Rankine.
    pub fn from_rankine(r: V) -> Self {
        Temp::Kelvin.quantity(r / V::from_f64(1.8).unwrap())
    }

    /// Convert this temperature to degrees Celsius.
    pub fn to_celsius(self) -> V {
        self.value_as(Temp::Kelvin) - V::from_f64(273.15).unwrap()
    }

    /// Convert this temperature to degrees Fahrenheit.
    pub fn to_fahrenheit(self) -> V {
        self.to_celsius() * V::from_f64(1.8).unwrap() + V::from_f64(32.0).unwrap()
    }

    /// Convert this temperature to degrees Rankine.
    pub fn to_rankine(self) -> V {
        self.value_as(Temp::Kelvin) * V::from_f64(1.8).unwrap()
    }
}


/// Calculate the [Standard Gravitational Parameter] for a given mass.
///
/// [Standard Gravitational Parameter]: https://en.wikipedia.org/wiki/Standard_gravitational_parameter
pub fn gravitational_parameter(mass: Quantity<Mass>) -> Quantity<GravParam> {
    (mass * CONST_G).convert()
}


/// Given two masses at a given distance, calculate the gravitational [`Force`]
///     exerted on each mass towards the other.
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


/// Convert a mass to its corresponding energy, according to E=mc².
pub fn mass_to_energy(mass: Quantity<Mass>) -> Quantity<Energy> {
    (mass * CONST_C2).convert()
}


/// Convert an energy to its corresponding mass, according to E=mc².
pub fn energy_to_mass(energy: Quantity<Energy>) -> Quantity<Mass> {
    (energy / CONST_C2).convert()
}


/// Calculate the energy of a photon with the given frequency of light. The
///     result will be in [electron volts](Energy::ElectronVolt).
pub fn photon_energy(freq: Quantity<Frequency>) -> Quantity<Energy> {
    (freq * CONST_H).convert_to(Energy::ElectronVolt)
}


/// Calculate the frequency of light for a photon with the given energy.
pub fn photon_frequency(energy: Quantity<Energy>) -> Quantity<Frequency> {
    (energy / CONST_H).convert()
}


/// Calculate the wavelength of a wave with the given frequency and speed.
pub fn frequency_to_wavelength<V: Value>(
    freq: Quantity<Frequency, V>,
    speed: Quantity<Speed, V>,
) -> Quantity<Length, V> {
    (speed / freq).convert()
}


/// Calculate the frequency of a wave with the given wavelength and speed.
pub fn wavelength_to_frequency<V: Value>(
    length: Quantity<Length, V>,
    speed: Quantity<Speed, V>,
) -> Quantity<Frequency, V> {
    (speed / length).convert()
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

    #[test]
    fn test_temp() {
        assert_eq!(Quantity::from_fahrenheit(212.0).to_celsius(), 100.0);
        assert_eq!(Quantity::from_fahrenheit(32.0).to_celsius(), 0.0);

        assert_eq!(Quantity::from_celsius(100.0).to_fahrenheit(), 212.0);
        assert_eq!(Quantity::from_celsius(0.0).to_fahrenheit(), 32.0);

        assert_eq!(Temp::Kelvin.quantity(0.0).to_rankine(), 0.0);
        assert_eq!(Quantity::from_rankine(0f64).value_as(Temp::Kelvin), 0.0);

        assert_eq!(Quantity::from_fahrenheit(0f64).to_rankine().round(), 460.0);
        assert_eq!(Quantity::from_rankine(460f64).to_fahrenheit().round(), 0.0);

        assert_eq!(Quantity::from_celsius(0f64).to_rankine().round(), 492.0);
        assert_eq!(Quantity::from_rankine(492f64).to_celsius().round(), 0.0);
    }
}
