# Bloom filter data structure
## (And his algorithm)

### hash functions
Contains all the hash functions that we will use for the algorithm.

### fill filter (In bloom_filter.rs)
This function applies all hash functions in an array to a given word
and add them to the bloom filter.

### is present
This function is the main function around the bloom filter, using every other
functions to decide is the word is in the list or not.
Takes the number of words, the error rate as arguments to create the filter.

### false positive prob calculator
#### Not implemented yet...
Computes the probabilty of positive result being false positive thanks to
Bose, Guo, Kranakis... algorithm.

### bloom size computing
Computes the perfect size of the bloom filter based on the number of keys and
the error rate.

### bloom filter
Main function around bloom filter.
Takes 2 parameters : The number of keys and the error rate aimed.

Optionnal : The array will be saved in a file to be re-used later.
