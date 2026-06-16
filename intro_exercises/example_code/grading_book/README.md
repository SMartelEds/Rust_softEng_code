# grading_book

A small beginner-friendly Rust command-line gradebook. It uses one binary crate and one source file: `src/main.rs`.

The app lets you:

- Add a student with a name and grade
- Display all students
- Search for a student by name
- Update a grade
- Calculate the class average
- Display the best student
- Classify grades as `Excellent`, `Pass`, `Retake`, or `Invalid`
- Quit from a simple numbered menu

## Rust concepts shown

- Cargo project structure and `Cargo.toml`
- Binary crate with `src/main.rs`
- Variables and mutability
- Basic types: `String`, `u8`, `f64`, `bool`, `usize`
- Stack vs heap idea with simple values, `String`, and `Vec`
- Tuples like `(String, u8)` to represent one student
- Enums
- Functions and procedures
- Ownership and borrowing with `&` and `&mut`
- `Vec<T>`
- `String` vs `&str`
- `if / else`, `match`, `loop`, `break`, and `continue`

## Commands

Create and enter the project:

```bash
cargo new grading_book
cd grading_book
```

Run the application:

```bash
cargo run
```

Check that the code compiles:

```bash
cargo check
```

Format the code:

```bash
cargo fmt
```

Run Clippy for beginner-friendly code suggestions:

```bash
cargo clippy
```

## Notes

This project uses only the Rust standard library. It does not use external crates, structs, tests, file storage, async code, traits, generics, or advanced patterns.
