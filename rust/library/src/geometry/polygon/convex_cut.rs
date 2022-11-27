use super::{
    area::*,
    prelude::Polygon,
    vector::{distance::*, intersect::*, place::*, prelude::*},
};
use std::ops::{Add, Mul, Sub};
/**
 * 凸多角形を２つにカットしたときのそれぞれの面積を求める
 * 与える点は反時計回りになっていること
 */
pub fn convex_cut(p: Polygon, v1: Vector, v2: Vector) -> f64 {
    // 初めに凸多角形を cut するときの線分の端点が凸多角形の内側に入っていないことを保証する（直線にする）
    let nv1 = v1.add(v1.sub(v2).mul(10000.));
    let nv2 = v2.add(v2.sub(v1).mul(10000.));

    let n = p.len();
    let mut cc = 0; // 交差回数
    let mut vs = vec![p[0]]; // 半時計回りに探索した時にキープする多角形の頂点
    let mut edge = 0; // cut が凸多角形の頂点一点で交わる時の頂点

    for i in 0..n {
        let cur = p[i];
        let next = p[(i + 1) % n];

        // 1. 辺が cut 上にある時(=辺の両端点が直線上にある時)は、多角形を分割できないから、面積か 0. を返す
        let dist1 = distance_lv(cur, nv1, nv2);
        let dist2 = distance_lv(next, nv1, nv2);
        if dist1 == 0. && dist2 == 0. {
            // 頂点が半時計回りに振られているから、内積<0. なら左手に面積は無いから 0., 内積>0. なら全ての面積を返す
            let cos = dot(next.sub(cur), nv2.sub(nv1));
            if cos < 0. {
                return 0.;
            } else {
                return area(p);
            }
        }

        // 2. 辺と cut が交わる時
        if intersect(nv1, nv2, cur, next) {
            let cross_point = point_at_intersection_on_ll(nv1, nv2, cur, next);

            // cut と線分の頂点で交わる時に、その頂点において、2回 push しないようにしたい
            // 交点が端点の前にある時のみ push することで後ろにある時はすでに対応済みとする
            // cut と頂点が交わらない場合は、普通の線分と直線の交点として push できる
            if cross_point != p[i] {
                vs.push(cross_point);
                cc += 1;
            }

            // cut が頂点と１回だけ交わる場合に対応したい
            // 2回目の線分と交わった時に頂点番号を記憶
            if cross_point == p[i] && cc % 2 == 1 {
                edge = i;
            }
        }

        // 管理してる多角形を分割している時は追加しない（凸多角形と cut はちょうど２点で交わる）
        if cc % 2 == 0 {
            vs.push(p[(i + 1) % n]);
        }
    }

    // 頂点と一回だけ交わるときは、多角形を分割できていない
    // 頂点から辺が出て行く時に、cut を始軸として
    // 時計回りにベクトルが出て行く時は、左手に面積が無い
    // 半時計回りにベクトルが出て行く時は、左手に面積がある
    if cc % 2 == 1 {
        let cur = p[edge];
        let next = p[(edge + 1) % n];
        let sin = cross(nv2.sub(nv1), next.sub(cur)); // v1,v2 の配置に注意
        if sin < 0. {
            return 0.;
        } else {
            return area(p);
        }
    }

    let place = place(p[0], nv1, nv2);
    let a = area(p);
    let part = area(vs);
    // カットした面積が左側なら、そのまま返してそうでなければ右側を返す
    if place == 1 {
        part
    } else {
        a - part
    }
}
