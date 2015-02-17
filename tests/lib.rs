#![feature(core)]
extern crate frankenstein;
extern crate rand;
pub use self::frankenstein::*;
pub use std::f64::consts::PI;
pub use std::num::Float;
use self::rand::distributions::*;

#[derive(Clone, Debug)]
pub struct EvolvableFloat {
    value : f64
}

impl EvolvableFloat {
    fn new(value: f64) -> Self {
        EvolvableFloat {value: value}
    }
}

impl rand::Rand for EvolvableFloat {
    fn rand<R: rand::Rng>(rng: &mut R) -> EvolvableFloat {
        EvolvableFloat::new(rng.gen::<f64>()*5.0)
    }
}

impl Evolvable for EvolvableFloat {
    fn mate<R: rand::Rng>(m: &Self, f: &Self, rng: &mut R) -> Self {
        let x = m.value;
        let y = f.value;
        let mean = (x + y) / 2.0;
        let normal = Normal::new(mean, 0.001);
        let mutated = normal.ind_sample(rng);
        EvolvableFloat::new(mutated)
    }

    fn fitness(&self) -> f64 {
        1.0 / Float::abs(self.value - PI)
    }
}

#[test]
fn evolveable_float_fitness_test() {
    let expected = 1.0 / (4.5 - PI) ;
    assert_eq!(EvolvableFloat::new(4.5).fitness(), expected);
}

#[test]
fn experiment_test() {
    let mut my_exp: Experiment<EvolvableFloat, rand::ThreadRng> = Experiment::new(10, 5, rand::thread_rng());
    assert_eq!(my_exp.population.len(), 10);
    let start_score = my_exp.population[0].fitness();
    my_exp.run_until(3, None);
    let end_score = my_exp.population[0].fitness();
    assert!(start_score < end_score, "start_score = {}, end_score = {}", start_score, end_score);
    my_exp.run_until(1000, Some(200.0));
    assert!(my_exp.score() > 200.0, "value = {}", my_exp.population[0].value);
}


