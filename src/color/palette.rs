use crate::color::ColorCode;

pub struct Palette {
    color_codes: Vec<ColorCode>,
}

impl Palette {
    pub fn new(color_codes: Vec<ColorCode>) -> Self {
        Self { color_codes }
    }

    pub fn add_color(&mut self, color_code: ColorCode) {
        self.color_codes.push(color_code);
    }

    pub fn color_codes(&self) -> &Vec<ColorCode> {
        &self.color_codes
    }
}
