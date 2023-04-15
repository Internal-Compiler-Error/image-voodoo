#![allow(dead_code)]

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub enum Distance {
    Manhattan,
    Chebyshev,
    Euclidean,
}

/// Distance metric on R^2
pub trait DistanceMetric {
    fn distance(&self, x1: u32, y1: u32, x2: u32, y2: u32) -> f64;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct Manhattan;

impl Manhattan {
    pub fn new() -> Self {
        Self
    }
}

impl DistanceMetric for Manhattan {
    fn distance(&self, x1: u32, y1: u32, x2: u32, y2: u32) -> f64 {
        (x1 as f64 - x2 as f64).abs() + (y1 as f64 - y2 as f64).abs()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct Chebyshev;

impl Chebyshev {
    pub fn new() -> Self {
        Self
    }
}

impl DistanceMetric for Chebyshev {
    fn distance(&self, x1: u32, y1: u32, x2: u32, y2: u32) -> f64 {
        (x1 as f64 - x2 as f64)
            .abs()
            .max((y1 as f64 - y2 as f64).abs())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct Euclidean;

impl Euclidean {
    pub fn new() -> Self {
        Self
    }
}

impl DistanceMetric for Euclidean {
    fn distance(&self, x1: u32, y1: u32, x2: u32, y2: u32) -> f64 {
        ((x1 as f64 - x2 as f64).powf(2.) + (y1 as f64 - y2 as f64).powf(2.)).sqrt()
    }
}
