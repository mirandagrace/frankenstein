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
    selection: F
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
    
    fn select(&self) -> Vec<&T> {
        let ref f = self.selection;
        f(&self.population)
    }
    
    fn trial(&mut self) {
        let mut children: Vec<T> = Vec::with_capacity(self.population.len());
        loop {
            children.push(Evolvable::mate(self.select()));
            if children.len() == self.population.len() { break; }
        }
        children.sort_by(Evolvable::rank);
        self.population = children;
    }
    
    pub fn run(&mut self, trials: usize) {
        let mut n_trials = 0;
        loop {
            if n_trials == trials { break ; }
            self.trial();
            n_trials += 1;
        }
    }
}