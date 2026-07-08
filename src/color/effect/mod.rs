use crate::color::ColorCode;

pub mod lighten;
pub use lighten::*;

pub trait Effect {
    type Params;
    fn apply(&self, color_code: ColorCode, params: Self::Params) -> ColorCode;
}
