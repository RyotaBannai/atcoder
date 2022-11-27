use super::prelude::*;
use std::ops::{Add, Mul, Sub};

/*
直行する時、内積は 0
ベクトル単体は原点をベースに考えているから、線分なら始点と終点の２点から計算する
*/
pub fn is_orthogonal(v1: Vector, v2: Vector, u1: Vector, u2: Vector) -> bool {
    let eps = 1e-10;
    let nv = v1.sub(v2);
    let nu = u1.sub(u2);
    dot(nv, nu).abs() < eps
}
pub fn is_parallel(v1: Vector, v2: Vector, u1: Vector, u2: Vector) -> bool {
    let eps = 1e-10;
    let nv = v1.sub(v2);
    let nu = u1.sub(u2);
    cross(nv, nu).abs() < eps
}
pub fn projection(v: Vector, v1: Vector, v2: Vector) -> Vector {
    let base = v2.sub(v1);
    let hypo = v.sub(v1);
    let t = dot(hypo, base) / norm(base); // 正射影のノルム
    let r = t / norm(base); // 正射影のノルムと base の比を取って、それを v1 から伸ばす
    v1.add(base.mul(r))
}
pub fn reflection(v: Vector, v1: Vector, v2: Vector) -> Vector {
    v.add((projection(v, v1, v2).sub(v)).mul(2.0))
}
