use std::{
    array::from_fn,
    marker::PhantomData,
    ops::{Add, Div, Mul, Neg, Sub},
    simd::*,
};
use crate::{Quantity, Scalar, units::{*, traits::*, unit_anon::UnitAnon}};


#[derive(Clone, Copy, Debug, PartialEq)]
pub struct QtySimd<U, V, const N: usize> where
    LaneCount<N>: SupportedLaneCount,
    U: Unit, V: Scalar + SimdElement,
{
    pub values: Simd<V, N>,
    pub scales: Simd<f64, N>,
    pub _u: PhantomData<U>,
}


impl<U, V, const N: usize> QtySimd<U, V, N> where
    LaneCount<N>: SupportedLaneCount,
    Simd<V, N>: Mul<Output=Simd<V, N>>,
    U: Unit, V: Scalar + SimdElement,
{
    pub fn get(&self, index: usize) -> Quantity<UnitAnon<U::Dim>, V> {
        let value = self.values[index];
        let unit = UnitAnon::new(self.scales[index]);

        unit.quantity(value)
    }

    pub fn get_as<W: Unit>(&self, index: usize, unit: W) -> Quantity<W, V> where
        UnitAnon<U::Dim>: ConvertInto<W>,
    {
        self.get(index).convert_to(unit)
    }

    pub fn rescale_values(&self, other: Simd<f64, N>) -> Simd<V, N> where
        Simd<V, N>: Mul<Output=Simd<V, N>>,
    {
        let rel: Simd<f64, N> = self.scales / other;
        let coeff: Simd<V, N> = Simd::from(from_fn(|n| V::from_f64(rel[n]).unwrap()));

        self.values * coeff
    }
}


//region `From` impls.
impl<U, V, const N: usize> From<Quantity<U, V>> for QtySimd<U, V, N> where
    LaneCount<N>: SupportedLaneCount,
    U: Unit, V: Scalar + SimdElement,
{
    fn from(input: Quantity<U, V>) -> Self {
        let values: [V; N] = [input.value; N];
        let scales: [f64; N] = [input.unit.scale(); N];

        Self {
            values: Simd::from(values),
            scales: Simd::from(scales),
            _u: PhantomData,
        }
    }
}

impl<U, V, const N: usize> From<[Quantity<U, V>; N]> for QtySimd<U, V, N> where
    LaneCount<N>: SupportedLaneCount,
    U: Unit, V: Scalar + SimdElement,
{
    fn from(input: [Quantity<U, V>; N]) -> Self {
        let values: [V; N] = from_fn(|n| input[n].value);
        let scales: [f64; N] = from_fn(|n| input[n].unit.scale());

        Self {
            values: Simd::from(values),
            scales: Simd::from(scales),
            _u: PhantomData,
        }
    }
}
//endregion


//region Mathematical operators.
//region Negation.
impl<U, V, const N: usize> Neg for QtySimd<U, V, N> where
    LaneCount<N>: SupportedLaneCount,
    Simd<V, N>: Neg<Output=Simd<V, N>>,
    U: Unit,
    V: Scalar + SimdElement,
{
    type Output = QtySimd<U, V, N>;

    fn neg(self) -> Self::Output {
        QtySimd {
            values: self.values.neg(),
            scales: self.scales,
            _u: PhantomData,
        }
    }
}
//endregion

//region Add/Sub.
impl<U, V, W, const N: usize> Add<QtySimd<W, V, N>> for QtySimd<U, V, N> where
    LaneCount<N>: SupportedLaneCount,
    Simd<V, N>: Add<Output=Simd<V, N>>,
    Simd<V, N>: Mul<Output=Simd<V, N>>,
    U: Unit, W: Unit<Dim=U::Dim>,
    V: Scalar + SimdElement,
{
    type Output = QtySimd<U, V, N>;

    fn add(self, rhs: QtySimd<W, V, N>) -> Self::Output {
        QtySimd {
            values: self.values + rhs.rescale_values(self.scales),
            scales: self.scales,
            _u: PhantomData,
        }
    }
}

impl<U, V, W, const N: usize> Sub<QtySimd<W, V, N>> for QtySimd<U, V, N> where
    LaneCount<N>: SupportedLaneCount,
    Simd<V, N>: Sub<Output=Simd<V, N>>,
    Simd<V, N>: Mul<Output=Simd<V, N>>,
    U: Unit, W: Unit<Dim=U::Dim>,
    V: Scalar + SimdElement,
{
    type Output = QtySimd<U, V, N>;

    fn sub(self, rhs: QtySimd<W, V, N>) -> Self::Output {
        QtySimd {
            values: self.values - rhs.rescale_values(self.scales),
            scales: self.scales,
            _u: PhantomData,
        }
    }
}
//endregion

//region Div/Mul.
impl<U, V, W, const N: usize> Div<QtySimd<W, V, N>> for QtySimd<U, V, N> where
    LaneCount<N>: SupportedLaneCount,
    Simd<V, N>: Div<Output=Simd<V, N>>,
    Simd<V, N>: Mul<Output=Simd<V, N>>,
    U: Unit + Div<W>, W: Unit,
    U::Output: Unit,
    V: Scalar + SimdElement,
{
    type Output = QtySimd<U::Output, V, N>;

    fn div(self, rhs: QtySimd<W, V, N>) -> Self::Output {
        QtySimd {
            values: self.values / rhs.values,
            scales: self.scales / rhs.scales,
            _u: PhantomData,
        }
    }
}

impl<U, V, W, const N: usize> Mul<QtySimd<W, V, N>> for QtySimd<U, V, N> where
    LaneCount<N>: SupportedLaneCount,
    Simd<V, N>: Mul<Output=Simd<V, N>>,
    U: Unit + Mul<W>, W: Unit,
    U::Output: Unit,
    V: Scalar + SimdElement,
{
    type Output = QtySimd<U::Output, V, N>;

    fn mul(self, rhs: QtySimd<W, V, N>) -> Self::Output {
        QtySimd {
            values: self.values * rhs.values,
            scales: self.scales * rhs.scales,
            _u: PhantomData,
        }
    }
}
//endregion

//region Div/Mul with single Quantity.
impl<U, V, W, X, const N: usize> Div<Quantity<W, X>> for QtySimd<U, V, N> where
    LaneCount<N>: SupportedLaneCount,
    U: Unit, V: Scalar + SimdElement,
    W: Unit, X: Scalar + SimdElement,
    Self: Div<QtySimd<W, X, N>>,
{
    type Output = <Self as Div<QtySimd<W, X, N>>>::Output;

    fn div(self, rhs: Quantity<W, X>) -> Self::Output {
        self / QtySimd::from(rhs)
    }
}

impl<U, V, W, X, const N: usize> Mul<Quantity<W, X>> for QtySimd<U, V, N> where
    LaneCount<N>: SupportedLaneCount,
    U: Unit, V: Scalar + SimdElement,
    W: Unit, X: Scalar + SimdElement,
    Self: Mul<QtySimd<W, X, N>>,
{
    type Output = <Self as Mul<QtySimd<W, X, N>>>::Output;

    fn mul(self, rhs: Quantity<W, X>) -> Self::Output {
        self * QtySimd::from(rhs)
    }
}
//endregion

//region Div/Mul with anything the inner simd can div/mul with.
impl<U, V, T, const N: usize> Div<T> for QtySimd<U, V, N> where
    LaneCount<N>: SupportedLaneCount,
    Simd<V, N>: Div<T, Output=Simd<V, N>>,
    U: Unit, V: Scalar + SimdElement,
{
    type Output = QtySimd<U, V, N>;

    fn div(self, rhs: T) -> Self::Output {
        QtySimd {
            values: self.values / rhs,
            scales: self.scales,
            _u: PhantomData,
        }
    }
}

impl<U, V, T, const N: usize> Mul<T> for QtySimd<U, V, N> where
    LaneCount<N>: SupportedLaneCount,
    Simd<V, N>: Mul<T, Output=Simd<V, N>>,
    U: Unit, V: Scalar + SimdElement,
{
    type Output = QtySimd<U, V, N>;

    fn mul(self, rhs: T) -> Self::Output {
        QtySimd {
            values: self.values * rhs,
            scales: self.scales,
            _u: PhantomData,
        }
    }
}
//endregion
//endregion


#[cfg(test)]
mod tests {
    use crate::{qty, units::symbols::*};
    use super::*;

    #[test]
    fn qty_simd_add() {
        let l1 = QtySimd::from([
            qty![1.0 km],
            qty![50.0 m],
        ]);
        let l2 = QtySimd::from([
            qty![25.0 m],
            qty![2.0 km],
        ]);

        let sum = l1 + l2;
        assert_eq!(sum.get(0), qty![1025.0 m]);
        assert_eq!(sum.get(1), qty![2050.0 m]);
    }

    #[test]
    fn qty_simd_div() {
        let array_l = QtySimd::from([
            qty![12.0 m],
            qty![24.0 m],
            qty![36.0 m],
            qty![48.0 m],
        ]);
        let array_t = QtySimd::from([
            qty![6.0 s],
            qty![2.0 s],
            qty![4.0 s],
            qty![3.0 s],
        ]);
        let array_v = array_l / array_t;

        assert_eq!(array_v.get(0), qty![ 2.0 m/s]);
        assert_eq!(array_v.get(1), qty![12.0 m/s]);
        assert_eq!(array_v.get(2), qty![ 9.0 m/s]);
        assert_eq!(array_v.get(3), qty![16.0 m/s]);
    }

    #[test]
    fn qty_simd_add_mul() {
        let positions = QtySimd::from([
            qty![10.0 m],
            qty![12.0 m],
            qty![14.0 m],
            qty![16.0 m],
        ]);
        let velocities = QtySimd::from([
            qty![4.0 m/s],
            qty![3.0 m/s],
            qty![2.0 m/s],
            qty![1.0 m/s],
        ]);
        let time = qty![2.0 s];

        let pos_new = positions + velocities * time;

        assert_eq!(pos_new.get(0), qty![18.0 m]);
        assert_eq!(pos_new.get(1), qty![18.0 m]);
        assert_eq!(pos_new.get(2), qty![18.0 m]);
        assert_eq!(pos_new.get(3), qty![18.0 m]);
    }
}
