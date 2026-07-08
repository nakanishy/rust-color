use crate::color::{ColorCode, Effect, LightenParams, Palette};
use colored::Colorize;

mod color;

fn main() {
    let palette = Palette::new(vec![
        ColorCode::new("#2ADFFF").unwrap(),
        ColorCode::new("#B3F3F2").unwrap(),
        ColorCode::new("#E1FAEE").unwrap(),
        ColorCode::new("#EF5A8C").unwrap(),
    ]);
    println!("Original:");
    print_palette(&palette);

    let mut new_palette = Palette::new(vec![]);
    palette.color_codes().iter().for_each(|c| {
        // let effect = color::HueShift;
        // let applied = effect.apply(c.clone(), HueShiftParams::new(260.0).unwrap());
        let effect = color::Lighten;
        let applied = effect.apply(c.clone(), LightenParams::new(70).unwrap());
        new_palette.add_color(applied);
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
