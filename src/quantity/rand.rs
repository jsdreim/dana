//! Support for [`rand`] sampling traits.

use rand::{
    distributions::uniform::{
        SampleUniform,
        UniformSampler,
        SampleBorrow,
    },
    prelude::*,
};
use crate::{Quantity, Unit, Value};


impl<U: Unit, V: Value + SampleUniform> SampleUniform for Quantity<U, V> {
    type Sampler = UniformQty<U, V>;
}


/// The back-end implementing [`UniformSampler`] for [`Quantity`].
///
/// Works for any `Quantity<U, V>` where `V` implements [`SampleUniform`].
pub struct UniformQty<U: Unit, V: Value + SampleUniform> {
    unit: U,
    sampler: V::Sampler,
}

impl<U: Unit, V: Value + SampleUniform> UniformSampler for UniformQty<U, V> {
    type X = Quantity<U, V>;

    fn new<B1, B2>(low: B1, high: B2) -> Self where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        let unit = low.borrow().unit;
        let sampler = V::Sampler::new(
            low.borrow().value.clone(),
            high.borrow().clone().value_as(unit),
        );

        Self { unit, sampler }
    }

    fn new_inclusive<B1, B2>(low: B1, high: B2) -> Self where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        let unit = low.borrow().unit;
        let sampler = V::Sampler::new_inclusive(
            low.borrow().value.clone(),
            high.borrow().clone().value_as(unit),
        );

        Self { unit, sampler }
    }

    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Self::X {
        self.unit.quantity(self.sampler.sample(rng))
    }
}
