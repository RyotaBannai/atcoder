/**
 * @cpg_dirspec intersect_circles
 *
 * cpg run -p src/bin/geometry/intersect_circles.rs
 */
use library::{
    geometry::{
        circle::{intersect::*, prelude::*},
        vector::prelude::Vector,
    },
    utils::read::*,
};

/**
 * 円の交差判定
 *
 * https://onlinejudge.u-aizu.ac.jp/courses/library/4/CGL/7/CGL_7_A
 *
 */

fn main() {
    let c1 = read::<f64>();
    let c2 = read::<f64>();

    println!(
        "{:.10}",
        is_intersect_circles(
            Circle::new(Vector::new(c1[0], c1[1]), c1[2]),
            Circle::new(Vector::new(c2[0], c2[1]), c2[2])
        )
    );
}
