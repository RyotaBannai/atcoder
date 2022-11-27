use super::prelude::*;
// 半径 r はただの大きさで、大きさを１とした時に x=cosθ, y=sinθ で求められるのと同じ
// その場合は、X=xcosθ-ysinθ, Y=ycosθ+xsinθ (polar_on_v)
// https://mathwords.net/heimenkaiten
pub fn polar_on_r(r: f64, rad: f64) -> Vector {
    Vector::new(r * rad.cos(), r * rad.sin())
}

pub fn polar_on_v(v: Vector, rad: f64) -> Vector {
    Vector::new(
        v.x * rad.cos() - v.y * rad.sin(),
        v.y * rad.cos() + v.x * rad.sin(),
    )
}
