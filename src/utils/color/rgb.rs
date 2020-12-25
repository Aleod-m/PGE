use std::convert::From;
use super::HsbColor;

pub struct RgbColor {
    pub red   : u8,
    pub green : u8,
    pub blue  : u8,
    pub alpha : Option<u8>
}

impl RgbColor {

    pub fn new(red : u8, green : u8,blue : u8) -> Self {
        Self {
            red,
            green,
            blue,
            alpha : None,
        }
    }

    pub fn to_vec(&self) -> Vec<f32> {
        return vec![self.red as f32, self.green as f32, self.blue as f32];
    }
}


impl From<HsbColor> for RgbColor {
    fn from(color : HsbColor) -> Self {
        let range = color.saturation * color.brightness;
        let mean = color.brightness - range;

        let (rp, gp, bp) = {
            let x = range * (1f32 - (color.hue / 60f32 % 2f32 - 1f32).abs());
            match color.hue as u16 / 60u16 % 6  {
                0 => (range, x, 0f32),
                1  => (x, range, 0f32),
                2 => (0f32, range, x),
                3 => (0f32, x, range),
                4 => (x, 0f32, range),
                5 => (range, 0f32, x),
                _ => (0f32, 0f32, 0f32),
            }
        };
        Self {
            red : ((rp + mean) * 255f32) as u8,
            green : ((gp + mean) * 255f32) as u8,
            blue : ((bp + mean) * 255f32) as u8,
            alpha : None,
        }
    }
}