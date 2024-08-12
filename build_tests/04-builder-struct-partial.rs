use buco::Builder;

#[derive(Builder)]
struct Elements {
    fire: u8,
}

fn main() {
    let _ = Elements::builder().build();
}
