/**
 * @cpg_dirspec intersect
 *
 * cpg run -p src/bin/geometry/intersect.rs
 */
use collection::{geometry::*, utils::read::*};

/**
 * 交差判定
 *
 * https://onlinejudge.u-aizu.ac.jp/courses/library/4/CGL/2/CGL_2_B
 *
 */

fn main() {
    let n = read::<usize>()[0];
    for _ in 0..n {
        let a = read::<f64>();
        let (v1, v2, u1, u2) = (
            Vector::new(a[0], a[1]),
            Vector::new(a[2], a[3]),
            Vector::new(a[4], a[5]),
            Vector::new(a[6], a[7]),
        );
        println!(
            "{}",
            if VectorFns::intersect(v1, v2, u1, u2) {
                "1"
            } else {
                "0"
            }
        );
    }
}
