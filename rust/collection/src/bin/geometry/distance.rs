/**
 * @cpg_dirspec distance
 *
 * cpg run -p src/bin/geometry/distance.rs
 */
use collection::{geo_lib::*, utils::read};

/**
 * 距離
 *
 * https://onlinejudge.u-aizu.ac.jp/courses/library/4/CGL/2/CGL_2_D
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
        println!("{:.10}", VectorFns::distance_ss(v1, v2, u1, u2));
    }
}
