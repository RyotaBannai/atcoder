use super::vector::prelude::*;
use std::f64::consts::PI;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Circle {
    pub c: Vector,
    pub r: f64,
}
impl Circle {
    pub fn new(c: Vector, r: f64) -> Self {
        Self { c, r }
    }
    pub fn area(&self) -> f64 {
        self.r * self.r * PI
    }
}
