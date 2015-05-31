extern crate frankenstein;
extern crate rand;
pub use self::frankenstein::*;
pub use std::f64::consts::PI;
use rand::distributions::*;

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
    
    fn select<R: rand::Rng>(population: &Vec<EvolvableFloat>, rng: &mut R) -> (usize, usize) {
        let top_half = population.len()/2;
        let range = Range::new(0, top_half as usize);
        let x = range.ind_sample(rng);
        let y = range.ind_sample(rng);
        (x, y)
    }

    fn fitness(&self) -> f64 {
        1.0 / (self.value - PI).abs()
    }
}

#[test]
fn evolveable_float_fitness_test() {
    let expected = 1.0 / (4.5 - PI) ;
    assert_eq!(EvolvableFloat::new(4.5).fitness(), expected);
}

fn custom_select<R: rand::Rng>(population: &Vec<EvolvableFloat>, rng: &mut R) -> (usize, usize){
    let top_quarter = population.len()/4;
    let range = Range::new(0, top_quarter as usize);
    let x = range.ind_sample(rng);
    let y = range.ind_sample(rng);
    (x, y)
}

#[test]
fn experiment_test() {
    let mut rng = rand::thread_rng();
    let mut my_exp: Experiment<EvolvableFloat> = Experiment::new(10, &mut rng);
    assert_eq!(my_exp.population.len(), 10);
    let start_score = my_exp.population[0].fitness();
    my_exp.run_until(3, &mut rng, None, None);
    let end_score = my_exp.population[0].fitness();
    assert!(start_score < end_score, "start_score = {}, end_score = {}", start_score, end_score);
    my_exp.run_until(1000, &mut rng, Some(200.0), None);
    assert!(my_exp.score() > 200.0, "value = {}", my_exp.population[0].value);
}

#[test]
fn experiment_test_cust_select() {
    let mut rng = rand::thread_rng();
    let mut my_exp: Experiment<EvolvableFloat> = Experiment::new(10, &mut rng);
    let start_score = my_exp.population[0].fitness();
    my_exp.run_until(3, &mut rng, None, Some(&custom_select));
    let end_score = my_exp.population[0].fitness();
    assert!(start_score < end_score, "start_score = {}, end_score = {}", start_score, end_score);
    my_exp.run_until(1000, &mut rng, Some(200.0), Some(&custom_select));
    assert!(my_exp.score() > 200.0, "value = {}", my_exp.population[0].value);
}

