#![allow(unused_imports)]
use dimensional::{qty, symbols::*};

fn main() {
    let _err = qty![1.0 m^(0/0)];
}
