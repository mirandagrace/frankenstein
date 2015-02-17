#![feature(core)]
extern crate frankenstein;
extern crate rand;
use frankenstein::*;
use std::f64::consts::PI;
use std::num::Float;
use rand::distributions::{Normal, IndependentSample};

// In this library we will set up a toy experiment that will evolve f64 numbers towards 
// pi. This is not a particularly interesting example, however it will show the usage of
// most of the library.

// Set up a toy struct, just a wrapper for an f64.
#[derive(Clone, Debug)]
struct EvolvableFloat(f64);

// Since the evolvable trait requires that rand::Rand be implemented on the base type,
// implement Rand.
impl rand::Rand for EvolvableFloat {
    fn rand<R: rand::Rng>(rng: &mut R) -> EvolvableFloat {
        EvolvableFloat(rng.gen::<f64>()*5.0)
    }
}

// Implement the Evolvable trait for EvolvableFloats
impl Evolvable for EvolvableFloat {
    fn mate<R: rand::Rng>(m: &Self, f: &Self, rng: &mut R) -> Self {
        let EvolvableFloat(x) = *m;
        let EvolvableFloat(y) = *f;
        let normal = Normal::new(((x + y) / 2.0), 0.00001);
        let mutated = normal.ind_sample(rng);
        EvolvableFloat(mutated)
    }

    fn fitness(&self) -> f64 {
        let EvolvableFloat(value) = *self;
        1.0 / Float::abs(value - PI)
    }
}

fn main() {
    let x = EvolvableFloat(1.05);
    println!("{:?}", x);
}