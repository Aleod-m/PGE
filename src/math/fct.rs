pub fn fast_floor(x : f32) -> i32 {
    let xi = x as i32;
    if x < xi as f32 {
        return xi - 1;
    }
    return xi;
}

pub fn soft_min(a : f32, b : f32, k : f32) -> f32 {
    let h = (k - (a-b).abs()).max(0.0)/k;
    a.min(b) - h*h*h*k/6.0
}

pub fn map(val:f32, ai:f32, bi:f32, af:f32, bf:f32) -> f32 {
    af + (val - ai) / (bi-ai) * (bf - af)
}
