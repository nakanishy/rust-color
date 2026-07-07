use crate::color::ColorCode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

impl Rgb {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Self {
            r: red,
            g: green,
            b: blue,
        }
    }

    pub fn get(&self) -> [u8; 3] {
        [self.r, self.g, self.b]
    }
}

impl From<ColorCode> for Rgb {
    fn from(color_code: ColorCode) -> Self {
        let hex = color_code.get().trim_start_matches('#');

        let r = u8::from_str_radix(&hex[0..2], 16).unwrap();
        let g = u8::from_str_radix(&hex[2..4], 16).unwrap();
        let b = u8::from_str_radix(&hex[4..6], 16).unwrap();

        Rgb { r, g, b }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let color_code = ColorCode::new("#ffffff").unwrap();
        let rgb: Rgb = color_code.into();
        assert_eq!(
            rgb,
            Rgb {
                r: 255,
                g: 255,
                b: 255
            }
        );

        let color_code = ColorCode::new("#000000").unwrap();
        let rgb: Rgb = color_code.into();
        assert_eq!(rgb, Rgb { r: 0, g: 0, b: 0 });
    }
}
