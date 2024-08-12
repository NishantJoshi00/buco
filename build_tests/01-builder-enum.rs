use buco::Builder;

#[derive(Builder)]
enum Elements {
    Fire,
    Water,
    Earth,
    Air,
}

fn main() {
    let _ = Elements::builder().set_fire().build();
}
