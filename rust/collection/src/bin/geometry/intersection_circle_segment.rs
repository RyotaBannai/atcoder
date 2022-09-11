/**
 * @cpg_dirspec intersection_circle_segment
 *
 * cpg run -p src/bin/geometry/intersection_circle_segment.rs
 */
use collection::{geo_lib::*, utils::read};

/**
 * 円と直線の交点
 *
 * https://onlinejudge.u-aizu.ac.jp/courses/library/4/CGL/7/CGL_7_D
 *
 */

fn main() {
    let a = read::<f64>();
    let c = Circle::new(Vector::new(a[0], a[1]), a[2]);

    let q = read::<usize>()[0];
    for _ in 0..q {
        let a = read::<f64>();
        let (v1, v2) = CircleFns::points_at_intersection_line(
            c,
            Vector::new(a[0], a[1]),
            Vector::new(a[2], a[3]),
        );
        let (a, b) = {
            if v1 < v2 {
                (v1, v2)
            } else {
                (v2, v1)
            }
        };
        println!("{:.8} {:.8} {:.8} {:.8}", a.x, a.y, b.x, b.y);
    }
}
