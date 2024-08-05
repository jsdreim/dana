//! Module for error types.


/// Error while converting `Quantity<Time>` into [`chrono::TimeDelta`].
#[cfg(feature = "chrono")]
#[derive(Debug, thiserror::Error)]
pub enum TimeDeltaError<V: std::fmt::Debug> {
    /// Quantity value cannot be cast to `f64`.
    #[error("cannot cast {0:?} to f64")]
    CastFailed(V),
    /// Quantity is outside the range [`chrono::TimeDelta`] can represent.
    #[error("duration is out of bounds")]
    OutOfBounds,
}
