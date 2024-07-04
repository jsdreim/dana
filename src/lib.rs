#[macro_use]
mod macros;

pub mod constants;
pub mod quantity;
pub mod scalar;
pub mod units;

pub use quantity::Quantity;
pub use scalar::Scalar;
pub use units::Unit;


#[cfg(test)]
mod tests {
    use super::*;
    use units::concrete::Distance;

    #[test]
    fn test_scale() {
        let dist = Distance::Millimeter.quantity(50.0);

        let mm = dist.with_unit(Distance::Millimeter).value;
        let cm = dist.with_unit(Distance::Centimeter).value;

        dbg!(mm, cm);
    }
}
