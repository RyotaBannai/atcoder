/**
 * @cpg_dirspec tangent_to_a_circle
 *
 * cpg run -p src/bin/geometry/tangent_to_a_circle.rs
 */
use collection::{geo_lib::*, utils::read};

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
    let (mut t1, mut t2) = CircleFns::points_at_intersection_line_from_polar(c, v);
    if t1 > t2 {
        std::mem::swap(&mut t1, &mut t2);
    }
    println!("{:.10} {:.10}", t1.x, t1.y);
    println!("{:.10} {:.10}", t2.x, t2.y);
}
