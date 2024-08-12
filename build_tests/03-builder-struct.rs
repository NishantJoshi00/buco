use buco::Builder;

#[allow(dead_code)]
#[derive(Builder)]
struct Elements {
    fire: u8,
    water: u8,
    earth: u8,
    air: u8,
}

fn main() {
    let _ = Elements::builder()
        .set_fire(1)
        .set_water(2)
        .set_earth(3)
        .set_air(4)
        .build();
}
