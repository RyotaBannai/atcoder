use super::vector::prelude::*;
/**
 * 二つのベクトルから傾きを求める
 */
pub fn slope(v1: Vector, v2: Vector) -> f64 {
    let eps = 1e-10;
    let dx = v2.x - v1.x;
    if dx.abs() < eps {
        0.
    } else {
        (v2.y - v1.y) / dx
    }
}
