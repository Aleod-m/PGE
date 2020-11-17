use std::convert::From;
use super::RgbColor;

pub struct HsbColor {
    pub hue : f64,
    pub saturation : f64,
    pub brightness : f64,
}

impl HsbColor {

    pub fn new(hue : f64, saturation : f64, brightness : f64) -> Self {
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
        let r = color.red as f64 / 255f64;
        let g = color.green as f64 / 255f64;
        let b = color.blue as f64 / 255f64;
        let max = r.max(g.max(b));
        let min = r.min(g.min(b));
        let range = max - min;
        let hue = match (range, max == r, max == g, max == b) {
            (0f64,_,_,_)    => 0f64,
            (_,true,_,_) => ((g - b) / range % 6f64 ) * 60f64,
            (_,_,true,_) => ((b - r) / range + 2f64) * 60f64,
            (_,_,_,true) => ((r - g) / range + 4f64) * 60f64,
            (_, false, false, false) => 0f64,
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