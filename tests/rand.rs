#![cfg(feature = "rand")]

use rand::prelude::*;
use dana::{prelude::*, symbols::basic::*};


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
