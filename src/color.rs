use regex::Regex;

#[derive(Debug)]
pub struct ColorCode(String);

impl ColorCode {
    fn new(value: String) -> anyhow::Result<Self> {
        let re = Regex::new(r"^#([A-Fa-f0-9]{6}|[A-Fa-f0-9]{3})$")?;
        re.is_match(&value)
            .then(|| Self(value))
            .ok_or_else(|| anyhow::anyhow!("Invalid color code"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid() {
        assert!(ColorCode::new("#ffffff".to_string()).is_ok());
        assert!(ColorCode::new("#fFFfFF".to_string()).is_ok());
        assert!(ColorCode::new("#FFFFFF".to_string()).is_ok());
        assert!(ColorCode::new("#fff".to_string()).is_ok());
        assert!(ColorCode::new("#Fff".to_string()).is_ok());
        assert!(ColorCode::new("#FFF".to_string()).is_ok());
    }

    #[test]
    fn invalid() {
        assert!(ColorCode::new("#".to_string()).is_err());
        assert!(ColorCode::new(" ".to_string()).is_err());
        assert!(ColorCode::new("#fffff".to_string()).is_err());
        assert!(ColorCode::new("#f".to_string()).is_err());
        assert!(ColorCode::new("##ffffff".to_string()).is_err());
        assert!(ColorCode::new("##FFF".to_string()).is_err());
    }
}
