pub fn manhattan(x1: u32, y1: u32, x2: u32, y2: u32) -> f64 {
    (x1 as f64 - x2 as f64).abs()
        + (y1 as f64 - y2 as f64).abs()
}

pub fn chebyshev(x1: u32, y1: u32, x2: u32, y2: u32) -> f64 {
    (x1 as f64 - x2 as f64)
        .abs()
        .max((y1 as f64 - y2 as f64).abs())
}

pub fn euclidean(x1: u32, y1: u32, x2: u32, y2: u32) -> f64 {
    (
        (x1 as f64 - x2 as f64).powf(2.)
            + (y1 as f64 - y2 as f64).powf(2.)
    ).sqrt()
}
