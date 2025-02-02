# Everything about Iterators.


## What is an iterator.
(<i>Iterators in Rust are "Lazy" </i>)
An iterator is a struct which is created when the
`iter()` method called on a sequential data type (Like a vector).
It is a completely separate entity from the original variable it is
created on top of.


### Three fields that exist inside an iterator:

- Pointer to the data on top of which the iterator is created.
- Pointer to the first element of the structure
- Pointer to the end. Outside the bounds of the vector it is created on top of.

## The `next()` function

Whenever the next() function is called, it retrieves the element residing at
the current position the second pointer (of the iterator struct) is pointing at.
Calling this fucntion returns an Option.



## Iterator Functions

Whenever you declare a for loop that iterates over a vector,
the for loop creates an iterator for you.

Rust iterations only happen, when you either call `next()`, or call a function
on the iterator that calls the "next" function on the vector, that consumes
data from the data.

### for_each - an iterator consumer
for_each calls `next()` on the iterator for you.


### map, an iterator adapter
It does something with the data inside the iterative data construct,
but doesn't really matter unless the iterator is consumed. It needs a consumer
to call the next function for it.

Adapters create an extra step in the iterator pipelin, but don't actually output
everything. Can't do that without a consumer.


[vector] --> [iter()] --> [map()] --> [for_each]
