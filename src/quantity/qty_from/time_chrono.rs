use chrono::TimeDelta;
use num_traits::AsPrimitive;
use crate::{error::TimeDeltaError, Quantity, units::{*, Time::*}, Value};


impl<V: Value> TryFrom<Quantity<Time, V>> for TimeDelta {
    type Error = TimeDeltaError<V>;

    fn try_from(qty: Quantity<Time, V>) -> Result<Self, Self::Error> {
        let v: V = qty.value_as(Second);
        let total: f64 = v.to_f64().ok_or(TimeDeltaError::CastFailed(v))?;
        let secs: i64 = total.trunc() as i64;
        let nanos: u32 = (total.fract().abs() * 1e9) as u32;

        Self::new(secs, nanos).ok_or(TimeDeltaError::OutOfBounds)
    }
}


impl<V: Value + Copy> From<TimeDelta> for Quantity<Time, V> where
    i32: AsPrimitive<V>,
    i64: AsPrimitive<V>,
{
    fn from(td: TimeDelta) -> Self {
        let seconds: V = td.num_seconds().as_();
        let nanosec: V = td.subsec_nanos().as_();

        Second.quantity(seconds) + NanoSecond.quantity(nanosec)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn qty_time_delta() {
        let q_0 = Minute.quantity(61.5);
        let dur: TimeDelta = q_0.try_into().unwrap();
        let q_1: Quantity<Time> = dur.into();

        assert_eq!(q_0, q_1);
        assert_eq!(q_1, q_0);
    }
}
