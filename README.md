# HyperBitBit
[![ci-badge](https://github.com/jettify/hyperbitbit/workflows/CI/badge.svg)](https://github.com/jettify/hyperbitbit/actions?query=workflow%3ACI)
[![Crates.io](https://img.shields.io/crates/v/hyperbitbit.svg)](https://crates.io/crates/hyperbitbit)
[![Documentation](https://docs.rs/hyperbitbit/badge.svg)](https://docs.rs/hyperbitbit/)

 A native rust implementation of a HyperBitBit algorithm introduced by
 by Robert Sedgewick in [AC11-Cardinality.pdf](https://www.cs.princeton.edu/~rs/talks/AC11-Cardinality.pdf)

 # Feature
 * HyperBitBit, for N < 2^64
 * Uses 128 + 8 bit of space
 * Estimated cardinality withing 10% or actuals for large N.

 Consider HyperLogLog variants for productions usage, sine this data structure
 extensively studied, merge able and more accurate. HyperBitBit is extremely
 cheap and fast alternative in expense of accuracy.

 # Usage

 This crate is [on crates.io](https://crates.io/crates/hyperbitbit) and
 can be used by adding `hyperbitbit` to the dependencies in your
 project's `Cargo.toml`.

 ```toml
 [dependencies]
 hyperbitbit = "0.0.1-alpha.1"
 ```
 If you want [serde](https://github.com/serde-rs/serde) support, include the feature like this:

 ```toml
 [dependencies]
 hyperbitbit = { version = "0.0.1-alpha.1", features = ["serde_support"] }
 ```

 add this to your crate root:

 ```rust
 extern crate hyperbitbit;
 ```

 # Example

 Create a HyperBitBit, add more data and estimate cardinality

 ```rust
 use hyperbitbit::HyperBitBit;
 use rand::distributions::Alphanumeric;
 use rand::prelude::*;
 use std::collections::HashSet;

 fn main() {
     // create hbb for cardinality estimation
     let mut hbb = HyperBitBit::new();
     // hash set to measure exact cardinality
     let mut items = HashSet::new();
     // fixed seed for RNG for reproducibly results
     let mut rng = StdRng::seed_from_u64(42);
     // put bunch of random strings on hbb and hash set for comparison
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
```
 # Lincese
  Licensed under the Apache License, Version 2.0
