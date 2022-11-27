use super::{
    intersect::*,
    polygon::prelude::Polygon,
    prelude::Circle,
    vector::{place::*, prelude::*},
};
use std::{f64::consts::PI, ops::Sub};
/**
 * ２つ円の共通部分の面積
 */
// 0<θ<=π 扇型の面積から三角形を引く
// π<θ<2π 扇型の面積に三角形を足す
pub fn area_of_two_cricles(c1: Circle, c2: Circle) -> f64 {
    let pos = is_intersect_circles(c1, c2);
    if pos == 3 || pos == 4 {
        // 外接 or 交わらない
        return 0.;
    } else if pos == 0 || pos == 1 {
        // 内接 or 内包
        if c1.r < c2.r {
            return c1.area();
        } else {
            return c2.area();
        }
    }

    // 交わる
    let cps = points_at_intersection_circles(c1, c2);
    let (cp1, cp2) = (cps[0], cps[1]);

    let place1 = place(c1.c, cp1, cp2);
    let place2 = place(c2.c, cp1, cp2);

    let s1 = cross(cp2 - c1.c, cp1 - c1.c).abs() * 0.5;
    let ang1 = cross(cp1 - c1.c, cp2 - c1.c) // 余弦定理 acos では精度が悪かった
        .atan2(dot(cp1 - c1.c, cp2 - c1.c))
        .abs();
    let cs1 = c1.r * c1.r * ang1 * 0.5;

    let s2 = cross(cp2 - c2.c, cp1 - c2.c).abs() * 0.5;
    let ang2 = cross(cp1 - c2.c, cp2 - c2.c)
        .atan2(dot(cp1 - c2.c, cp2 - c2.c))
        .abs();
    let cs2 = c2.r * c2.r * ang2 * 0.5;

    if place1 * place2 == 3 {
        // 交点を結んだ線分が円の中心間の間にある
        cs1 + cs2 - s1 - s2
    } else if c1.r < c2.r {
        // c1 の中心が線分を超えて、c2 側にある
        let cs1 = c1.r * c1.r * (2. * PI - ang1) * 0.5;
        cs1 + cs2 + s1 - s2
    } else {
        // c2 の中心が線分を超えて、c1 側にある
        let cs2 = c2.r * c2.r * (2. * PI - ang2) * 0.5;
        cs1 + cs2 - s1 + s2
    }
}

/**
* 円と多角形の交わる部分の面積
*/
// 各頂点、交点を使って三角形と扇型の面積どっちを採用するかどうかは図から考える
pub fn area_of_circle_polygon(c: Circle, p: Polygon) -> f64 {
    #[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
    enum TYPE {
        Cross,
        Edge,
    }
    let mut ps = vec![];
    let mut area = 0.0;
    let n = p.len();
    // まず多角形の頂点と円との交点とでできる多角形の頂点を求める（多角形の頂点と多角形と円とでできる交点全て）
    for i in 0..n {
        let ni = (i + 1) % n; // 最後は cn0 になる

        // 交わる (=多角形の辺が円に出入りする) (接点も考慮)
        points_at_intersection_segment_from_two_vectors(c, p[i], p[ni])
            .iter()
            .filter(|cp| !cp.x.is_nan() && !cp.y.is_nan())
            .for_each(|&x| ps.push((x, TYPE::Cross, true)));

        let is_in = p[ni].sub(c.c).norm() <= c.r;
        ps.push((p[ni], TYPE::Edge, is_in));
    }
    // ps.iter().for_each(|x| println!("ps {:?}", &x));

    let m = ps.len();
    for i in 0..m {
        let ni = (i + 1) % m; // 最後は cm0 になる
        let (p, _, is_in) = ps[i];
        let (np, _, nis_in) = ps[ni];
        if is_in && nis_in {
            // どっちも内側にある
            area += cross(p - c.c, np - c.c) * 0.5;
        } else {
            // 一つでも外側にある
            let ang = cross(p - c.c, np - c.c).atan2(dot(p - c.c, np - c.c));
            area += c.r * c.r * ang * 0.5;
        }
    }
    area
}
