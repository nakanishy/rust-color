use crate::color::{ColorCode, Effect, Palette};
use colored::Colorize;

mod color;

fn main() {
    let color_codes = vec![
        ColorCode::new("#2ADFFF").unwrap(),
        ColorCode::new("#B3F3F2").unwrap(),
        ColorCode::new("#E1FAEE").unwrap(),
        ColorCode::new("#EF5A8C").unwrap(),
    ];
    let palette = Palette::new(color_codes);
    println!("Original:");
    print_palette(&palette);

    let effects: Vec<Box<dyn Effect>> = vec![
        Box::new(color::HueShift::new(
            color::HueShiftParams::new(10.0).unwrap(),
        )),
        Box::new(color::Lighten::new(color::LightenParams::new(30).unwrap())),
    ];

    let mut new_palette = Palette::new(vec![]);
    for c in palette.color_codes().iter() {
        let mut new_color_code = c.clone();

        // Apply every effects to new_color_code
        for effect in effects.iter() {
            new_color_code = effect.apply(new_color_code);
        }

        new_palette.add_color(new_color_code);
    }

    println!("Effect Applied:");
    print_palette(&new_palette);
}

fn print_palette(palette: &Palette) {
    for color_code in palette.color_codes().iter() {
        let [r, g, b] = color_code.clone().to_rgb().to_u8_array();
        print!("{}", "▆".truecolor(r, g, b));
    }
    println!("");
}
