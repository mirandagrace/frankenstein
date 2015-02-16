#![feature(core)]
extern crate frankenstein;
extern crate rand;

use frankenstein::*;
use std::f64::consts::PI;
use std::num::Float;
use rand::distributions;

struct EvolvableFloat(f64);

fn main() {
    let x = EvolvableFloat(1.05);
    print_ln("{}", x);
}