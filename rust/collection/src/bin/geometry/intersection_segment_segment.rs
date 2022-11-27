/**
 * @cpg_dirspec intersection_segment_segment
 *
 * cpg run -p src/bin/geometry/intersection_segment_segment.rs
 */
use library::{
    geometry::vector::{intersect::*, prelude::Vector},
    utils::read::*,
};

/**
 * 交点
 *
 * https://onlinejudge.u-aizu.ac.jp/courses/library/4/CGL/2/CGL_2_C
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
        let ans = point_at_intersection_on_ss(v1, v2, u1, u2);
        println!("{:.10} {:.10}", ans.x, ans.y);
    }
}
