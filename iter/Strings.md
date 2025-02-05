# What are Strings

Strings are considered in the context of collections because in Rust strings are treated as a collection of Bytes.There are various ways in which
other languages handle strings, but Rust has chosen the most proper way as its default, in order to handle UTF-8 issues up front. This approach avoids
the developer to face ASCII character related problems with their strings downstream the development lifecycle.


### Functionalities for creating, updating and reading a string.

### The String Slice `&str` type.
The string slice types that are often used in their borrowed state `&str`, is a string type that is a reference of some UTF-8 encoded string data
stored elsewhere. String literals for example, are storeid in the program's binary and therefore string slices.

### The `String` Type.
Is a growable, mutable, owned, UTF-8 encoded string type. String operations are similar to Vector<T> operations in Rust, since Strings are essentially
vectors of type bytes with an extra layer of behaviours and restrictions.

```
      let mut s = String::new();


      // wanting some data loaded to it

      let data = "Initial content"
      let s = data.to_string();
      //or

      let s = "Initial content".to_string();
```

### Updating String types.

We can grow a string using the push_str method to append a string slice. You can grow a string variable just like for Vec<T>, if you push
more data into it. You can use the `+` operator or the `format!` macro to concatenate two strings as well.

```
    let mut s = String::from("lo");
    let s2 = "and behold";
    s1.push_str(s2);
    println!("s2 is {s2}");

```

When we use the `+` operator to add two strings, we use one owned string on the left side of the operator, and a reference on the right side of it.
The reason being the signature of the `+` operator itself uses the `add` operation, who's signature looks something like the following:

```
    fn add(self, s: &str) -> String
```

You can only add an owned string with an &str, being that the second argument in the `add` method has &String as a reference. this is because the compiler
coerces the &String value to &str. Which is `&str -> &str[..]`.

Second, we can see in the signature that add takes ownership of self because self does not have an &.
This means s1 in Listing 8-18 will be moved into the add call and will no longer be valid after that.


### Use format! instead.

WHen you want to concat multiple strings inside a single string, the use of `+` operator can cause confusion ,due to the constant transfer of ownership.
Use the `format!` macro instead. You can simply have a

```
  let s1 = String::from("Hello");
  let s2 = String::from("World");
  let s2 = format!("{}, {}",s1, s2);

  ```

  Where format! takes references to concatenate strings and doesn't take ownership of any of the parameters and uses references instead.


### Reasons why you can't index a string with a single integer

1. A string is a vector of bytes. that's it.
  - because of this, the complexity is no longer O(1), since, Rust has to go through the contents of the byte vector to find the `nth` index.
  - It's not clear what the return type of the indexing operation would be, a byte value, a character or a grapheme cluster (UTF-8 encoded bytes);
  - Rust provides different ways to interpret strings that computers store so that each program can store the interpretation it needs.
  - Indexing using a single number would just return a byte value (technically it should) which means nothing.

Instead of using a single number, use 2 numbers and retrieve a slice of the string containing particular bytes. (Depends on how many bytes make up that particular character.)
`As long as they fulfill qualifying as character boundaries`


### Iterating over strings

You need to be explicit about characters or bytes.
For individual unicode character values, use the `chars` method. calling `chars` separates out the returns two values of type char, and you can iterate
over the result to access each element.


```
  for c in "Зд".chars() {
      println!("{c}");
  }

  ```

Similarly, the `bytes` method returns you each raw byte, which might appropriate for your domain.

for b in "Зд".bytes() P{
    println!({b});
}
