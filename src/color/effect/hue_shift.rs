use crate::color::{ColorCode, Effect};

pub struct HueShift;

pub struct HueShiftParams {
    hue: f32,
}

impl HueShiftParams {
    pub fn new(hue: f32) -> anyhow::Result<Self> {
        if hue < 0.0 || hue > 360.0 {
            anyhow::bail!("Hue must be between 0 and 360");
        }
        Ok(Self { hue })
    }
}

impl Default for HueShiftParams {
    fn default() -> Self {
        Self { hue: 0.0 }
    }
}

impl Effect for HueShift {
    type Params = HueShiftParams;
    fn apply(&self, color_code: ColorCode, params: Self::Params) -> ColorCode {
        let mut hsl = color_code.to_hsl();
        let [h, _, _] = hsl.to_f32_array();
        hsl.set_hue((h + params.hue).clamp(0.0, 360.0));
        hsl.into()
    }
}
