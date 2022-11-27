use super::{
    intersect::*, linear_equation::prelude::LinearEquation, prelude::Circle, vector::prelude::*,
};
/**
 * 円と円の外にある点からの接線の接点を求める
 * v は「極」
 *
 * tangent_point_from_polar を使っても同じ結果が期待できる
 */
pub fn tangent_point_from_polar_2(c: Circle, v: Vector) -> Vec<Vector> {
    let center = v - c.c; // 新しい円の中心
    let r = (center.x * center.x + center.y * center.y - c.r * c.r).sqrt(); // 三平方の定理 新しい円の半径
    let new_c = Circle::new(v, r);
    points_at_intersection_circles(c, new_c) // 二つの円から交点を求める

    // or
    // let le = ommon_line(c, new_c); // 二つの交点を通る直線を求めてから、
    // points_at_intersection_line_from_le(c, le) // ax+by+k=0 の式を使って交点を求める
    // points_at_intersection_line_from_le_2(c, le) // ax+by+k=0 の式を使って交点を求める
}

/**
* ２つの円の接点を求める
*/
pub fn tangent_circle(c1: Circle, c2: Circle) -> Vec<Vector> {
    let (r1, r2, v) = (c1.r, c2.r, c2.c - c1.c);
    let d = v.norm(); // 中心間の距離
    let mut res = vec![];
    if d > r1 + r2 {
        // 共通内接線
        // 円が離れている
        let polar = c1.c + v * (r1 / (r1 + r2));
        let pt = tangent_point_from_polar_2(c1, polar);
        res.extend(pt);
    }
    if d == r1 + r2 || d == (r1 - r2).abs() {
        // c1 に対して、c2 は内接円 or 外接円
        // let le = common_line(c1, c2); // 共通接線
        // let pt = points_at_intersection_line_from_le(c1, le);
        let pt = points_at_intersection_circles(c1, c2);
        res.push(pt[0]); // 一点で交わるから一つだけ入れる
    }
    if d > (r1 - r2).abs() {
        // 共通外接線 (２つの円が交わる場合も考慮)
        if r1 == r2 {
            // 円の半径が同じ場合共通外接線は並行で交わらないから、中心から半径分垂直に移動した位置を直接求める
            let n = Vector::new(v.y / d, -v.x / d); // Normal (法線ベクトル)
            res.extend(vec![c1.c + n * r1, c1.c - n * r1]);
        } else {
            let polar = c1.c + v * (r1 / (r1 - r2));
            let pt = tangent_point_from_polar_2(c1, polar);
            res.extend(pt);
        }
    }
    res
}
/**
 * 円と円の外にある点からの接線の接点を求める
 * v は「極」
 */
pub fn tangent_point_from_polar(c: Circle, v: Vector) -> Vec<Vector> {
    let (x0, y0, r, x1, y1) = (c.c.x, c.c.y, c.r, v.x, v.y);
    // a,b,k は、極線が (x1-x0)(x-x0)+(y1-y0)(y-y0) = r*r で表されるから、x,yについて整理すると得られる
    let a = x1 - x0;
    let b = y1 - y0;
    let k = x0 * x0 + y0 * y0 - x1 * x0 - y1 * y0 - r * r;
    let le = LinearEquation::new(a, b, k); // 極線
    points_at_intersection_line_from_le(c, le)
}
/**
 * 二つの円から共通接線の方程式を求める
 * (x-a)2+(y-b)2=r2 を展開して、その二つの方程式の差が共通接線の方程式
 */
pub fn common_line(c1: Circle, c2: Circle) -> LinearEquation {
    let a = -2. * c1.c.x;
    let b = -2. * c1.c.y;
    let c = c1.c.x * c1.c.x + c1.c.y * c1.c.y - c1.r * c1.r;
    let d = -2. * c2.c.x;
    let e = -2. * c2.c.y;
    let f = c2.c.x * c2.c.x + c2.c.y * c2.c.y - c2.r * c2.r;
    LinearEquation::new(a - d, b - e, c - f)
}
