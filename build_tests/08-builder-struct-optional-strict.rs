use buco::Builder;

#[allow(dead_code)]
#[derive(Builder)]
#[buco(strict)]
struct Elements {
    light: Option<u8>,
}

fn main() {
    let _ = Elements::builder().build();
}
