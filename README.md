# frankenstein
frankenstein is a rust library for evolutionary algorithms.

## Evolutionary algorithms 
Evolutionary algorithms are based on the biological idea of survival of the fittest. Most evolutionary algorithms follow a simple procedure. 
   1. A population is randomly initialized. 
    
   2. Each member of this population is then evaluated according to some measurement of fitness. 
    
   3. Two parents are randomly chosen from this population (choices are biased toward more sucessful parents) and combined to create a child.
    
   4. The third step is repeated until the child population is the same size as the original parent population.
   
   5. Steps 2-4 are repeated a certain number of times, or until the fitness function climbs above a certain acceptable threshold. 
    
You provide information about how to evaluate fitness, and how to mate objects. frankenstein will then set up an expiriement for you (this is step 1). Then steps 2-4 can be automated in only a few lines of code. Frankenstein will take care of the rest.

## The Evolvable Trait
Frankenstein is highly customizable and makes almost no assumptions about the kind of creatures you are trying to create, only requiring that the members of your population have implemented the Evolvable trait.  This trait is how you pass along information about your fitness metric, your mating procedure, and your selection process. The Evolvable trait only requires three functions--fitness, mate, and select--to be defined, however it also requires that the Rand trait already be implemented for the object in question.

### Fitness
The signature for the fitness method is:

      fn fitness(&self) -> f64;

This function should take a non-mutable pointer to the instance, and return a score in the form of a f64. Higher scores indicate better fitness. It is worth noting that the fitness function will be called often, so if evaluating your scoring function is going to be expensive, and perfomance is a priority, it might be worth caching the result in an instance field, and only doing a full evaluation once if at all possible. It is also worth knowing, that since the results of this function will be used to compare objects, if fitness returns nan, inf or negative inf values, frankenstein will panic. Additionally, fitness will be used to create the weights for the random selection process, therefore it must only return values >= 0. Values less than zero will cause a panic.

### Mating
The other required method in Evolvable is mate. Its signature is:

      fn mate<R: rand::Rng>(&Self, &Self, &mut R) -> Self;
      
This function should take two non-mutable pointers to instances of whatever your population type is, and return a new instance of your population. The reason it also allows for a random number generator is because a typical implementation would involve taking some set of features from the mother and some set from the father, and then applying some random noise to the child.

### Selection
The final method required by the Evolvable trait is select:

      fn select<R: rand::Rng>(&Vec<Self>, &mut R) -> (usize, usize);
      
This function takes a pointer to a vector of Evolvable objects (The population), and a random number generator, then it returns a pair of usize integers representing the indicies of two members of the population to be mated. The population vector is guarenteed to be sorted, so you simply have to bias your selection function towards creatures that appear earlier in the vector.


## Experiments
The Experiment struct is the core of the frankenstein library. This simple struct has one field, a vector of objects implementing the Evolvable trait:

      pub struct Experiment<T: Evolvable, F: Fn(&Vec<T>) -> Vec<&T>> {
            pub population: Vec<T>,
      }

Note that the population vector is guaranteed to be sorted with the best scoring population members at the front and the worst at the back.

Experiments implements several functions to make running evolutionary algorithms easier. Full example workflows can be found in the examples folders.

### New
To create a new expiriment use the following syntax (experiments need to be mutable).

      let mut my_exp = Experiment::new(size: 10, &mut rng: R);

Size controls the size of the population. The random number generator that you pass in will be used to generate the initial random population. If you pass in a seeded random number generator, you should then be able to replicate your results.

### Result
The result function returns a pointer to the most successful value currently in the population.

      my_exp.result(&self)
      
Since the population is always sorted, this is equivalent to:

      &my_exp.population[0]

If you want to save this value for posterity you will need to copy the data, because the population is all wiped from memory with each step. Trying to hold on to this reference will prevent you from running any more trials.

### Score
The score function simply returns the fitness of the most successful element in the population at that time.

      my_exp.score()
      
This function is equivalent to:

      my_exp.population[0].fitness()

### Trial
This function allows the experiment to run for a single generation, i.e. one pass through steps 2-4 above.

      my_exp.trial(rng, None)
      
This function takes two arguments: a mutable pointer to a random number generator that will control all the randomness involved in the procedure, and an Option<&Fn(&Vec<T:Evolvable>, &mut R)  -> (usize, usize)>. The option allows you to use a selection function other than the one defined in the trait if you so choose. If you pass None, then the selection function defined in the trait implementation.
      
### Run Until
This function takes four arguments: max_trial: usize, rng: &mut R: rand::Rng, threshold: Some<f64>, and fselect: Option<&Fn(&Vec<T:Evolvable>, &mut R)  -> (usize, usize)>. It will run the experiment until either the maximum number of trials has been reached, or the function finds a population member whose fitness is above threshold. Threshold is an Option so that it can be set to None, in which case the experiment will run for the specified number of trials. The rng and fselect are passed through to the trial function as described above.

      
      my_exp.run_until(100, rng, Some(3000.0),
                       Some(&cust_select)): \\ will run for 100 trials, 
                                            \\ or until my_experiment.score() 
                                            \\ is greater than 3000.0, 
                                            \\ whichever comes first. 
      my_exp.run_until(100, rng, None, None); \\ will run for 100 trials

### Possible Future Features
A library of commonly used selection function prototypes.
