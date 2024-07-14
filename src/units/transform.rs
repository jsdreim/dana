// pub mod cancel;
pub mod convert;
pub mod simplify;


#[cfg(test)]
mod tests {
    use crate::units::symbols::*;

    #[test]
    fn test_xform() {
        let v = qty![3.0 m/s];
        let t = qty![2.0 s];
        let a = qty![v/t -> L/T^2];

        let a2_a = qty![
            a * a
            in (km  / h^2) * (km / h^2)
            -> (L   / T^2) ^ 2
            -> (L^2 / T^4)
        ].floor();

        let a2_b = qty![
            a * a
            -> (L   / T^2) ^ 2
            in (km  / h^2) ^ 2
            -> (L^2 / T^4)
        ].floor();

        let a2_c = qty![
            a * a
            -> (L    / T^2) ^ 2
            -> (L^2  / T^4)
            in (km^2 / h^4)
        ].floor();

        assert_eq!(a2_a, qty![2.25 m^2/s^4]);
        assert_eq!(a2_a, a2_b);
        assert_eq!(a2_a, a2_c);
    }
}
