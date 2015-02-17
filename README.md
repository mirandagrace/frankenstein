# frankenstein
frankenstein is a rust library for evolutionary algorithms.

## Evolutionary algorithms 
Evolutionary algorithms are based on the biological idea of survival of the fittest. Most evolutionary algorithms follow a simple procedure. 
   1. A population is randomly initialized. 
    
   2. Each member of this population is then evaluated according to some measurement of fitness. 
    
   3. Via some selection process (usually biased towards the most successful members of the population), parents are chosen and 'mated' or combined to create a child.
    
   4. The third step is repeated until the child population is the same size as the original parent population.
   
   5. Steps 2-4 are repeated a certain number of times, or until the fitness function climbs above a certain acceptable threshold. 
    
You provide information about how to evaluate fitness, how to 'mate' objects, and a selection process. frankenstein will then set up an expiriement for you this is step 1. Then steps 2-4 can be automated in only a lines.

## The Evolvable Trait
Frankenstein is highly customizable and makes almost no assumptions about the kind of creatures you are trying to create, only requiring that the members of your population have implemented the Evolvable trait.  This trait is how you pass along information about your fitness metric as well as your mating procedure. The Evolvable trait only requires two functions--fitness and mate--to be defined, however it also requires that the Rand trait already be implemented for the object in question.

### Fitness
The signature for the fitness method is:

      fn fitness(&self) -> f64;

This function should take a non-mutable pointer to the instance, and return a score in the form of a f64. Higher scores indicate better fitness. It is worth noting that the fitness function will be called often, so if evaluating your scoring function is going to be expensive, and perfomance is a priority, it might be worth caching the result in an instance field, and only doing a full evaluation once if at all possible. It is also worth knowing, that since the results of this function will be used to compare objects, if fitness returns nan, inf or negative inf values, frankenstein will panic.

### Mating
The other required method in Evolvable is mate. Its signature is:

      fn mate(Vec<&Self>) -> Self;
      
This function should take a vector of non-mutable pointers to instances of whatever your population type is, and return a new instance of your population. Typically, this input vector would have two elements, corresponding to the two biological parents, and return a child that is in some way a combination of features from the two parents. However, if your evolutionary process calls for it, a given child can have as many parents as necessary.

## Experiments
The Experiment struct is the core of the frankenstein library. This simple struct has two fields:

      pub struct Experiment<T: Evolvable, F: Fn(&Vec<T>) -> Vec<&T>> {
            pub population: Vec<T>,
            selection: F,
      }

The population field stores the current population of whatever it is you are trying to evolve, and selection stores a function that will (usually randomly) select some subset of the population to be passed on to the mate method described earlier. Note that the population vector is guaranteed to be sorted with the best scoring population members at the front and the worst at the back.

Experiments implements several functions to make running evolutionary algorithms easier. Full example workflows can be found in the examples folders. All code below comes from the example find_pi.rs

### New
To create a new expiriment use the following syntax (experiments need to be mutable).

   
      let mut my_exp = Experiment::new(size, selection);
      
Size is a usize that controls the size of the population, and selection is the function that will randomly select parents from the population vector.

