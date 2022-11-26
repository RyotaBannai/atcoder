/**
 * @cpg_dirspec tangent_to_a_circle
 *
 * cpg run -p src/bin/geometry/tangent_to_a_circle.rs
 */
use collection::{geometry::*, utils::read::*};

/**
 * 円の接線
 *
 * https://onlinejudge.u-aizu.ac.jp/courses/library/4/CGL/7/CGL_7_F
 *
 * 参考
 * https://qiita.com/tydesign/items/5ce4bd2cc58902606895
 */

fn main() {
    let a = read::<f64>();
    let b = read::<f64>();
    let v = Vector::new(a[0], a[1]);
    let c = Circle::new(Vector::new(b[0], b[1]), b[2]);
    let mut pt = CircleFns::tangent_point_from_polar(c, v);
    pt.sort();
    let (v1, v2) = (pt[0], pt[1]);
    println!("{:.10} {:.10}", v1.x, v1.y);
    println!("{:.10} {:.10}", v2.x, v2.y);
}
