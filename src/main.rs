use crate::color::ColorCode;

mod color;

fn main() {
    let c = ColorCode::new("#f24").unwrap();
    println!("The color is: {:?}", c);
    println!("RGB: {:?}", c.to_rgb());
}
