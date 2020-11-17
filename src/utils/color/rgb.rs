use std::convert::From;
use super::hsb::HsbColor;

pub struct RgbColor {
    pub red   : u8,
    pub green : u8,
    pub blue  : u8,
}

impl RgbColor {

    pub fn new(red : u8,green : u8,blue : u8) -> Self {
        Self {
            red,
            green,
            blue,
        }
    }

    pub fn to_vec(&self) -> Vec<f64> {
        return vec![self.red as f64, self.green as f64, self.blue as f64];
    }
}


impl From<HsbColor> for RgbColor {
    fn from(color : HsbColor) -> Self {
        let range = color.saturation * color.brightness;
        let mean = color.brightness - range;

        let (rp, gp, bp) = {
            let x = range * (1f64 - (color.hue / 60f64 % 2f64 - 1f64).abs());
            match color.hue as u16 / 60u16 % 6  {
                0 => (range, x, 0f64),
                1  => (x, range, 0f64),
                2 => (0f64, range, x),
                3 => (0f64, x, range),
                4 => (x, 0f64, range),
                5 => (range, 0f64, x),
                _ => (0f64, 0f64, 0f64),
            }
        };
        Self {
            red : ((rp + mean) * 255f64) as u8,
            green : ((gp + mean) * 255f64) as u8,
            blue : ((bp + mean) * 255f64) as u8,
        }
    }
}