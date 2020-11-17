pub fn fast_floor(x : f64) -> i32 {
    let xi = x as i32;
    if x < xi as f64 {
        return xi - 1;
    }
    return xi;
}

pub fn soft_min(a : f64, b : f64, k : f64) -> f64 {
    let h = (k - (a-b).abs()).max(0.0)/k;
    a.min(b) - h*h*h*k/6.0
}

pub fn map(val:f64, ai:f64, bi:f64, af:f64, bf:f64) -> f64 {
    af + (val - ai) / (bi-ai) * (bf - af)
}
