## Ownership, Borrowing and Lifetimes System

- Three connected systems
- Tough to understand, but represent 90% of the difficulty of Rust
- Dramatically change the way you will design and write code.


### Rust Ownership and borrowing has 12 different Rules.

1. Every value is owned by a single struct, variable , vector , etc at one time.
2. Reassigning a variable to another variable,passing it to anohter function, putting it into a vector, etc, moves the value. The old variable can't be used anymore.
3. You can create many read only references to a value that exist at the same time.
4. You can't move a value while a ref to a value exists.
5. You can make a writable (mutable) reference to a value only if there is not read-only references currently in use. One mutable ref to value can exist at a time.
6. You can't mutate a value   through the owner when any ref (mutable or immutable) to the value exists.
7. Some types of values are copied instead of moved (numbers, bool, chars, arrays, tuples, with copyable elements)
8. When a variable goes out of scope, the value owned by it is dropped. (cleaned up in memory)
9. Values can't be dropped if there are still active references to it.
10. References to a variable can't outlive the value they refer to.

** When in doubt, remember that Rusts wan'ts to inherently minimise the number of updates made to data **
