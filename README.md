# frankenstein
frankenstein is a rust library for evolutionary algorithms.

## Evolutionary algorithms 
Evolutionary algorithms are based on the biological idea of survival of the fittest. Most evolutionary algorithms follow a simple procedure. 
   1. A population is randomly initialized. 
    
   2. Each member of this population is then evaluated according to some metric. 
    
   3. Via some selection process (usually biased towards the most successful members of the population), parents are chosen and 'mated' or combined to create a child.
    
   4. The third step is repeated until the child population is the same size as the original parent population.
    
## The Evolvable Trait
Frankenstein is customizable and makes almost no assumptions about the kind of algorithm you are trying to train, only requiring that the members of your population have implemented the Evolvable trait.  
