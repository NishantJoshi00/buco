## buco (**Bu**ilder at **Co**mpile Time)

A simple crate for implementing builder pattern, while still maintaining the safety and predictability of the Rust compiler.

### Usage

Add the following to your `Cargo.toml`:

```toml
[dependencies]
buco = "0.1"
```

### Example

```rust
use buco::Builder;

#[derive(Builder)]
struct Foo {
    a: i32,
    b: i32,
    c: i32,
}

fn main() {
    let foo = Foo::builder()
        .set_a(1)
        .set_b(2)
        .set_c(3)
        .build();

    assert_eq!(foo.a, 1);
    assert_eq!(foo.b, 2);
    assert_eq!(foo.c, 3);
}
```
