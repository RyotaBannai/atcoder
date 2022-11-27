use super::{intersect::*, prelude::*};
use std::ops::Sub;
// ベクトル間の距離
pub fn distance_vv(v1: Vector, v2: Vector) -> f64 {
    abs(v1, v2)
}
/*
ベクトルと直線間の距離
直線の場合、線分と違って交わるかどうかは気にしなくて良いため、平行四辺形を作って垂直線を下ろした時の距離（最短距離）がベクトルと直線間の距離になる
線分の場合、交わるかどうかはわからないため追加の処理がいる
*/
pub fn distance_lv(v: Vector, v1: Vector, v2: Vector) -> f64 {
    let nv = v2.sub(v1);
    let nu = v.sub(v1);
    (cross(nv, nu) / norm(nv)).abs() // 絶対値の abs
}
/*
ベクトルと線分間の距離 p382
線分の端点 v1,v2 について、
v の位置が v1 からの角度が 90/-90 以上であれば、v1 より後退した場所に位置しているため、最短距離は v1 との距離になる
同様に v2
それ以外は、ベクトル v は v1,v2 の間に位置しているため。これは線分とベクトルの距離
なす角が 90/-90 の時、内積は負（cosθより）
*/
pub fn distance_sv(v: Vector, v1: Vector, v2: Vector) -> f64 {
    if dot(v2.sub(v1), v.sub(v1)) < 0.0 {
        // v1 を始点に試す
        // 2点間の距離
        norm(v.sub(v1))
    } else if dot(v1.sub(v2), v.sub(v2)) < 0.0 {
        // v2 を始点に試す
        // 2点間の距離
        norm(v.sub(v2))
    } else {
        // 線分との距離
        distance_lv(v, v1, v2)
    }
}
// 線分間の距離
pub fn distance_ss(v1: Vector, v2: Vector, u1: Vector, u2: Vector) -> f64 {
    if intersect(v1, v2, u1, u2) {
        // 交差するなら距離 0
        0.0
    } else {
        // 各線分に対して、一方の端点からの距離の最小が線分どうしの距離となる
        let mut mi = std::f64::MAX;
        for &d in &[
            distance_sv(u1, v1, v2),
            distance_sv(u2, v1, v2),
            distance_sv(v1, u1, u2),
            distance_sv(v2, u1, u2),
        ] {
            if d < mi {
                mi = d
            }
        }
        mi
    }
}
