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
