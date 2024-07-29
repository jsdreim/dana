use std::time::Duration;
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
