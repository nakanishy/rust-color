use crate::color::{ColorCode, Effect, Palette};
use colored::Colorize;

mod color;

fn main() {
    let mut color_codes = vec![
        ColorCode::new("#2ADFFF").unwrap(),
        ColorCode::new("#B3F3F2").unwrap(),
        ColorCode::new("#E1FAEE").unwrap(),
        ColorCode::new("#EF5A8C").unwrap(),
    ];
    // let color_codes: Vec<_> = color_codes
    //     .iter()
    //     .cloned()
    //     .cycle()
    //     .take(8_000_000)
    //     .collect();
    let palette = Palette::new(color_codes);
    println!("Original:");
    print_palette(&palette);

    let mut new_palette = Palette::new(vec![]);
    let effects: Vec<Box<dyn Effect>> = vec![
        Box::new(color::HueShift::new(
            color::HueShiftParams::new(10.0).unwrap(),
        )),
        Box::new(color::Lighten::new(color::LightenParams::new(30).unwrap())),
    ];
    palette.color_codes().iter().for_each(|c| {
        let mut c = c.clone();
        for effect in effects.iter() {
            c = effect.apply(c);
        }
        new_palette.add_color(c);
    });
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
