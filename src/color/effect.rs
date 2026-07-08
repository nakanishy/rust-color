use crate::color::{ColorCode, Rgb};

pub trait Effect {
    type Params;
    fn apply(&self, color_code: ColorCode, params: Self::Params) -> ColorCode;
}

struct Lighten;

struct LightenParams {
    percent: u8,
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
        let effect = Lighten;
        let color_code = Rgb::new(0, 0, 0).into();
        let params = LightenParams { percent: 50 };
        let result = effect.apply(color_code, params);
        assert_eq!(result, Rgb::new(127, 127, 127).into());
    }
}
