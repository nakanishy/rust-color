use crate::color::{ColorCode, Rgb};

pub trait Effect {
    type Params;
    fn apply(&self, color_code: ColorCode, params: Self::Params) -> ColorCode;
}

pub struct Lighten;

pub struct LightenParams {
    percent: u8,
}

impl LightenParams {
    pub fn new(percent: u8) -> anyhow::Result<Self> {
        if percent <= 100 {
            Ok(Self { percent })
        } else {
            Err(anyhow::anyhow!("Percent must be between 0 and 100."))
        }
    }
}

impl Effect for Lighten {
    type Params = LightenParams;

    fn apply(&self, color_code: ColorCode, params: LightenParams) -> ColorCode {
        let rgb: Rgb = color_code.into();
        let [r, g, b] = rgb.to_u8_array();

        Rgb::new(
            r + ((255. - r as f32) * (params.percent as f32 / 100.)) as u8,
            g + ((255. - g as f32) * (params.percent as f32 / 100.)) as u8,
            b + ((255. - b as f32) * (params.percent as f32 / 100.)) as u8,
        )
        .into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lighten() {
        let color_code = Rgb::new(0, 0, 0).into();
        let params = LightenParams::new(10).unwrap();
        let result = Lighten.apply(color_code, params);
        assert_eq!(result, Rgb::new(127, 127, 127).into());
    }
}
