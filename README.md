Silvers Convenient Rust Types or scrtypes for short is a utility crate of some boilerplate heavy yet simple tasks that bloat my code-bases frequently to eliminate those pain points I created a set of utilities mostly expanding on data types that already exist inside of the rust standard library.

---
## String Expansion 

```rust 
use scrtypes::string_expansion::string_extension::ExtendedStrings;
use scrtypes::string_expansion::smart_strings::SmartStrings;
```

The trait `ExtendedStrings` provides:
1. `.is_int()` a way to validate if a `String` or `&str` is a valid integer
2. `.is_float()` a way to validate if a `String` or `&str` is a valid floating point integer
3. `.to_int()` returns an integer or `ParseErrors::NotAValidInt` 
4. `.to_float()` returns an floating point integer or `ParseErrors::NotAValidFloat

The trait `SmartStrings` provides:
1. `.lexer()` turns a `String` or `&str` into a set of tokens of type `Vec<String>` tokens include everything but whitespaces
2. `.fq_tokens()` returns a `HashMap<String,usize>` where each `String` is a token and each `usize` is the frequency
3. `.fq_chars()` returns a `HashMap<char,usize>` where each `char` is every non whitespace character in the initial `String` and `usize` is the frequency  

---
## Vector Expansion

```rust
use scrtypes::vector_expansion::smart_vectors::SmartVectors;
```

The trait `SmartVectors` provides:
1. `.frequencies()` analyzes the frequency of each item in a vector and returns `HashMap<T,usize>` where `T` is the type of items in the vector
2. `.dedup_clone()` returns a `Vec<T>` of the same type as the one it is used on after removing duplicate elements
3. `.not_contains(item: &T)` returns true if the item is not in the vector otherwise false note that the type of the item must match the type of the items in the vector

---
## Boolient Expansion

```rust
use scrtypes::boolean_expansion::boolean_extension::BoolExpansion;
```

The trait `BoolExpansion` offers:
1. `.flip()` returns `true` if used of a boolean of value `false` and vise versa

---
## HashMap Expansion

```rust
use scrtypes::hashmap_expansion::hashmap_extension::HashMapExpansion;
use scrtypes::hashmap_expansion::hashmap_simplify::SimpleHashMap;
```

The trait `HashMapExpansion` offers:
1. `.increment(key: K, amount: T)` finds a key and increments its value by an amount 
2. `.decrement(key: K, amount: T)` finds a key and decrements its value by an amount

The trait `SimpleHashMap` offers:
1. `.if_get(key: &K, f: F)` allows the usage of a closure f to have access to a key from a HashMap safely 
2. `.if_get_mut(key: &K, f: F)` similarly to `.if_get()` it allows the usage of a closure to have access to a key from a HashMap safely but it allows mutation of the value

---
## File Expansion

```rust 
use scrtypes::file_expansion::smart_files::read_lines;
```

This provides a singular function `read_lines(path: &str);` that returns a `Vec<String>` after reading an entire file line by line or provides a `FileErrors` if it encounters an error.

---
## Errors
```rust
#[derive(Debug, PartialEq)]
pub enum ParseErrors {
    NotAValidInt,
    NotAValidFloat,
}

#[derive(Debug, PartialEq)]
pub enum FileErrors {
    FileNotExists,
    UnableToReadFromFile,
}
```
These I believe are the only errors the crate can have.

---
## Note
The crate was developed by me with tests written by Claude I encourage adding abstractions for things you consider bothersome, improving existing abstractions or writing the tests as they are currently not the best. Criticism is welcome as long as it is respectful and technical. A lot of corners were cut for the sake of convenience some will be fixed later.

- [SudoSilver](https://github.com/SudoSilver) 