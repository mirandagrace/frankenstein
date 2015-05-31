extern crate rand;
//use std::iter::{FromIterator};
use std::cmp::Ordering;
//use rand::distributions::{Weighted, WeightedChoice, IndependentSample};


pub trait Evolvable : rand::Rand + Clone {
    fn fitness(&self) -> f64;
    
    fn mate<R: rand::Rng>(&Self, &Self, &mut R) -> Self;
    
    fn select<R: rand::Rng>(&Vec<Self>, &mut R) -> (usize, usize);
    
    fn rank(a: &Self, b: &Self) -> Ordering {
        b.fitness().partial_cmp(&a.fitness()).expect("Fitness returned NAN or similar value.")
    }
}

pub type Selection<T: Evolvable, R: rand::Rng> = Fn(&Vec<T>, &mut R)  -> (usize, usize);

pub struct Experiment<T: Evolvable> {
    pub population: Vec<T>,
}

impl<T: Evolvable> Experiment<T> {
    pub fn new<R: rand::Rng>(size: usize, rng: &mut R) -> Experiment<T> {
        let mut population: Vec<T> = Vec::with_capacity(size);
        for _ in 0..size {
            population.push(rand::Rand::rand(rng));
        }
        population.sort_by(Evolvable::rank);
        Experiment {population: population}
    }
    
    fn mate<R: rand::Rng>(&mut self, mother: usize, father: usize, rng: &mut R) -> T {
        Evolvable::mate(&self.population[mother], &self.population[father], rng)
    }
    
    pub fn result(&self) -> &T {
        &self.population[0]
    }
    
    pub fn run_until<R: rand::Rng>(&mut self, max_trials: usize, rng: &mut R,
                                   threshold: Option<f64>,
                                   custom_select: Option<&Selection<T, R>>) {
        let mut n_trials = 0;
        loop {
            if n_trials == max_trials || (threshold.is_some() 
              && self.score() >= threshold.unwrap_or(0.0)) { break ; };
            self.trial(rng, custom_select);
            n_trials += 1;
        }
    }
    
    pub fn score(&self) -> f64 {
        self.population[0].fitness()
    }
    
    pub fn trial<R: rand::Rng>(&mut self, rng: &mut R, custom_select: Option<&Selection<T, R>>) {
        let l = self.population.len();
        let mut children: Vec<T> = Vec::with_capacity(self.population.len());
        for _ in 0..l {
            let (mother, father) = match custom_select {
                Some(f) => f(&self.population, rng),
                None => Evolvable::select(&self.population, rng)};
            children.push(self.mate(mother, father, rng));
        }
        children.sort_by(Evolvable::rank);
        self.population = children;
    }
}