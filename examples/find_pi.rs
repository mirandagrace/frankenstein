extern crate frankenstein;
extern crate rand;
use frankenstein::*;
use std::f64::consts::PI;
use rand::{Rng, Rand, SeedableRng, StdRng};
use rand::distributions::{Normal, IndependentSample, Range};

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
    // Mating two evolvable floats is taking their mean and applying normally distributed 
    // random noise.
    fn mate<R: rand::Rng>(m: &Self, f: &Self, rng: &mut R) -> Self {
        let EvolvableFloat(x) = *m;
        let EvolvableFloat(y) = *f;
        let normal = Normal::new(((x + y) / 2.0), 0.0001);
        let mutated = normal.ind_sample(rng);
        EvolvableFloat(mutated)
    }


    // 
    fn fitness(&self) -> f64 {
        let EvolvableFloat(value) = *self;
        1.0 / (value - PI).abs()
    }
    
     fn select<R: rand::Rng>(population: &Vec<EvolvableFloat>, rng: &mut R) -> (usize, usize) {
        let top_half = population.len()/2;
        let range = Range::new(0, top_half as usize);
        let x = range.ind_sample(rng);
        let y = range.ind_sample(rng);
        (x, y)
    }
}

fn main() {
    // set up and seed the random number generator
    let seed = [31, 4, 1, 5, 1, 3, 1, 8, 1, 1, 9, 3, 8, 7, 3, 7, 9, 2, 3, 7, 96, 234];
    let mut rng: StdRng = SeedableRng::from_seed(&seed[0..10]);
    
    // create a new experiment
    let mut my_exp: Experiment<EvolvableFloat> = Experiment::new(15, &mut rng);
    
    // run for one generation
    my_exp.trial(&mut rng, None);
    
    //print results
    println!("After 1 trial, the best result is {:?}. Its score is {:?}", my_exp.population[0], my_exp.score());
    
    //run for four generations
    my_exp.run_until(4, &mut rng, None, None);
    
    // print results
    println!("After 5 trials, the best result is {:?}. Its score is {:?}", my_exp.population[0], my_exp.score());
    
    // run until the score is better than 200, with a max of 1000 trials
    my_exp.run_until(1000, &mut rng, Some(10000.0), None);
    println!("The best result is {:?}. Its score is {:?}", my_exp.population[0], my_exp.score());
    
}