# Bloom filter data structure
## (And his algorithm)

### hash functions
Contains all the hash functions that we will use for the algorithm.

### fill filter
This function applies all hash functions in an array to a given word
and add them to the bloom filter.

### func1
lorem ispum

### contains word
This function is the main function around the bloom filter, using every other
functions to decide is the word is in the list or not.
Takes the number of words, the error rate as arguments to create the filter.
