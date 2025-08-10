# Learning Rust

Going through the tutorial

https://doc.rust-lang.org/book/ch01-00-getting-started.html

## Managing things with `rustup`

`rustup` is a CLI tool for managing Rust versions.

`rustup update` will update Rust

`rustup doc` opens the local doc

## Cargo - the build system and package manager

`cargo new hello_world` is a good way to create a new Rust project. If the folder exists use `cargo init`.

`Cargo.toml` defines your project, dependencies, etc.

`cargo build` builds your project

`cargo build --release` builds in release mode

`cargo run` runs your project

`cargo check` checks code but doesn't build it

## Variables and mutability

Rust variables are immutable by default. Using `let mut x` declares a mutable variable.

`const x` declares a constant which can't be mutable. They must have a type annotation.

Shadowing is allowed in the same scope or in child scopes.

## Data types

Rust has sized integral types `iN, uN` for `N` in `[8,16,32,64,128]` and `isize, usize`.

Numeric literals can use `_` as a separator for readability.

Integer overflows panic in debug mode and wrap in production. Use the various methods like `wrapping_*, checked_*, overflowing_*, saturating_*` to get specific defined behavior.

`f32, f64` are the two floating point types.

`char` is the fundamental character type and defined with single quotes.

`tuples` are defined with parens `let tup: (i32, f64, u8) = (500, 6.4, 1);`. Destructuring assignments work as expected with this syntax. Elements can be accessed using `tup.N` for each index.

`array` types use square brackets and are homogeneous. They are fixed size. `vectors` are allowed to change size. Array types are declared with the base type and number of elements `let a: [i32; 5] = [1,2,3,4,5]` or `let a = [3; 5]` for a 5 element array containing 3 in each slot.

Rust does runtime index checks.

## Functions

Function declarations look like

```rust
fn add_two(a: i32, b: i32) -> i32 {
  return a + b;
}
```

and return the last expression implicitly when a semicolon is omitted.

## Control flow

`if`'s are expressions and don't use parens around the condition. They require the condition to be boolean and do no conversions. Ternary operators can be used like `let x = if cond {6} else {7}`

Looping constructs include `loop, while, for`. Values can be returned from loops `break val;` which is the output of the loop expression. Loop labels allow disambiguating loops for `break`.

`for element in array {...}` allows looping through the contents of something including ranges like `1..4`.

## Ownership

Ownership allows Rust to have memory safety without garbage collection or other runtime memory management.

**Ownership rules**

1. Each value in Rust has an **owner**
2. There can only be one owner at a time
3. When the owner goes out of scope, the value is dropped

When something goes out of scope, Rust calls the `drop` function that can be overridden.

Assigning from one heap value to another in Rust is a **move** where the original is invalidated. No automatic deep copies are created for assignments. `clone` is a commonly implemented method to perform deep copies.

The `Copy` trait signifies that a type is trivially copyable and is mutually exclusive with the `Drop` trait. Primitive types and tuples of those are trivially copyable. For these, assignments don't move, they copy.

Passing values to functions will move or copy. In case of a move, the value is invalid after the call. Rust gives static checks to avoid issues. Returning values also transfers ownership.

References allow passing values without moves/copies and use `&`. References are **guaranteed** to point to valid objects. Creating a reference is referred to as **borrowing** because ownership is not transferred.

References are immutable by default. You can use `&mut` for mutable references. There can only be 1 mutable reference to a value. This helps avoid data races. If a mutable reference exists, there can be no immutable
references.

## Slices

Slices can be defined via `let slice = &s[3..6];` where the start index is included and the end index is excluded. `..3` is the same as `0..3` and `&s[1..s.len()]` is the same as `&s[1..]`.

The type of a string slice is `&str`. The type of an array slice is something like `&[i32]`.

## Enums and match

In Rust enums can contain arbitrary data and allow for matching in `match`. `match` is a powerful pattern matching construct. The `if...let` and `let...else` constructs are shorthand ways to use matching when you don't need an entire match.

Next:
https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html

## Crates, packages, and modules

A **crate** is the smallest chunk of code that Rust will compile. There are binary crates and library crates. A package is defined by a `Cargo.toml` file and can contain 0 or more binary crates and at most 1 library crate. For each file in the `src/bin` folder, you get a binary crate.

Rust begins compiling by starting from the crate root, usually either `src/lib.rs` or `src/main.rs` for lib/binary crates.

Modules can be declared with `mod garden;`. The compiler looks for the definition

* Inline in curly brackets
* In `src/garden.rs`
* In `src/garden/mod.rs`

Submodules can be defined in a module file `mod vegetables;`

* Inline
* in `src/garden/vegetables.rs`
* In `src/garden/vegetables/mod.rs`

Paths are then built up with `crate::garden::vegetables::Asparagus`. Modules can be public (`pub`) or private (the default) to parents. Module members can be public or private too.

You refer to modules using **paths** which can be absolute or relative. Absolute paths are from the crate root which is either the crate's name for external modules or the literal `crate` for the current crate.

Relative paths can use `super, self` or an id in the current module.

Ancestor modules can't see descendent module contents by default but descendants can see ancestors.

Struct types can be public or private as can each of their fields. With a private field, you need to include a constructor function. For enums only the type can be declared public, at which point all members become public.

`use` can be used to import a name to a scope. When importing functions, the parent module is imported so the function is `modname::func_name` but for types the type is imported directly.

`pub use` can be used to re-export something.

## Collections

### Vectors

`Vec<T>` is a heap allocated vector. Use `Vec::new` or the `vec!` macro to allocate. Use `[]` for direct access which will panic on out of bounds. `Vec::get` returns an `Option<&element>`. Looping through the `Vec` can be done with

```rust
// Skip the mut for immutable references
let mut v = vec![100, 32, 57];
for i in &mut v {
  *i += 50;
}
```

Vectors are homogenous but can use `enums` as a variant to store a finite set of base types.

### Strings

See the [string section](https://doc.rust-lang.org/book/ch08-02-strings.html) for all of the details.

The core string type is a string slice `str` usually seen in borrowed form `&str`. That's the type of string literals. `String` is in the standard library is a growable, mutable, owned, UTF-8 encoded string type.

String is a wrapper around a vector of bytes and so provides some of the same APIs.

`.to_string()` can be used to get a `String` on any type implementing `Display` including string literals: `let s = "hello".to_string();`. `String::from` does the same thing.

`push, push_str` can be used to add to the string.

Note that `+` takes ownership of the first operand. If you don't want that use `format!` and string template parameters.

Indexing into a string via `&s[n]` is disallowed because the `String` is effectively `Vec<u8>` and UTF-8 is a variable-length encoding. Indexing bytes is rarely what someone wants.

Slicing `&s[0..2]` is supported but is checked at runtime to ensure you're operating at character boundaries.

Use `.chars(), .bytes()` to say if you want chars or bytes and iterate/index those instead.

The Book says that handling grapheme clusters is even more complicated and not handled in the standard library.

### HashMap

`HashMap<K, V>` is a hash table. Create one with `::new` use `::insert` to insert things. Use `::get` to get an `Option<&V>` like `m.get(&team_name).copied().unwrap_or(0);`.

Values are copied/moved into the hash table so it owns them. If references are stored, then borrowing happens. `scores.entry(String::from("Yellow")).or_insert(50);` can be used to insert only if the key does not exist.

`let count = map.entry(word).or_insert(0); *count += 1;` can be used to update a value in place.

**Note:** Iteration order is unstable in `HashMap`. There's a crate called `IndexMap` that claims to fix this.

## Error handling

Rust separates two types of errors **recoverable** and **unrecoverable**. The first are handled by using a `Result<T,E>` type and the second is with `panic!`.

Panics occur with explicit calls to `panic!` or with operations that trigger a panic like indexing an array out of bounds.

Panics unwind the stack and clean up by default. Adding

```toml
[profile.release]
panic = 'abort'
```

in your `Cargo.toml` skips the unwinding and just aborts.

Setting then env variable `RUST_BACKTRACE=1` shows a backtrace during a panic.

The result type is used in operations which have recoverable errors

```rust
enum Result<T,E> {
  Ok(T),
  Err(E),
}
```

As an example, opening a file may fail. Using the `Result` output allows you to handle cases:

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            _ => {
                panic!("Problem opening the file: {error:?}");
            }
        },
    };
}
```

Using other tools like `unwrap_or_else, unwrap, expect` can make your code more concise.

Given this `Result` idiom, propagating errors amounts to returning results up the call stack until something fully handles it. This gets quite messy when done explicitly using the match code above.

Rust provides the `?` operator:

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
```

Putting `?` after something returning a `Result` means to return the error from the function if there is one. Otherwise the `Ok` value is returned from the expression.

`?` causes the `from` function to be called on the error to covert it to the type declared in the containing function declaration.

`?` can be chained and used as a value. It can only be used on expression in functions where the expression's type matches the function's return type.

`?` also works with `Option`.

The `Result::ok` and `Option::ok_or` allow converting `Result` to `Option` and vice versa.


