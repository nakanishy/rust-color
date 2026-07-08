use crate::color::{ColorCode, Effect, HueShiftParams};

mod color;

fn main() {
    let c = ColorCode::new("#f24").unwrap();

    println!("The color is: {:?}", c);
    println!("RGB: {:?}", c.clone().to_rgb());

    let hue_shift = color::HueShift;
    let c = hue_shift.apply(c, HueShiftParams::new(260.0).unwrap());

    println!("Hue Shifted: {:?}", c);
    println!("RGB: {:?}", c.to_rgb());
}
