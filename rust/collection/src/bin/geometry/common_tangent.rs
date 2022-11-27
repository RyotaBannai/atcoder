/**
 * @cpg_dirspec common_tangent
 *
 * cpg run -p src/bin/geometry/common_tangent.rs
 */
use library::{
    geometry::{
        circle::{prelude::Circle, tangent::*},
        vector::prelude::Vector,
    },
    utils::read::*,
};

/**
 * 円の共通接線
 *
 * https://onlinejudge.u-aizu.ac.jp/courses/library/4/CGL/7/CGL_7_G
 *
 */

fn main() {
    let a = read::<f64>();
    let b = read::<f64>();
    let c1 = Circle::new(Vector::new(a[0], a[1]), a[2]);
    let c2 = Circle::new(Vector::new(b[0], b[1]), b[2]);
    let mut res = tangent_circle(c1, c2);
    res.sort();

    for tp in res {
        println!("{:.10} {:.10}", tp.x, tp.y);
    }
}
