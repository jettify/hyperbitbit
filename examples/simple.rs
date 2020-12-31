use hyperbitbit::HyperBitBit;
use rand::Rng;
use rand::SeedableRng;
use rand::distributions::Alphanumeric;
use rand_isaac::Isaac64Rng;
use std::collections::HashSet;


fn main() {
    let mut hbb = HyperBitBit::new();
    let mut items = HashSet::new();
    let mut rng = Isaac64Rng::seed_from_u64(42);

    let maxn = 100000;
    for _ in 1..maxn {
        let s = (&mut rng).sample_iter(&Alphanumeric).take(3).collect::<String>();

        hbb.insert(&s);
        items.insert(s);
    }
    let expected: i64 = items.len() as i64;
    let rel: f64 = (100.0 * (expected - hbb.cardinality() as i64) as f64) / (expected as f64);

    println!("Cardinality:           {:?}", expected);
    println!("Estimated cardinality: {:?}", hbb.cardinality());
    println!("Error % cardinality:   {:.2}", rel);
}
