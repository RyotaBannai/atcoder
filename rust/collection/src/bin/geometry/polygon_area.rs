/**
 * @cpg_dirspec polygon_area
 *
 * cpg run -p src/bin/geometry/polygon_area.rs
 */
use collection::{geo_lib::*, utils::read};

/**
 * 面積
 *
 * https://onlinejudge.u-aizu.ac.jp/courses/library/4/CGL/3/CGL_3_A
 *
 * tags: #area #polygon
 *
 * 多角形の各ベクトルを順に二つずつとって外積の 1/2 を求めてその総和が多角形の面積になる
 * 余分な部分は内積のz成分が負となり、総和を求めることで相殺される
 *
 * 参考
 * https://imagingsolution.net/math/calc_n_point_area/
 */

fn main() {
    let n = read::<usize>()[0];
    let mut p = vec![];

    for _ in 0..n {
        let a = read::<f64>();
        p.push(Vector::new(a[0], a[1]));
    }

    println!("{:.1}", PolygonFns::area(p));
}
