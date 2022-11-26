/**
 * @cpg_dirspec outcircle
 *
 * cpg run -p src/bin/geometry/outcircle.rs
 */
use collection::{geometry::*, utils::read::*};

/**
 * 外接円
 *
 * tags: #circumscribred_circle_of_a_triangle
 *
 * https://onlinejudge.u-aizu.ac.jp/courses/library/4/CGL/7/CGL_7_C
 *
 */

fn main() {
    let mut p = vec![];
    for _ in 0..3 {
        let a = read::<f64>();
        p.push(Vector::new(a[0], a[1]));
    }

    let c = CircleFns::outcircle(p[0], p[1], p[2]);
    println!("{:.10} {:.10} {:.10}", c.c.x, c.c.y, c.r);
}
