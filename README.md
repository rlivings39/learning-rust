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

## Generics, traits, and lifetimes

Generics, traits, and lifetimes are all ways to help reduce code duplication and make your code more flexible while still preserving Rust's static safety guarantees.

### Generics

Generics look like other languages.

* Function `fn largest<T>(list: &[T]) -> &T`
* Structs `struct Point<T> {}`
* Enums `enum Option<T> {Some(T), None}`
* Methods `impl<T> Point<T> {fn foo<T,S>()..}`.
  * Specialization also works `impl Point<f32> {}` to only impact `f32` points.

### Traits

A **trait** defines functionality that a type has and can share with other types. Traits define behavior in an abstract way. **Trait bounds** specify that a generic type posses certain behavior. Traits are like interfaces in other languages.

Traits define a set of one or more methods that a type must have

```rust
pub trait Summary {
  fn summarize(&self) -> String;
}
```

To implement a trait for a type use the `impl...for` construct: `impl Summary for BlogPost {}` outside of the definition for `BlogPost`.

When using a type implementing the trait, you have to pull in both the type and the trait.

You can implement a trait on a type if at least one of the trait or type is local to your crate. So you can't implement an external trait on an external type. Rust calls this preserving **coherence**.

Traits can also provide default behavior that can optionally be overridden. Default implementations can call other methods in the trait even if those do not have default implementations.

Traits can be used as parameters `fn notify(item: &impl Summary)` to allow accepting any type implementing `Summary`. This is shorthand for the trait bound `fn notify<T: Summary>(item: &T)`.

`+` can be used to create sum types to implement if traits

```rust
pub fn notify(item: &(impl Summary + Display));
pub fn notify<T: Summary + Display>(item: &T);
```

`where` clauses can clean up the signature by placing the trait bounds after the declaration

```rust
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
```

`impl TraitName` can also be used in return types, but not to allow returning multiple types.

Trait bounds allow for conditionally implementing methods on generics only when the generic type parameters satisfy the traits:

```rust
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// Only implement cmp_display for types implementing Display and PartialOrd
impl<T: Display + PartialOrd> Pair<T> {
  fn cmp_display(&self){...}
}
```

You can also conditionally implement traits only for types implementing other traits. These are called **blanket implementations**:

```rust
impl<T: Display> ToString for T {
    // Now any displayable type also implements ToString!
}
```

### Lifetimes

Lifetimes are another kind of generic that ensure references are valid as long as they need to be. They are often inferred but lifetimes can also be explicit.

Lifetime annotations don't change how long a reference lives. Rather they describe relationships of lifetimes of multiple references among each other.

Lifetime annotations must start with `'` and are short, lowercase names by convention. `'a` is often used as the first. For example:

```rust
&i32        // A reference
&'a i32     // A reference with a named lifetime
&'a mut i32 // A mutable reference with a named lifetime
```
To use lifetimes in function signatures, declare them within `<>` just like generic type parameters and then use them as appropriate.

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  // Lifetimes tell the borrow checker that the return value lives
  // as long as the shorter of the lives of x or y
  if x.len() > y.len() {x} else {y}
}
```

Lifetime annotations specify constraints used by the borrow checker when analyzing your program. They are part of the contract of the function.

Structs can also hold references and need lifetime annotations in this case.

The Rust compiler has a set of **lifetime elision rules** which are cases where the compiler can explicitly infer lifetime relationships relieving programmers from having to specify explicit lifetimes.

Three rules are applied for functions

1. Each input parameter gets it's own lifetime
2. If there is one input lifetime, that lifetime is assigned to all output lifetimes
3. If there are multiple input lifetimes but one is `&self` or `&mut self`, the lifetime of `self` is assigned to all output lifetimes making methods easier to read and write.

There is one special lifetime `'static` that means that the value is available for the entire life of the program.

Finally, here is a function combining generics, traits, and lifetimes

```rust
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() { x } else { y }
}
```

## Testing in Rust

Rust supports marking things with attributes like `#[test]`. You can apply that macro to a function to make it a test function.

Making a new Cargo lib project will create a test module and example test for you. For example

```rust
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
```
Run that with `cargo test`. Note that this will also extract any code snippets in documentation and run those too to ensure they work.

Test functions are run in their own dedicated thread. Tests fail when they panic. Using `assert!, assert_eq!, assert_ne!`. The 2nd and third will print out the values compared in case of a failure. Adding `#[derive(PartialEq, Debug)]` to your structs or enums will make them compatible with these operations.

Custom failure messages can be passed to the `assert*` macros.

The `#[should_panic]` attribute will make a test verify that the code under test panics. This takes an optional `expected` parameter which allows verifying the right type of panic happens.

`#[ignore]` says to ignore a test by default

Test functions can also return `Result<(), String>` so that they fail on `Err` instead of relying on panics.

### Controlling how tests run

Options can be passed to `cargo test` and to the test binary: `cargo test --flag1 --flag2 -- --binary-flag1 --binary-flag2`

Some useful options are

* `cargo test -- --test-threads=N` - Set number of threads
* `cargo test -- --show-output` - Show any stdout output
* `cargo test filter_string` - Only run tests whose name matches `filter_string`
* `cargo test -- --ignored` - Run ignored tests
* `cargo test -- --include-ignored` - Run all tests ignored or not
* `cargo test --test integration_test_file` - Run all tests in the integration test file

### Test organization

Unit tests are typically located in the `src/` directory in a `tests` module inside the file being tested. The tests module is annotated with `#[cfg(test)]`. This attribute tells the compiler to only compile this code when building for test.

Because test functions are in a nested module, they can access private functions. It's up to you whether or not you think it's a good idea to test private code.

Integration tests go in a `tests/` directory parallel to your source directory. They test your public API. Each file is treated as a separate crate during build.

To make a shared testing module, use the older convention of making a directory `tests/mod_name/mod.rs`

Integration tests can only test lib crates. This is why binary crates are usually structured as a thin wrapper around a lib crate that **can** be tested.

## Closures

Rust has closures like many other languages

```rust
// Fully annotated
let f1 = |num: i32| -> i32 { num + 1 }
// Everything inferred
let f2 = |num| num + 1
```

Closure capture is usually done by reference with mutability inferred by the compiler. To force a move, use `move || closure_body()`. This is useful to get a thread to take ownership of some data.

Closures and functions in Rust implement one or more of a variety of traits

1. `FnOnce` applies to closures that can be called once. All closures implement at least this trait because all closures can be called. A closure that moves captured values out of its body will only implement FnOnce and none of the other Fn traits, because it can only be called once.

2. `FnMut` applies to closures that don’t move captured values out of their body, but that might mutate the captured values. These closures can be called more than once.

3. `Fn` applies to closures that don’t move captured values out of their body and that don’t mutate captured values, as well as closures that capture nothing from their environment. These closures can be called more than once without mutating their environment, which is important in cases such as calling a closure multiple times concurrently.

## Iterators

Iterators are used to iterate through collections. In Rust iterators are lazy and are often had by doing `thing.iter()`. They implement the `Iterator` trait which requires defining an associated `Item` type and a `next` method that returns `Option<Item>` returning `None` when iteration is finished.

`iter()` returns immutable references to the items. `into_iter()` takes ownership of the items. `iter_mut()` yields mutable references to the items.

`Iterator` has many methods (aka **consuming adapters**) which take ownership of the iterator and consume it like `sum` or `collect`.

Other `Iterator` methods are **iterator adapters** which take the original iterator and return a new iterator modifying the original. For example, `map` takes the original and returns a new iterator which applies a function to each element of the original returning the modified result.

`impl Iterator<Item = String>` can be used to take an iterator.

Iterators and closures in Rust tend to be zero cost abstractions. The book shows examples where they match performance of ordinary for loops.

## Smart pointers

Smart pointers are structs that own memory and provide functionality via the `Drop` and `Deref` traits. There are reference counted smart pointers too.

**Note** `Drop::drop()` can't be called directly as doing so would result in a double free. Instead call `std::mem::drop` aka `drop(obj)` instead to free resources early.

`String` and `Vec` can be considered smart pointers.

Three major smart pointers in Rust are

* `Box<T>` for allocating values on the heap
* `Rc<T>` for reference counted multiple ownership
* `Ref<T>, RefMut<T>` accessed via `RefCell<T>` that enforces borrowing rules at runtime instead of compile time

**Note** `Rc<T>` and `RefCell<T>` only work in single-threaded contexts.

### `Box<T>` and `Rc<T>`

`Box<T>` is used to store data on the heap rather than on the stack. It has no performance overhead and is often used with:

* Types whose sizes are only known at runtime in a context where a size must be known at compile-time. E.g. recursive types
* Transferring large amounts of data from the stack to the heap
* Owning a value that you only care implements a particular trait rather than being of a specific type

A box's underlying data can generally be accessed just like data on the stack.

`Box` implements `Deref` so that you can refer to it just like a reference and `Drop` so it cleans up its data when it goes out of scope.

`Rc<T>` works similarly to `Box<T>` but implements reference counting. If you have `let x: Rc<T> = Rc::new(...)` then you can call `Rc::clone(&x)` to increment the reference count to `x`. This is preferable to calling `x.clone()` because the latter is often used for deep copies. `Rc::clone` is cheap and so good to visually distinguish.

`Rc::strong_count()` returns the number of strong references. `Rc::weak_count()` refers to the weak references.

### `RefCell<T>`
`RefCell<T>` is single ownership like `Box<T>` but does its checking at runtime

* `Box<T>` allows immutable and mutable borrows checked at compile-time
* `Rc<T>` enables multiple owners. `Box<T>, RefCell<T>` have single owners
* `RefCell<T>` allows immutable or mutable borrows checked at runtime even when the `RefCell<T>` is immutable

The pattern of mutating a value inside of an immutable value is called the **interior mutability** pattern.

This could be useful for cases when you might use `mutable` in C++. For example having a mutable field inside of an immutable reference to store messages in a mock message sender object.

When you have a `RefCell<T>` you can call `borrow()` or `borrow_mut()` to borrow a reference to the underlying object. The former returns `Ref<T>` the latter `RefMut<T>` which both implement `Deref`.

These borrow methods keep track of how many of each type of reference is live and panic if the ownership rules are violated at runtime.

You can hold `Rc<RefCell<T>>` if you'd like to have multiple owners of data that you can mutate.

### `Weak<T>`

`Rc::clone()` produces a strong reference and increments the reference count. Using `Rc::downgrade(&x)` returns a `let w = Weak<T>` that holds a weak reference. When you'd like to get a strong reference for usage later on call `w.upgrade()` to get a `Option<Rc<T>>` that is none if the reference has been deleted and has the value in the some variant.

### `Deref`

The ability to dereference something using `*x` is provided by implementing the `Deref` trait. When Rust sees `*x` on a type implementing `Deref` it changes the code to `*(x.deref())` so that the bare `*` is called on the output of the `deref` method.

Rust also has deref coercion which allows passing `&T` to a function or method expecting `&R` where `T::deref() -> &R`. This is why `&String` can be passed to a function taking `&str`. `Deref::deref()` is applied as many times as needed to match the source and destination types.

`DerefMut` can be used to implement the same behavior on mutable references. Coercion from immutable to immutable and mutable to mutable are both straightforward. Rust will also coerce mutable to immutable as that preserves safety.

## Cargo

Build profiles can be customized in `Cargo.toml`

```toml
[profile.dev]
opt-level = 1
```

`cargo doc` builds doc for your crate based on doc comments `/// With 3 slashes`. These are formatted with markdown.

Common sections include: examples, panics, safety.

`cargo test` extracts code snippets from your documentation and runs them as tests to ensure they still compile and run.

Using `//!` for a doc comment adds documentation to the containing item rather than the following item. These are useful for crate root files and modules.

`pub use` can be used to re-export items in more convenient locations in your crate.

Cargo supports **workspaces** which allow bundling multiple related packages together. To do this create a root folder with a Cargo.toml containing

```toml
[workspace]
ws_name = "1.2.3"
```

then run `cargo add new_package [--lib]` for each sub package.

Workspaces all share the same target folder and don't necessarily depend on each other. They can refer to each other via `dependencies`.

Each sub-package/crate has its own `Cargo.toml` and manages things independently.

`cargo install` can be used to install binary crates locally.

Adding a binary in your path called `cargo-something` lets you extend cargo by calling `cargo something`. These also show up in `cargo list`. Such extensions can be installed from public repos.

