pub fn lerp(x0: f64, x1: f64, t: f64) -> f64 {
    let ti = 1. - t;
    x0 * ti + x1 * t
}

pub fn nearest_neighbor(x0: f64, x1: f64, t: f64) -> f64 {
    if t < 0.5 {
        x0
    } else {
        x1
    }
}