#![allow(unused_imports)]
use dana::{qty, symbols::*};

fn main() {
    let _err = qty![1.0 m^(1/0x00)];
}
