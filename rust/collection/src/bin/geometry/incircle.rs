/**
 * @cpg_dirspec incircle
 *
 * cpg run -p src/bin/geometry/incircle.rs
 */
use library::{
    geometry::{circle::other::*, vector::prelude::Vector},
    utils::read::*,
};

/**
 * 内接円
 *
 * https://onlinejudge.u-aizu.ac.jp/courses/library/4/CGL/7/CGL_7_B
 *
 */

fn main() {
    let mut p = vec![];
    for _ in 0..3 {
        let a = read::<f64>();
        p.push(Vector::new(a[0], a[1]));
    }

    let c = incircle(p[0], p[1], p[2]);
    println!("{} {} {}", c.c.x, c.c.y, c.r);
}
