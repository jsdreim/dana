use rand::{
    distributions::uniform::{
        SampleUniform,
        UniformSampler,
        SampleBorrow,
    },
    prelude::*,
};
use crate::{Quantity, Scalar, Unit};


impl<U: Unit, V: Scalar + SampleUniform> SampleUniform for Quantity<U, V> {
    type Sampler = UniformQty<U, V>;
}


pub struct UniformQty<U: Unit, V: Scalar + SampleUniform> {
    unit: U,
    sampler: V::Sampler,
}

impl<U: Unit, V: Scalar + SampleUniform> UniformSampler for UniformQty<U, V> {
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


#[cfg(test)]
mod tests {
    use crate::symbols::*;
    use super::*;

    #[test]
    fn test_range() {
        let mut rng = thread_rng();

        let q0 = qty![1.0 m];
        let q1 = qty![200.0 cm];

        for _ in 0..1000 {
            let new = rng.gen_range(q0..q1);

            assert!(
                q0 <= new && new < q1,
                "randomly generated Quantity ({new}) is outside of expected \
                range: [{q0}, {q1})",
            );
        }
    }
}
