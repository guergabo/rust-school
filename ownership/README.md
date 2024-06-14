# Rust Ownership & Borrowing Rules

This README provides an in-depth explanation of Rust's ownership (a.k.a move semantics) and borrowing (a.k.a references) rules using code snippets from the provided example. Each rule is introduced and correlated with the memory safety or concurrency issue it addresses. Additionally, we'll discuss the `Copy` trait, the differences between heap and stack values, the `Drop` trait, and the concepts of moved semantics, shared references, mutable references, lifetimes, and RAII.

## Ownership Rules (a.k.a Move Semantics)

1. **Each value in Rust has an owner.**

In the example, the `CSVParser` struct owns the `data` field, which is a `Vec<Row>`. The `CSVProcessor` struct owns a mutable reference to a `CSVParser` instance.

```rust
struct CSVParser {
    data: Vec<Row>,
}

struct CSVProcessor<'a> {
    parser: &'a mut CSVParser,
}
```

This rule ensures that there is always a clear owner responsible for freeing the memory associated with a value.

2. **There can only be one owner at a time.**

When the `CSVProcessor::new` method is called, ownership of the `CSVParser` instance is not transferred. Instead, a mutable reference to the `CSVParser` is passed to the `CSVProcessor`.

```rust
impl<'a> CSVProcessor<'a> {
    fn new(parser: &'a mut CSVParser) -> Self {
        CSVProcessor { parser }
    }
    // ...
}
```

This rule prevents multiple owners from freeing the same memory, which would lead to a double-free error.

3. **When the owner goes out of scope, the value will be dropped.**

In the `CSVParser::parse_csv` and `CSVParser::write_csv` methods, the `File` instances are automatically closed when they go out of scope due to Rust's RAII (Resource Acquisition Is Initialization) principles.

```rust
impl CSVParser {
    fn parse_csv(&mut self, file_path: &str) -> Result<(), Box<dyn Error>> {
        let file = File::open(file_path)?;
        // ...
    }

    fn write_csv(&self, file_path: &str) -> Result<(), Box<dyn Error>> {
        let mut file = File::create(file_path)?;
        // ...
    }
    // ...
}
```

This rule ensures that resources are properly cleaned up when they are no longer needed, preventing resource leaks.

## Borrowing Rules (a.k.a References)

1. **At any given time, you can have either one mutable reference or any number of immutable references.**

The `CSVParser::get_row` and `CSVParser::get_cell` methods return shared references to the internal data, allowing multiple read-only accesses.

```rust
impl CSVParser {
    fn get_row(&self, row_index: usize) -> Option<&Row> {
        // ...
    }

    fn get_cell(&self, row_index: usize, col_index: usize) -> Option<&str> {
        // ...
    }
    // ...
}
```

On the other hand, the `CSVParser::update_cell` method takes a mutable reference to `self`, allowing modification of the internal data.

```rust
impl CSVParser {
    fn update_cell(&mut self, row_index: usize, col_index: usize, value: String) -> Result<(), String> {
        // ...
    }
    // ...
}
```

This rule prevents data races, where multiple references access the same data concurrently, with at least one of them being a write access.

2. **References must always be valid.**

The `CSVProcessor` struct uses an explicit lifetime `'a` to ensure that the reference to the `CSVParser` remains valid for as long as the `CSVProcessor` instance exists.

```rust
struct CSVProcessor<'a> {
    parser: &'a mut CSVParser,
}
```

This rule prevents dangling references, where a reference outlives the data it refers to, which would lead to accessing invalid memory.

## Summary

- **Moved Semantics**: When a value is moved, ownership is transferred to the new location. This ensures that only one owner is responsible for the value at a time.

- **Shared References**: Shared references (`&T`) allow multiple read-only accesses to a value. They do not transfer ownership and ensure that the borrowed value remains valid for the duration of the reference.

- **Mutable References**: Mutable references (`&mut T`) allow a single write access to a value. They do not transfer ownership and ensure that the borrowed value remains valid for the duration of the reference.

- **Lifetimes**: Lifetimes are used to express the scope and duration of references. They ensure that references remain valid and prevent dangling references.

- **RAII**: Resource Acquisition Is Initialization (RAII) is a principle where the acquisition and release of resources are tied to the lifetime of an object. In Rust, this is achieved through ownership and the `Drop` trait, ensuring that resources are properly cleaned up when they are no longer needed.

In summary, Rust's ownership and borrowing rules,work together to ensure memory safety and prevent common issues such as null or dangling pointer dereferences, double frees, and data races. By enforcing these rules at compile-time, Rust helps developers catch potential bugs early in the development process, leading to more reliable software.
