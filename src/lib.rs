use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
#[cfg(feature = "serde")]
use serde_crate::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize),
    serde(crate = "serde_crate")
)]
pub struct HyperBitBit {
    lgn: u8,
    sketch1: u64,
    sketch2: u64,
}

impl Default for HyperBitBit {
    fn default() -> HyperBitBit {
        HyperBitBit {
            lgn: 5,
            sketch1: 0,
            sketch2: 0,
        }
    }
}

impl HyperBitBit {
    pub fn new() -> HyperBitBit {
        Default::default()
    }

    pub fn cardinality(&self) -> u64 {
        let exponent: f64 = self.lgn as f64 + 5.4 + (self.sketch1.count_ones() as f64) / 32.0;
        return f64::powf(2.0, exponent) as u64;
    }

    pub fn add(&mut self, v: &String) {
        let mut hasher = DefaultHasher::new();
        v.hash(&mut hasher);
        let hash_val: u64 = hasher.finish();

        let k: u64 = (hash_val << 58) >> 58;
        let r: u64 = ((hash_val >> 6).leading_zeros() - 6).into();

        if r > self.lgn.into() {
            self.sketch1 = self.sketch1 | ((1 as u64) << k)
        }

        if r > (self.lgn + 1).into() {
            self.sketch2 = self.sketch2 | ((1 as u64) << k)
        }
        if self.sketch1.count_ones() > 31 {
            self.sketch1 = self.sketch2;
            self.sketch2 = 0;
            self.lgn += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate rand;

    use rand::distributions::Alphanumeric;
    use rand::prelude::*;
    use std::collections::HashSet;
    use super::HyperBitBit;

    #[test]
    fn test_basic() {
        let mut h = HyperBitBit::new();
        assert_eq!(1351, h.cardinality());
        h.add(&String::from("xxx"));
        h.add(&String::from("yyy"));
        assert_eq!(1351, h.cardinality());
    }

    #[test]
    fn test_cardinality() {
        let mut h = HyperBitBit::new();
        let mut items = HashSet::new();

        assert_eq!(1351, h.cardinality());

        let mut rng = StdRng::seed_from_u64(42);
        let maxn = 10000;
        for _ in 1..=maxn {
            let s = (&mut rng).sample_iter(&Alphanumeric).take(4).collect::<String>();

            h.add(&s);
            items.insert(s);
        }
        let expected: i64 = items.len() as i64;
        let rel: f64 = (100.0 * (expected - h.cardinality() as i64) as f64) / (expected as f64);
        assert!(rel < 10.0);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serde() {
        let mut h = HyperBitBit::new();
        h.insert("foo");

        let serialized_h = bincode::serialize(&h).unwrap();
        let other_h: HyperBitBit = bincode::deserialize(&serialized_h).unwrap();

        assert_eq!(h.cardinality(), other_h.cardinality());
        assert_eq!(h.sketch1, other_h.sketch1);
        assert_eq!(h.sketch2, other_h.sketch2);
    }
}
