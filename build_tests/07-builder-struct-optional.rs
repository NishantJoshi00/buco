use buco::Builder;

#[allow(dead_code)]
#[derive(Builder)]
struct Elements {
    fire: u8,
    water: u8,
    earth: Option<u8>,
    air: u8,
    light: Option<u8>,
    dark: u8,
}

fn main() {
    let _ = Elements::builder()
        .set_fire(1)
        .set_water(2)
        .set_air(4)
        .set_light(Some(5))
        .set_dark(6)
        .build();
}
