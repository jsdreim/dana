macro_rules! impl_mul_unit_scalar {
    ($unit:ty) => {
        impl<V: $crate::Scalar> ::std::ops::Mul<V> for $unit {
            type Output = $crate::Quantity<$unit, V>;

            fn mul(self, rhs: V) -> Self::Output {
                $crate::Quantity {
                    unit: self,
                    value: rhs,
                }
            }
        }
    };
}
