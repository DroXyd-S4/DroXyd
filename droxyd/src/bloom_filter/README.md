# Bloom filter data structure
## (And his algorithm)

### hash functions
Contains all the hash functions that we will use for the algorithm.

### fill filter
This function applies all hash functions in an array to a given word
and add them to the bloom filter.

### contains word
This function is the main function around the bloom filter, using every other
functions to decide is the word is in the list or not.
Takes the number of words, the error rate as arguments to create the filter.

### false positive prob calculator
Computes the probabilty of positive result being false positive thanks to
Bose, Guo, Kranakis... algorithm.

### bloom filter
Main function around bloom filter.
Takes 2 parameters : The number of keys and the error rate aimed.
Based on those numbers, an array will be created thanks to a magic formula
that finds the perfect size.

Optionnal : The array will be saved in a file to be re-used later.
