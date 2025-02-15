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


## Iterator consumers
### Teh `for_each` consumer
for_each calls `next()` on the iterator for you.You can enter a closure function inside the for_each call that allows you to
run an expression for each element that is pushed by the for_each function's "Next", call.

#### The `collect` consumer

`collect()`: Collect can be used on top of a `map()` adapter to get a new vector/ return a new vector. Or store it into a new variable.
Similar to how iterators can be used to iterate over any sort of sequential data structure, collect can be used to "gather" elements of an iterator
into different kinds of data structures.

Collect, if it is the last statement in a funciton, looks at the type annotation in the function declaration and processes iterator objects accordingly.
Return types in `collect` are coerced from the type annotations most significant to it. Its either one of 3 places
1. Type annotation in the return type of the function
2. Type annoatation afte `let` into which the collect function will be storing its output.
3. Inline with the collect call you can mention the type itself.

You can also use :: to type annotate when you call collect itself.

` .collect::<Vec<String>>()` or `.collect::<Vec<_>>() `

Note: In rust, types you add in can modify how your code actually functions.

### map, an iterator adapter
It does something with the data inside the iterative data construct,
but doesn't really matter unless the iterator is consumed. It needs a consumer
to call the next function for it.

Adapters create an extra processing step in the iterator pipeline, which also takes a closure function as an argument,
but doesn't actually output anything since `next` isn't called. Can't do that without a consumer.


[vector] --> [iter()] --> [map()] --> [for_each]

We rely on iterator consumers to call next() for us because: Iterators are lazy.


## Mutable iterators

1. `iter()` : Will give you a read-only reference to each element
2. `iter_mut()`: Will give you a mutable reference to each element.
3. `into_iter()`: The Iterator gives you an ownership of each result element, unless called on a mutable reference to a vector.
(Any function with the word 'into' as prefix will much likely take ownership of the )


## The `find` and `map_or` function

Calling `find` on an iterator returns an Option, which is used by `map_or` to Return the 'Some' or the 'None'.
Find calls `next` until it finds a "truthy" value and then returns an Option.
