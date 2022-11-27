use super::{prelude::Polygon, vector::prelude::*};
// それぞれのベクトルは先頭から順に並んでいるもとのする（p1,p2,...,pn-1,pn）
pub fn area(p: Polygon) -> f64 {
    let n = p.len();
    let mut area = 0.0;
    // 順に二つずつとって外積の 1/2 を求めてその総和が多角形の面積になる
    for i in 0..n {
        let ni = (i + 1) % n; // 最後は 0n1 になる
        area += cross(p[i], p[ni]) / 2.0;
    }
    area
}
