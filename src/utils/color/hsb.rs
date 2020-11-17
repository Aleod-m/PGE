use std::convert::From;
use super::RgbColor;

pub struct HsbColor {
    pub hue : f32,
    pub saturation : f32,
    pub brightness : f32,
}

impl HsbColor {

    pub fn new(hue : f32, saturation : f32, brightness : f32) -> Self {
        //TODO : treat invalid parameters case 
        Self {
            hue,
            saturation,
            brightness,
        }
    }
}

impl From<RgbColor> for HsbColor  {
    fn from(color : RgbColor) -> Self {
        let r = color.red as f32 / 255f32;
        let g = color.green as f32 / 255f32;
        let b = color.blue as f32 / 255f32;
        let max = r.max(g.max(b));
        let min = r.min(g.min(b));
        let range = max - min;
        let hue = match (range, max == r, max == g, max == b) {
            (0f32,_,_,_)    => 0f32,
            (_,true,_,_) => ((g - b) / range % 6f32 ) * 60f32,
            (_,_,true,_) => ((b - r) / range + 2f32) * 60f32,
            (_,_,_,true) => ((r - g) / range + 4f32) * 60f32,
            (_, false, false, false) => 0f32,
        };

        let brightness = max;
        let saturation = range / brightness;

        Self {
            hue,
            saturation,
            brightness,
        }
    }
}