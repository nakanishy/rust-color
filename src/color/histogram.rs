use crate::color::Palette;

pub struct RgbHist {
    r: [usize; 256],
    g: [usize; 256],
    b: [usize; 256],
}

impl RgbHist {
    pub fn new(palette: &Palette) -> Self {
        let mut hist = [[0usize; 256]; 3];
        for c in palette.color_codes().iter() {
            let [r, g, b] = c.clone().to_rgb().to_u8_array();
            hist[0][r as usize] += 1;
            hist[1][g as usize] += 1;
            hist[2][b as usize] += 1;
        }
        Self {
            r: hist[0],
            g: hist[1],
            b: hist[2],
        }
    }

    pub fn red(&self) -> [usize; 256] {
        self.r
    }

    pub fn green(&self) -> [usize; 256] {
        self.g
    }

    pub fn blue(&self) -> [usize; 256] {
        self.g
    }
}
