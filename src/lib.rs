extern crate rand;
use std::cmp::Ordering;

pub trait Evolvable : rand::Rand + Clone {
    fn mate(Vec<&Self>) -> Self;
    
    fn fitness(&self) -> f64;
    
    fn rank(a: &Self, b: &Self) -> Ordering {
        b.fitness().partial_cmp(&a.fitness()).expect("Fitness returned NAN or similar value.")
    }
}

#[derive(Clone)]
pub struct Experiment<T: Evolvable, F: Fn(&Vec<T>) -> Vec<&T>> {
    pub population: Vec<T>,
    selection: F,
}

impl<T: Evolvable, F: Fn(&Vec<T>) -> Vec<&T>> Experiment<T, F> {
    pub fn new(size: usize, selection: F) -> Experiment<T, F> {
        let mut population: Vec<T> = Vec::with_capacity(size);
        loop {
            population.push(rand::random());
            if population.len() == size { break; }
        }
        population.sort_by(Evolvable::rank);
        Experiment {population: population, selection: selection}
    }
    
    pub fn reset(&mut self) {
        let size = self.population.len();
        self.population.clear();
        loop {
            self.population.push(rand::random());
            if self.population.len() == size { break; }
        }
    }
    
    pub fn result(&self) -> &T {
        &self.population[0]
    }
    
    pub fn run_fitness(&mut self, threshold: f64){
        loop {
            if self.score() > threshold { break; }
            self.trial();
        }
    }
    
    pub fn run_trials(&mut self, trials: usize) {
        let mut n_trials = 0;
        loop {
            if n_trials == trials { break ; }
            self.trial();
            n_trials += 1;
        }
    }
    
    pub fn run_until(&mut self, trials: usize, threshold: f64) {
        let mut n_trials = 0;
        loop {
            if n_trials == trials || self.score() > threshold { break ; }
            self.trial();
            n_trials += 1;
        }
    }
    
    pub fn score(&self) -> f64 {
        self.population[0].fitness()
    }
    
    fn select(&self) -> Vec<&T> {
        let ref f = self.selection;
        f(&self.population)
    }
    
    pub fn trial(&mut self) {
        let mut children: Vec<T> = Vec::with_capacity(self.population.len());
        loop {
            children.push(Evolvable::mate(self.select()));
            if children.len() == self.population.len() { break; }
        }
        children.sort_by(Evolvable::rank);
        self.population = children;
    }
}