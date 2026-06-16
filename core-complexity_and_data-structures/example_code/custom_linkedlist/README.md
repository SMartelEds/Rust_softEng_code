# Custom LinkedList Rust Demo

This project demonstrates core Rust concepts without external crates.

## Concepts Covered

- **Structs**: `LinkedList<T>`, private `Node<T>`, and `CourseTopic<'a>`.
- **Implementations**: methods are grouped in `impl<T> LinkedList<T>`.
- **Generics**: the list stores any `T`, not only one concrete type.
- **Option**: each node points to `Option<Box<Node<T>>>`, and lookups return `Option<&T>`.
- **Result**: indexed operations such as `insert` and `remove` return `Result<_, ListError>`.
- **Traits**: `Display`, `Iterator`, `FromIterator`, and a custom `Summary` trait.
- **Iterator**: `LinkedList<T>` is a consuming iterator, while `Iter<'a, T>` is a borrowing iterator.
- **Lifetimes**: `Iter<'a, T>`, `CourseTopic<'a>`, and `first_or_default<'a, T>` show borrowed data.

## Run

```bash
cargo run
```

## Test

```bash
cargo test
```
