pub fn fast_floor(x : f64) -> i32 {
    let xi = x as i32;
    if x < xi as f64 {
        return xi - 1;
    }
    return xi;
}

pub fn max(a : f64, b : f64) -> f64{
    if a > b {
        a
    }
    else {
        b
    }
}

pub fn min(a : f64, b : f64) -> f64{
    if a < b {
        a
    }
    else {
        b
    }
}

pub fn clamp(val : f64, low : f64, high : f64) -> f64{
    min(high, max(low, val))    
}

pub fn soft_min(a : f64, b : f64, k : f64) -> f64 {
    let h = max(k - (a-b).abs(), 0.0)/k;
    min(a,b) - h*h*h*k/6.0
}

pub fn map(val:f64, ai:f64, bi:f64, af:f64, bf:f64) -> f64 {
    af + (val - ai) / (bi-ai) * (bf - af)
}
