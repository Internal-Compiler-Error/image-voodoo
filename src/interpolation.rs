/// LinEar intERPolation, follows the standard formula
/// it's called LERP in the game industry so I'll stick with that
pub fn lerp(x0: f64, x1: f64, t: f64) -> f64 {
    let ti = 1. - t;
    x0 * ti + x1 * t
}

/// Nearest Neighbor interpolation, returns x0 if t < 0.5, x1 otherwise
pub fn nearest_neighbor(x0: f64, x1: f64, t: f64) -> f64 {
    if t < 0.5 {
        x0
    } else {
        x1
    }
}