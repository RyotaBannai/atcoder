/**
 * @cpg_dirspec parallel_orthogonal
 *
 * cpg run -p src/bin/geometry/parallel_orthogonal.rs
 */
use collection::{geometry::*, utils::read::*};

/**
 * 平行・垂直
 *
 * https://onlinejudge.u-aizu.ac.jp/courses/library/4/CGL/2/CGL_2_A
 *
 * tags: #parallel #orthogognal
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
        if VectorFns::is_parallel(v1, v2, u1, u2) {
            println!("2");
        } else if VectorFns::is_orthogonal(v1, v2, u1, u2) {
            println!("1");
        } else {
            println!("0");
        }
    }
}
