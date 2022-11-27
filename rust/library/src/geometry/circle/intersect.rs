use super::{
    linear_equation::prelude::LinearEquation,
    prelude::Circle,
    vector::{distance::*, other::*, place::*, polar::*, prelude::*},
};
use std::{
    f64::NAN,
    ops::{Add, Mul, Sub},
};
/**
 * 円と直線が交差するかどうか判定（線分で端点が円の中にある場合は考慮しない）
 * ベクトルから円の中心までの距離が、円の半径より小さければok
 */
pub fn is_intersect_line(c: Circle, v1: Vector, v2: Vector) -> bool {
    let d = distance_lv(c.c, v1, v2);
    // 接する場合も true
    d <= c.r
}
/**
* 二つの円が交差するかどうか判定
*
* 0: 一方がもう一方を内包する場合（共通接線がない場合）
* 1: それらが内接する場合（共通接線の数が 1 の場合）
* 2: それらが交わる場合（共通接線の数が 2 の場合）
* 3: それらが外接する場合（共通接線の数が 3 の場合）
* 4: それらが離れている場合（共通接線の数が 4 の場合）
*/
pub fn is_intersect_circles(c1: Circle, c2: Circle) -> usize {
    let eps = 1e-10;
    let nv = c2.c.sub(c1.c); // c1 の中心から c2 の中心へのベクトル
    let d = norm(nv);
    if (d - (c1.r + c2.r)).abs() < eps {
        // 外接
        3
    } else if ((d + c1.r) - c2.r).abs() < eps || ((d + c2.r) - c1.r).abs() < eps {
        // いずれかが、内接
        1
    } else if d + c1.r < c2.r || d + c2.r < c1.r {
        // 内包
        0
    } else if d > c1.r + c2.r {
        // 離れている
        4
    } else {
        // 交わる
        2
    }
}
/**
* 円と線分との交点座標
* 線分は二つのベクトルとして与える
* 交差しない場合は vec![Vector{NAN,NAN}] を返す
*
* 直線で交点があっても線分上にない場合は返さない
*/
pub fn points_at_intersection_segment_from_two_vectors(
    c: Circle,
    v1: Vector,
    v2: Vector,
) -> Vec<Vector> {
    let cp = points_at_intersection_line_from_two_vectors(c, v1, v2);
    // 交差しないならそのまま返す
    if cp[0].x.is_nan() || cp[0].y.is_nan() {
        return cp;
    }
    let mut res = vec![];
    let p1 = place(cp[0], v1, v2);
    let p2 = place(cp[1], v1, v2);
    if vec![7, 11, 17, 23].iter().any(|&x| x == p1) {
        res.push(cp[0]);
    }
    if vec![7, 11, 17, 23].iter().any(|&x| x == p2) {
        res.push(cp[1]);
    }
    res
}
/**
* 円と直線との交点座標
* 直線は二つのベクトルとして与える
* 交差しない場合は vec![Vector{NAN,NAN}] を返す
*
* ２つ交点がある場合は、v1 に近い方を先に返す
*/
pub fn points_at_intersection_line_from_two_vectors(
    c: Circle,
    v1: Vector,
    v2: Vector,
) -> Vec<Vector> {
    if !is_intersect_line(c, v1, v2) {
        return vec![Vector::new(NAN, NAN)];
    }
    let pr = projection(c.c, v1, v2);
    let e = unit(v1, v2);
    let nv = pr.sub(c.c);
    let base = (c.r * c.r - dot(nv, nv)).sqrt(); // base: 直線の円の内部における長さの1/2点間の距離
    let nu = e.mul(base); // unit に大きさ base をかけると交点に向けたベクトルになる. それを、正射影のベクトルに加えると交点のベクトルになる
    vec![pr.sub(nu), pr.add(nu)]
}
/**
* 円と直線との交点座標
* 直線は一時関数として与える
* 交差しない場合は vec![Vector{NAN,NAN}] を返す
*/
pub fn points_at_intersection_line_from_le(c: Circle, mut le: LinearEquation) -> Vec<Vector> {
    le = le.normalize().unwrap();
    let (a, b, k, x0, y0, r) = (le.a, le.b, le.k, c.c.x, c.c.y, c.r);
    let d = (a * x0 + b * y0 + k).abs(); // 正規化したから、分母は 1.
    if d > r {
        return vec![Vector::new(NAN, NAN)];
    }
    let cmn = (c.r * c.r - d * d).sqrt();
    vec![
        Vector::new(a * d - b * cmn + x0, b * d + a * cmn + y0),
        Vector::new(a * d + b * cmn + x0, b * d - a * cmn + y0),
    ]
}
/**
* 円と直線との交点座標
* 直線は一時関数として与える
* 交差しない場合は vec![Vector{NAN,NAN}] を返す
*
* points_at_intersection_line_from_le と同様の結果を期待できる
*/
fn points_at_intersection_line_from_le_2(c: Circle, le: LinearEquation) -> Vec<Vector> {
    let mut res = Vec::with_capacity(2);
    // 単位ベクトルとベクトルの内積をすれば、|base|=1 となって計算が楽
    let r = le.e.dot(c.c - le.v);
    let p_mid = le.v + le.e * r;
    let d = (c.r * c.r - dot(p_mid - c.c, p_mid - c.c)).sqrt();
    if d > 0. {
        // d == 0. の場合（点で交わる場合）は二つとも同じ点になるが二つ入れる
        res.push(p_mid + le.e * d);
        res.push(p_mid - le.e * d);
    } else {
        res.push(Vector::new(NAN, NAN));
    }
    res
}

/**
 * 二つの円の交点座標
 * 交差しない場合は vec![Vector{NAN,NAN}] を返す
 */
pub fn points_at_intersection_circles(c1: Circle, c2: Circle) -> Vec<Vector> {
    let nv = c2.c.sub(c1.c); // c1 の中心から c2 の中心へのベクトル
    let d = norm(nv);
    if d > c1.r + c2.r {
        return vec![Vector::new(NAN, NAN)];
    }
    // 余弦定理より半径 c1.r, c2.r, d(c2.c - c1.c)を用いて r1 と d がなす角 a を求める（ベクトル r は中心からのベクトルを意味する）(接する時は cosθ=0 )
    let a = ((c1.r * c1.r + d * d - c2.r * c2.r) / (2.0 * c1.r * d)).acos();
    //  x 軸と d のなす角
    let t = nv.y.atan2(nv.x);
    // ここではベクトル{x,y} を回転した結果を c の中心に加えてるわけではない.
    vec![
        c1.c.add(polar_on_r(c1.r, t + a)), // 反時計回りに回転、正
        c1.c.add(polar_on_r(c1.r, t - a)), // 時計回りに回転したい、負
    ]
}
