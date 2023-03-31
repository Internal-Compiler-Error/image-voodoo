/// Interpolation functions
pub trait BiInterpolation {
    /// Interpolates the value of f(x, y) given the values of f(x0, y0), f(x1, y0), f(x0, y1), and
    /// f(x1, y1).
    fn interpolate(
        &self,
        x: f64,
        y: f64,
        x0: u32,
        y0: u32,
        x1: u32,
        y1: u32,
        f: &dyn Fn(u32, u32) -> u8,
    ) -> u8;
}

pub struct BiLinearInterpolation;

impl BiInterpolation for BiLinearInterpolation {
    fn interpolate(
        &self,
        x: f64,
        y: f64,
        x0: u32,
        y0: u32,
        x1: u32,
        y1: u32,
        f: &dyn Fn(u32, u32) -> u8,
    ) -> u8 {
        bilinear_interpolation(x, y, x0, y0, x1, y1, f)
    }
}

pub struct BiNearestNeighbourInterpolation;
impl BiInterpolation for BiNearestNeighbourInterpolation {
    fn interpolate(&self, x: f64, y: f64, x0: u32, y0: u32, x1: u32, y1: u32, f: &dyn Fn(u32, u32) -> u8) -> u8 {
       bi_nearest_neighbour_interpolation(x, y, x0, y0, x1, y1, f)
    }
}

pub(crate) fn linear_interpolation(x: f64, x0: u32, x1: u32, f0: f64, f1: f64) -> f64 {
    assert!(x0 as f64 <= x && x <= x1 as f64);

    // technically, linear interpolation is not defined when x0 == x1, but we can just return f0
    // since we are really asking for the value of f(x) at x and x0 == x1 means that f(x) is a
    // constant
    if x0 == x1 {
        return f0 as f64;
    }

    let f0_weight: f64 = (x1 as f64 - x) as f64 / (x1 - x0) as f64;
    let f1_weight: f64 = 1f64 - f0_weight;

    f0 as f64 * f0_weight + f1 as f64 * f1_weight
}

pub(crate) fn bilinear_interpolation(
    x: f64,
    y: f64,
    x0: u32,
    y0: u32,
    x1: u32,
    y1: u32,
    f: &dyn Fn(u32, u32) -> u8,
) -> u8 {
    assert!(x0 as f64 <= x && x <= x1 as f64);
    assert!(y0 as f64 <= y && y <= y1 as f64);

    let f00: u8 = f(x0, y0);
    let f01: u8 = f(x0, y1);
    let f10: u8 = f(x1, y0);
    let f11: u8 = f(x1, y1);

    // interpolate between f00 and f01
    let f0 = linear_interpolation(y, y0, y1, f00 as f64, f01 as f64);

    // interpolate between f10 and f11
    let f1 = linear_interpolation(y, y0, y1, f10 as f64, f11 as f64);

    // interpolate between f0 and f1
    linear_interpolation(x, x0, x1, f0, f1)
        .clamp(0f64, 255f64)
        .round() as u8
}

pub(crate) fn nearest_neighbour_interpolation(x: f64, x0: u32, x1: u32, f0: u8, f1: u8) -> u8 {
    assert!(x0 as f64 <= x && x <= x1 as f64);

    let x0_dist = x - x0 as f64;
    let x1_dist = x1 as f64 - x;

    if x0_dist < x1_dist {
        f0
    } else {
        f1
    }
}

pub(crate) fn bi_nearest_neighbour_interpolation(
    x: f64,
    y: f64,
    x0: u32,
    y0: u32,
    x1: u32,
    y1: u32,
    f: &dyn Fn(u32, u32) -> u8,
) -> u8 {
    assert!(x0 as f64 <= x && x <= x1 as f64);
    assert!(y0 as f64 <= y && y <= y1 as f64);

    let f00: u8 = f(x0, y0);
    let f01: u8 = f(x0, y1);
    let f10: u8 = f(x1, y0);
    let f11: u8 = f(x1, y1);

    // interpolate between f00 and f01
    let f0 = nearest_neighbour_interpolation(y, y0, y1, f00, f01);

    // interpolate between f10 and f11
    let f1 = nearest_neighbour_interpolation(y, y0, y1, f10, f11);

    // interpolate between f0 and f1
    nearest_neighbour_interpolation(x, x0, x1, f0, f1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linear_interpolation_on_identify() {
        let f = |x: u32| x as u8;

        let x0 = 0u32;
        let x1 = 99u32;
        let y0 = f(x0);
        let y1 = f(x1);

        for x in x0..=x1 {
            let real = f(x);
            let interpolated = linear_interpolation(x as f64, x0, x1, y0.into(), y1.into());

            let diff = (real as i32 - interpolated as i32).abs();
            if diff > 0 {
                println!("x: {}, real: {}, interpolated: {}", x, real, interpolated);
            }

            assert!(diff <= 1);
        }
    }

    #[test]
    fn linear_interpolation_on_2x() {
        let f = |x: u32| 2 * x as u8;
        let x0 = 0u32;
        let x1 = 99u32;
        let y0 = f(x0);
        let y1 = f(x1);

        for x in 0..=15 {
            let real = f(x);
            let interpolated = linear_interpolation(x as f64, x0, x1, y0.into(), y1.into());

            let diff = (real as i32 - interpolated as i32).abs();
            if diff > 0 {
                println!("x: {}, real: {}, interpolated: {}", x, real, interpolated);
            }

            assert!(diff <= 1);
        }
    }

    #[test]
    fn bilinear_interpolation_on_identity() {
        let f = |x: u32, y: u32| x as u8 + y as u8;
        let x0 = 0u32;
        let x1 = 99u32;
        let y0 = 0u32;
        let y1 = 99u32;

        for i in 0..=99 {
            let real = f(i, i);
            let interpolated = bilinear_interpolation(i as f64, i as f64, x0, y0, x1, y1, &f);

            let diff = (real as i32 - interpolated as i32).abs();
            if diff > 0 {
                println!("x: {}, real: {}, interpolated: {}", i, real, interpolated);
            }
            assert!(diff <= 1);
        }
    }
}
