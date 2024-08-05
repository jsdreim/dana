use core::time::Duration;
use crate::{Quantity, units::{*, Time::*}};


impl From<Quantity<Time, f32>> for Duration {
    fn from(qty: Quantity<Time, f32>) -> Self {
        Duration::from_secs_f32(qty.value_as(Second))
    }
}


impl From<Quantity<Time, f64>> for Duration {
    fn from(qty: Quantity<Time, f64>) -> Self {
        Duration::from_secs_f64(qty.value_as(Second))
    }
}


impl From<Duration> for Quantity<Time, f32> {
    fn from(duration: Duration) -> Self {
        Second.quantity(duration.as_secs_f32())
    }
}

impl From<Duration> for Quantity<Time, f64> {
    fn from(duration: Duration) -> Self {
        Second.quantity(duration.as_secs_f64())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn qty_duration() {
        let q_0 = Minute.quantity(61.5);
        let dur: Duration = q_0.into();
        let q_1: Quantity<Time> = dur.into();

        assert_eq!(q_0, q_1);
        assert_eq!(q_1, q_0);
    }
}
