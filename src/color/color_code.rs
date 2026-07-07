use regex::Regex;

#[derive(Debug)]
pub struct ColorCode(pub String);

impl ColorCode {
    pub fn new(value: &str) -> anyhow::Result<Self> {
        let re = Regex::new(r"^#([A-Fa-f0-9]{6}|[A-Fa-f0-9]{3})$")?;
        if !re.is_match(value) {
            return Err(anyhow::anyhow!("Invalid color code: {}", &value));
        }
        if value.len() == 7 {
            return Ok(Self(value.to_string()));
        }
        Ok(Self(format!(
            "#{0}{0}{1}{1}{2}{2}",
            &value[0..1],
            &value[1..2],
            &value[2..3]
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid() {
        assert!(ColorCode::new("#ffffff").is_ok());
        assert!(ColorCode::new("#fFFfFF").is_ok());
        assert!(ColorCode::new("#FFFFFF").is_ok());
        assert!(ColorCode::new("#fff").is_ok());
        assert!(ColorCode::new("#Fff").is_ok());
        assert!(ColorCode::new("#FFF").is_ok());
    }

    #[test]
    fn invalid() {
        assert!(ColorCode::new("#").is_err());
        assert!(ColorCode::new(" ").is_err());
        assert!(ColorCode::new("#fffff").is_err());
        assert!(ColorCode::new("#f").is_err());
        assert!(ColorCode::new("##ffffff").is_err());
        assert!(ColorCode::new("##FFF").is_err());
    }
}
