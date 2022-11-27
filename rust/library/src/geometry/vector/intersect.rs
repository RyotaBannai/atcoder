use super::{place::*, prelude::*};
use std::{
    f64::NAN,
    ops::{Add, Mul, Sub},
};
// ２つの線分が交差するかどうか判定
pub fn intersect(v1: Vector, v2: Vector, u1: Vector, u2: Vector) -> bool {
    let place1 = place(u1, v1, v2);
    let place2 = place(u2, v1, v2);
    let place3 = place(v1, u1, u2);
    let place4 = place(v2, u1, u2);

    if vec![place1, place2, place3, place4]
        .iter()
        .any(|&p| p == 7 || p == 11 || p == 17 || p == 19 || p == 23)
    {
        // いずれかの点が一方の線分上にある
        true
    } else {
        // それぞれの端点が時計回りかつ反時計回りにある
        place1 * place2 == 3 && place3 * place4 == 3
        // 1*1*3*3 の場合もあるから place1 * place2 * place3 * place4 == 9 とはしない
    }
}
// ２つの線分の交点
// 交差しない場合は Vector{NAN,NAN} を返す
pub fn point_at_intersection_on_ss(v1: Vector, v2: Vector, u1: Vector, u2: Vector) -> Vector {
    if !intersect(v1, v2, u1, u2) {
        return Vector::new(NAN, NAN);
    }
    point_at_intersection_on_ll(v1, v2, u1, u2)
}

// ２つの直線の交点
pub fn point_at_intersection_on_ll(v1: Vector, v2: Vector, u1: Vector, u2: Vector) -> Vector {
    let base = v2.sub(v1);
    // 高さ // 二つの平行四辺形を底辺 base で割る
    let h1 = (cross(base, u1.sub(v1)) / norm(base)).abs(); // 絶対値 // |base| は t の計算で約分するから省略可
    let h2 = (cross(base, u2.sub(v1)) / norm(base)).abs();
    // base にならない方の線分の交点までの比は t: 1-t = h1: h2
    let t = h1 / (h1 + h2);
    let nv = u2.sub(u1).mul(t);
    u1.add(nv)
}
