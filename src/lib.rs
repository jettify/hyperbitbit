use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Clone, Copy)]
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

    use super::HyperBitBit;
    use rand::distributions::Alphanumeric;
    use rand::Rng;
    use std::collections::HashSet;

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

        let rng = rand::thread_rng();
        let maxn = 100000;
        for n in 1..=maxn {
            let s = rng.sample_iter(&Alphanumeric).take(3).collect::<String>();

            h.add(&s);
            items.insert(s);
        }
        let expected: i64 = items.len() as i64;
        let rel: f64 = (100.0 * (expected - h.cardinality() as i64) as f64) / (expected as f64);
        println!(
            "cardinality {:?} expected {:?} rel {:?}",
            h.cardinality(),
            items.len(),
            rel
        );
    }
}
