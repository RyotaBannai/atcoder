/**
 * @cpg_dirspec segment_intersections_manhattan_geometry
 *
 * cpg run -p src/bin/geometry/segment_intersections_manhattan_geometry.rs
 */
use collection::{geo_lib::*, utils::read};

/**
 * 線分交差問題
 *
 * https://onlinejudge.u-aizu.ac.jp/courses/library/4/CGL/6/CGL_6_A
 *
 * tags: #線分交差問題 #マンハッタン幾何
 *
 * マンハッタン幾何：線分が軸に平行な線分交差問題, 平面走査（sweep）で解く
 * 平面走査：x軸（またはy軸）に並行な直線（走査線）を上へ（右へ）向かって並行移動させながら、交点を見つけていく,
 *
 */

fn main() {
    let n = read::<usize>()[0];
    let mut segs = vec![];
    for _ in 0..n {
        let a = read::<f64>();
        segs.push((Vector::new(a[0], a[1]), Vector::new(a[2], a[3])));
    }

    let ans = manhattan_geo::plane_sweep(segs);
    println!("{}", ans);
}
