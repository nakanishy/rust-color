use crate::color::ColorCode;

pub mod hue_shift;
pub mod lighten;
pub use hue_shift::*;
pub use lighten::*;

pub trait Effect {
    type Params;
    fn apply(&self, color_code: ColorCode, params: Self::Params) -> ColorCode;
}
