extern crate hyperbitbit;
extern crate rand;

use hyperbitbit::HyperBitBit;
use rand::distributions::Alphanumeric;
use rand::prelude::*;
use std::collections::HashSet;

fn main() {
    let mut hbb = HyperBitBit::new();
    let mut items = HashSet::new();
    let mut rng = StdRng::seed_from_u64(42);

    let maxn = 10000;
    for _ in 1..maxn {
        let s = (&mut rng).sample_iter(&Alphanumeric).take(4).collect::<String>();

        hbb.add(&s);
        items.insert(s);
    }
    let expected: i64 = items.len() as i64;
    let rel: f64 = (100.0 * (expected - hbb.cardinality() as i64) as f64) / (expected as f64);

    println!("Actuals cardinality:   {:?}", expected);
    println!("Estimated cardinality: {:?}", hbb.cardinality());
    println!("Error % cardinality:   {:.2}", rel);
}
