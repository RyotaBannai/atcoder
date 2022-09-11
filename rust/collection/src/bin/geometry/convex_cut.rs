/**
 * @cpg_dirspec convex_cut
 *
 * cpg run -p src/bin/geometry/convex_cut.rs
 */
use collection::{geo_lib::*, utils::read};

/**
 * 凸多角形の切断
 *
 * https://onlinejudge.u-aizu.ac.jp/courses/library/4/CGL/4/CGL_4_C
 *
 * tags: #convex_cut
 *
 */

fn main() {
    let n = read::<usize>()[0];
    let mut p = vec![];
    for _ in 0..n {
        let a = read::<f64>();
        p.push(Vector::new(a[0], a[1]));
    }

    let q = read::<usize>()[0];
    for _ in 0..q {
        let a = read::<f64>();
        let ans =
            PolygonFns::convex_cut(p.clone(), Vector::new(a[0], a[1]), Vector::new(a[2], a[3]));
        println!("{:.8}", ans);
    }
}
