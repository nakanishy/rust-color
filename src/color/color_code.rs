use regex::Regex;

use crate::color::Rgb;

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
}

impl From<Rgb> for ColorCode {
    fn from(rgb: Rgb) -> Self {
        let [r, g, b] = rgb.get();
        ColorCode(format!("#{:02x}{:02x}{:02x}", r, g, b))
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
