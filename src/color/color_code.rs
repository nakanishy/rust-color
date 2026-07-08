use regex::Regex;

use crate::color::{Hsl, Rgb};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ColorCode(String);

impl ColorCode {
    pub fn new(value: &str) -> anyhow::Result<Self> {
        let re = Regex::new(r"^#([A-Fa-f0-9]{6}|[A-Fa-f0-9]{3})$")?;
        if !re.is_match(value) {
            return Err(anyhow::anyhow!("Invalid color code: {}", &value));
        }
        if value.len() == 7 {
            return Ok(Self(value.to_string().to_lowercase()));
        }
        Ok(Self(
            format!(
                "#{0}{0}{1}{1}{2}{2}",
                &value[1..2],
                &value[2..3],
                &value[3..4]
            )
            .to_lowercase(),
        ))
    }

    pub fn get(&self) -> &str {
        &self.0
    }

    pub fn to_rgb(self) -> Rgb {
        self.into()
    }

    pub fn to_hsl(self) -> Hsl {
        self.into()
    }
}

impl From<Rgb> for ColorCode {
    fn from(rgb: Rgb) -> Self {
        let [r, g, b] = rgb.to_u8_array();
        ColorCode(format!("#{:02x}{:02x}{:02x}", r, g, b))
    }
}

impl From<Hsl> for ColorCode {
    fn from(hsl: Hsl) -> Self {
        let [h, s, l] = hsl.to_f32_array();
        let h = h.clamp(0.0, 360.0);
        let s = s.clamp(0.0, 100.0) / 100.0;
        let l = l.clamp(0.0, 100.0) / 100.0;

        let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
        let hh = h / 60.0;
        let x = c * (1.0 - (hh % 2.0 - 1.0).abs());

        let (r1, g1, b1) = if (0.0..1.0).contains(&hh) {
            (c, x, 0.0)
        } else if (1.0..2.0).contains(&hh) {
            (x, c, 0.0)
        } else if (2.0..3.0).contains(&hh) {
            (0.0, c, x)
        } else if (3.0..4.0).contains(&hh) {
            (0.0, x, c)
        } else if (4.0..5.0).contains(&hh) {
            (x, 0.0, c)
        } else {
            (c, 0.0, x)
        };

        let m = l - c / 2.0;

        let r = ((r1 + m) * 255.0).round() as u8;
        let g = ((g1 + m) * 255.0).round() as u8;
        let b = ((b1 + m) * 255.0).round() as u8;

        let rgb = Rgb::new(r, g, b);
        rgb.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let c = ColorCode::new("#ffffff").unwrap();
        assert_eq!(c.get(), "#ffffff");
        let c = ColorCode::new("#fFFffF").unwrap();
        assert_eq!(c.get(), "#ffffff");
        let c = ColorCode::new("#FFFFFF").unwrap();
        assert_eq!(c.get(), "#ffffff");
        let c = ColorCode::new("#fff").unwrap();
        assert_eq!(c.get(), "#ffffff");
        let c = ColorCode::new("#Fff").unwrap();
        assert_eq!(c.get(), "#ffffff");
        let c = ColorCode::new("#FFF").unwrap();
        assert_eq!(c.get(), "#ffffff");

        assert!(ColorCode::new("#").is_err());
        assert!(ColorCode::new(" ").is_err());
        assert!(ColorCode::new("#fffff").is_err());
        assert!(ColorCode::new("#f").is_err());
        assert!(ColorCode::new("##ffffff").is_err());
        assert!(ColorCode::new("##FFF").is_err());
    }

    #[test]
    fn from_rgb() {
        let rgb = Rgb::new(0, 0, 0);
        assert_eq!(ColorCode::new("#000000").unwrap(), rgb.into());
        let rgb = Rgb::new(255, 255, 0);
        assert_eq!(ColorCode::new("#ffff00").unwrap(), rgb.into());
    }
}
