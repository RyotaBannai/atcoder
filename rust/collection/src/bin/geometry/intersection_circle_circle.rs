/**
 * @cpg_dirspec intersection_circle_circle
 *
 * cpg run -p src/bin/geometry/intersection_circle_circle.rs
 */
use collection::{geo_lib::*, utils::read};

/**
 * 円の交点
 *
 * https://onlinejudge.u-aizu.ac.jp/courses/library/4/CGL/7/CGL_7_E
 *
 */

fn main() {
    let a = read::<f64>();
    let b = read::<f64>();
    let (c1, c2) = (
        Circle::new(Vector::new(a[0], a[1]), a[2]),
        Circle::new(Vector::new(b[0], b[1]), b[2]),
    );
    let (v1, v2) = CircleFns::points_at_intersection_circles(c1, c2);
    let (a, b) = {
        if v1 < v2 {
            (v1, v2)
        } else {
            (v2, v1)
        }
    };
    println!("{:.8} {:.8} {:.8} {:.8}", a.x, a.y, b.x, b.y);
}
