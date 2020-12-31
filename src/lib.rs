//! A native rust implementation of a HyperBitBit algorithm introduced by
//! by Robert Sedgewick in [AC11-Cardinality.pdf](https://www.cs.princeton.edu/~rs/talks/AC11-Cardinality.pdf)
//!
//! # Feature
//! * HyperBitBit, for N < 2^64
//! * Uses 128 + 8 bit of space
//! * Estimated cardinality withing 10% or actuals for large N.
//!
//! Consider HyperLogLog variants for productions usage, sine this data structure
//! extensively studied, merge able and more accurate. HyperBitBit is extremely
//! cheap and fast alternative in expense of accuracy.
//!
//! # Usage
//!
//! This crate is [on crates.io](https://crates.io/crates/hyperbitbit) and
//! can be used by adding `hyperbitbit` to the dependencies in your
//! project's `Cargo.toml`.
//!
//! ```toml
//! [dependencies]
//! hyperbitbit = "0.0.1-alpha.1"
//! ```
//! If you want [serde](https://github.com/serde-rs/serde) support, include the feature like this:
//!
//! ```toml
//! [dependencies]
//! hyperbitbit = { version = "0.0.1-alpha.1", features = ["serde_support"] }
//! ```
//!
//! add this to your crate root:
//!
//! ```rust
//! extern crate hyperbitbit;
//! ```
//!
//! # Example
//!
//! Create a HyperBitBit, add more data and estimate cardinality
//!
//! ```rust
//! use hyperbitbit::HyperBitBit;
//! use rand::Rng;
//! use rand::SeedableRng;
//! use rand::distributions::Alphanumeric;
//! use rand_isaac::Isaac64Rng;
//! use std::collections::HashSet;
//!
//!
//! fn main() {
//!     let mut hbb = HyperBitBit::new();
//!     let mut items = HashSet::new();
//!     let mut rng = Isaac64Rng::seed_from_u64(42);
//!
//!     let maxn = 100000;
//!     for _ in 1..maxn {
//!         let s = (&mut rng).sample_iter(&Alphanumeric).take(3).collect::<String>();
//!
//!         hbb.insert(&s);
//!         items.insert(s);
//!     }
//!     let expected: i64 = items.len() as i64;
//!     let rel: f64 = (100.0 * (expected - hbb.cardinality() as i64) as f64) / (expected as f64);
//!
//!     println!("Cardinality:           {:?}", expected);
//!     println!("Estimated cardinality: {:?}", hbb.cardinality());
//!     println!("Error % cardinality:   {:.2}", rel);
//! }
//!```
//! # Lincese
//!  Licensed under the Apache License, Version 2.0


use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[cfg(feature="serde_support")]
extern crate serde;

#[cfg(feature="serde_support")]
use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
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
    /// create a new HyperBitBit struct
    ///
    /// # Example
    /// ```
    /// # use hyperbitbit::HyperBitBit;
    /// let mut h = HyperBitBit::new();
    /// ```
    pub fn new() -> HyperBitBit {
        Default::default()
    }

    /// estimate cardinality
    ///
    /// # Example
    /// ```
    /// # use hyperbitbit::HyperBitBit;
    /// let mut h = HyperBitBit::new();
    /// h.insert(&String::from("xxx"));
    /// println!("{}", h.cardinality());
    /// ```
    pub fn cardinality(&self) -> u64 {
        let exponent: f64 = self.lgn as f64 + 5.4 + (self.sketch1.count_ones() as f64) / 32.0;
        f64::powf(2.0, exponent) as u64
    }

    /// add string to HyperBitBit
    ///
    /// # Example
    /// ```
    /// # use hyperbitbit::HyperBitBit;
    /// let mut h = HyperBitBit::new();
    /// h.insert(&String::from("xxx"));
    /// ```
    pub fn insert(&mut self, v: &str) {
        let mut hasher = DefaultHasher::new();
        v.hash(&mut hasher);
        let hash_val: u64 = hasher.finish();

        let k: u64 = (hash_val << 58) >> 58;
        let r: u64 = ((hash_val >> 6).leading_zeros() - 6).into();

        if r > self.lgn.into() {
            self.sketch1 |= 1_u64 << k
        }

        if r > (self.lgn + 1).into() {
            self.sketch2 |= 1_u64 << k
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
    extern crate serde_json;

    use rand::distributions::Alphanumeric;
    use rand::Rng;
    use rand::SeedableRng;
    use rand_isaac::Isaac64Rng;
    use std::collections::HashSet;
    use super::HyperBitBit;

    #[test]
    fn test_basic() {
        let mut h = HyperBitBit::new();
        // HyperBitBit is not working for small cardinalities
        assert_eq!(1351, h.cardinality());
        h.insert(&String::from("xxx"));
        h.insert(&String::from("yyy"));
        assert_eq!(1351, h.cardinality());
    }

    #[test]
    fn test_cardinality() {
        let mut h = HyperBitBit::new();
        let mut items = HashSet::new();

        assert_eq!(1351, h.cardinality());

        let mut rng = Isaac64Rng::seed_from_u64(42);
        let maxn = 10000;
        for _ in 1..=maxn {
            let s = (&mut rng).sample_iter(&Alphanumeric).take(2).collect::<String>();

            h.insert(&s);
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
        h.insert(&String::from("xxx"));

        let serialized_h = serde_json::to_string(&h).unwrap();
        let other_h: HyperBitBit = serde_json::from_str(&serialized_h).unwrap();

        assert_eq!(h.cardinality(), other_h.cardinality());
        assert_eq!(h.sketch1, other_h.sketch1);
        assert_eq!(h.sketch2, other_h.sketch2);
        assert_eq!(h.lgn, other_h.lgn);
    }
}
