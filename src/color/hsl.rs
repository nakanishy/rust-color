use crate::color::ColorCode;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hsl {
    /// 0-360
    h: f32,
    /// 0-100
    s: f32,
    /// 0-100
    l: f32,
}

impl Hsl {
    fn new(h: f32, s: f32, l: f32) -> anyhow::Result<Self> {
        // validate inputs
        if !(h >= 0. && h <= 360.) {
            anyhow::bail!("hue must be between 0 and 360");
        }
        if !(s >= 0. && s <= 100.) {
            anyhow::bail!("saturation must be between 0 and 100");
        }
        if !(l >= 0. && l <= 100.) {
            anyhow::bail!("lightness must be between 0 and 100");
        }

        Ok(Self { h, s, l })
    }
}

impl From<ColorCode> for Hsl {
    fn from(color_code: ColorCode) -> Self {
        let [r, g, b] = color_code.to_rgb().to_u8_array();

        // 0.0 - 1.0 に正規化
        let [r, g, b] = [(r as f32 / 255.), (g as f32 / 255.), (b as f32 / 255.)];

        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let l = (max + min) / 2.0;

        if (max - min).abs() < f32::EPSILON {
            return Hsl {
                h: 0.0,
                s: 0.0,
                l: l * 100.0,
            };
        }

        let d = max - min;
        let s = d / (1.0 - (2.0 * l - 1.0).abs());

        let mut h = if (max - r).abs() < f32::EPSILON {
            60.0 * ((g - b) / d)
        } else if (max - g).abs() < f32::EPSILON {
            60.0 * ((b - r) / d + 2.0)
        } else {
            60.0 * ((r - g) / d + 4.0)
        };

        if h < 0.0 {
            h += 360.0;
        }

        Hsl {
            h,
            s: s * 100.0,
            l: l * 100.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_color_code() {
        let c = ColorCode::new("#f00").unwrap();
        let hsl: Hsl = c.into();
        assert_eq!(
            hsl,
            Hsl {
                h: 0.0,
                s: 100.0,
                l: 50.0,
            }
        );

        let c = ColorCode::new("#f20").unwrap();
        let hsl: Hsl = c.into();
        assert_eq!(
            hsl,
            Hsl {
                h: 8.0,
                s: 100.0,
                l: 50.0
            }
        );

        let c = ColorCode::new("#ffe79a").unwrap();
        let hsl: Hsl = c.into();
        assert_eq!(
            hsl,
            Hsl {
                h: 45.742577,
                s: 100.000015,
                l: 80.19608
            }
        );
    }
}
