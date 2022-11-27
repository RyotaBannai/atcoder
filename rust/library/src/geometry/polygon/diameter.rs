use super::{convex_hull::*, prelude::Polygon, vector::prelude::*};
use std::ops::Sub;
/**
 * 多角形から直径を求める
 */
pub fn diameter(p: Polygon) -> f64 {
    // 凸包
    let (up, mut low) = convex_hull(p);
    let mut vs = up;
    let wl = low.len();
    vs.append(&mut low[1..wl - 1].to_vec());

    let n = vs.len();
    let mut ma = 0.;
    let (mut pos1, mut pos2) = (0, 1); // vs の index // pos1 は後ろ、pos2 は前
    let (mut ma_pos1, mut ma_pos2) = (0, 1); // vs の index

    loop {
        let d = vs[pos2].sub(vs[pos1]).norm();
        if ma < d {
            ma = d;
            ma_pos1 = pos1;
            ma_pos2 = pos2;
        };

        let s1 = vs[(pos1 + 1) % n].sub(vs[pos1]);
        let s2 = vs[(pos2 + 1) % n].sub(vs[pos2]);

        if cross(s1, s2) > 0. {
            pos1 = (pos1 + 1) % n;
        } else {
            pos2 = (pos2 + 1) % n;
        }

        if pos1 == ma_pos1 && pos2 == ma_pos2 {
            break;
        }
    }
    ma
}
