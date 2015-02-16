#![feature(core)]
extern crate frankenstein;
extern crate rand;

#[cfg(test)]
mod test {
    pub use frankenstein::*;
    pub use std::f64::consts::PI;
    pub use std::num::Float;
    pub use rand;
    use rand::distributions::*;
    
    mod test_helpers { 
    
        #[derive(Clone, Debug)]
        pub struct EvolvableFloat {
            value : f64
        }
    
        pub fn select(population: &Vec<EvolvableFloat>) -> Vec<&EvolvableFloat> {
            let mut range = Range::new(0, population.len()/2);
            let mut parents: Vec<&EvolvableFloat> = Vec::with_capacity(2);
            let x = range.sample(&mut rand::thread_rng());
            let y = range.sample(&mut rand::thread_rng());
            parents.push(&population[x]);
            parents.push(&population[y]);
            parents
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
            fn mate(parents: Vec<&Self>) -> Self {
                let x = parents[0].value;
                let y = parents[1].value;
                let mean = (x + y) / 2.0;
                let normal = Normal::new(mean, 0.00001);
                let mutated = normal.ind_sample(&mut rand::thread_rng());
                EvolvableFloat::new(mutated)
            }
        
            fn fitness(&self) -> f64 {
                2.0 / ((self.value - PI) * (self.value - PI))
            }
        }
    }
        
    #[test]
    fn evolveable_float_mate_test() {
        let x = EvolvableFloat::new(4.0);
        let y = EvolvableFloat::new(5.0);
        assert!(Float::abs(Evolvable::mate(vec![&x, &y]).value - 4.5) < 0.1);
    }
    
    #[test]
    fn evolveable_float_fitness_test() {
        let expected = 2.0 / ((4.5 - PI) * (4.5 - PI)) ;
        assert_eq!(EvolvableFloat::new(4.5).fitness(), expected);
    }
    
    #[test]
    fn experiment_test() {
        let mut my_exp = Experiment::new(10, select);
        assert_eq!(my_exp.population.len(), 10);
        let start_score = my_exp.population[0].fitness();
        my_exp.run(2);
        let end_score = my_exp.population[0].fitness();
        assert!(start_score < end_score, "start_score = {}, end_score = {}", start_score, end_score);
        my_exp.run(10);
        assert!(Float::abs(my_exp.population[0].value - PI) < 0.1, "value = {}", my_exp.population[0].value);
    }
}


