use buco::Builder;

#[derive(Builder)]
struct Elements {
    fire: u8,
}

fn main() {
    let _ = Elements::builder().set_fire(1).set_fire(2).build();
}
