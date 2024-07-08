#[macro_use]
mod macros;

pub mod constants;
pub mod equations;
pub mod quantity;
pub mod scalar;
pub mod units;

pub use quantity::Quantity;
pub use scalar::Scalar;
pub use units::Unit;


#[cfg(test)]
mod tests {
    use num_traits::Inv;
    use crate::{Quantity, units::{*, traits::*}};

    #[test]
    pub fn test_macros() {
        //  Start with a basic length unit, and ensure `unit!` correctly
        //      produces one.
        let u: Length = unit!(m);
        assert_eq!(u, u.inv().inv());

        //  Confirm that `utype!` produces types that agree.
        let _: utype!(l) = u;

        //  Check implicitly-positive exponents.
        let _: utype!(l^ 1) = u;
        let _: utype!(l^ 2) = u.squared();
        let _: utype!(l^ 3) = u.cubed();
        let _: utype!(l^ 4) = u.pow(4.0);
        let _: utype!(l^ 2.0) = u.pow(2.0);

        //  Check explicitly-positive exponents.
        let _: utype!(l^+1) = u;
        let _: utype!(l^+2) = u.squared();
        let _: utype!(l^+3) = u.cubed();
        let _: utype!(l^+4) = u.pow(4.0);
        let _: utype!(l^+2.0) = u.pow(2.0);

        //  Check explicitly-negative exponents.
        let _: utype!(l^-1) = u.inv();
        let _: utype!(l^-2) = u.squared().inv();
        let _: utype!(l^-3) = u.cubed().inv();
        let _: utype!(l^-4) = u.pow(4.0).inv();
        let _: utype!(l^-2.0) = u.pow(2.0).inv();


        //  Use that unit for a quantity, and ensure the `qty!` macro correctly
        //      produces one.
        let q: Quantity<Length> = qty!(2.0 u);
        assert_eq!(q, q.inv().inv());

        //  Confirm that `qtype!` produces types that agree.
        let _: qtype!(l) = q;

        //  Check implicitly-positive exponents.
        let _: qtype!(l^ 1) = q;
        let _: qtype!(l^ 2) = q.squared();
        let _: qtype!(l^ 3) = q.cubed();
        let _: qtype!(l^ 4) = q.pow(4.0);
        let _: qtype!(l^ 2.0) = q.pow(2.0);

        //  Check explicitly-positive exponents.
        let _: qtype!(l^+1) = q;
        let _: qtype!(l^+2) = q.squared();
        let _: qtype!(l^+3) = q.cubed();
        let _: qtype!(l^+4) = q.pow(4.0);
        let _: qtype!(l^+2.0) = q.pow(2.0);

        //  Check explicitly-negative exponents.
        let _: qtype!(l^-1) = q.inv();
        let _: qtype!(l^-2) = q.squared().inv();
        let _: qtype!(l^-3) = q.cubed().inv();
        let _: qtype!(l^-4) = q.pow(4.0).inv();
        let _: qtype!(l^-2.0) = q.pow(2.0).inv();
    }

    #[test]
    fn test_scale() {
        let dist = Distance::Millimeter.quantity(50.0);

        let mm = dist.with_unit(Distance::Millimeter).value;
        let cm = dist.with_unit(Distance::Centimeter).value;

        dbg!(mm, cm);
    }
}
