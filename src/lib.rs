#![feature(core)]
extern crate rand;
use std::iter::{FromIterator};
use std::cmp::Ordering;
use std::num::{Float, NumCast};
use rand::distributions::{Weighted, WeightedChoice, IndependentSample};

pub trait Evolvable : rand::Rand + Clone {
    fn fitness(&self) -> f64;
    
    fn mate<R: rand::Rng>(&Self, &Self, &mut R) -> Self;
    
    fn rank(a: &Self, b: &Self) -> Ordering {
        b.fitness().partial_cmp(&a.fitness()).expect("Fitness returned NAN or similar value.")
    }
}

#[derive(Clone)]
pub struct Experiment<T: Evolvable, R: rand::Rng> {
    pub population: Vec<T>,
    death_rate: usize,
    rng: R,
}

impl<T: Evolvable, R: rand::Rng> Experiment<T, R> {
    pub fn new(size: usize, death_rate: usize, mut rng: R) -> Experiment<T, R> {
        let mut population: Vec<T> = Vec::with_capacity(size);
        loop {
            population.push(rand::Rand::rand(&mut rng));
            if population.len() == size { break; }
        }
        population.sort_by(Evolvable::rank);
        Experiment {population: population, death_rate: death_rate, rng: rng}
    }
    
    fn mate(&mut self, mother: usize, father: usize) -> T {
        Evolvable::mate(&self.population[mother], &self.population[father], &mut self.rng)
    }
    
    fn make_weighted(&self, index: usize) -> Weighted<usize> {
        let w = if index < self.death_rate { 
            NumCast::from(Float::ceil(self.population[index].fitness())).expect("Unable to cast fitness to uint. Fitness must be >= 0")
        } else { 0 };
        Weighted { weight: w, item: index }
    }
    
    pub fn result(&self) -> &T {
        &self.population[0]
    }
    
    pub fn run_until(&mut self, max_trials: usize, threshold: Option<f64>) {
        let mut n_trials = 0;
        loop {
            if (n_trials == max_trials || (threshold.is_some() && self.score() >= threshold.unwrap_or(0.0))) { break ; };
            self.trial();
            n_trials += 1;
        }
    }
    
    pub fn score(&self) -> f64 {
        self.population[0].fitness()
    }
    
    pub fn trial(&mut self) {
        let l = self.population.len();
        let mut weighted: Vec<Weighted<usize>> = FromIterator::from_iter((0..l).map(|x| self.make_weighted(x)));
        let wc = WeightedChoice::new(weighted.as_mut_slice());
        let mut children: Vec<T> = Vec::with_capacity(self.population.len());
        loop {
            let mother = wc.ind_sample(&mut self.rng);
            let father = wc.ind_sample(&mut self.rng);
            children.push(self.mate(mother, father));
            if children.len() == self.population.len() { break; }
        }
        children.sort_by(Evolvable::rank);
        self.population = children;
    }
}