/**
 * @cpg_dirspec area_of_two_circles
 *
 * cpg run -p src/bin/geometry/area_of_two_circles.rs
 */
use collection::{geometry::*, utils::read::*};

/**
 * ２つ円の共通部分の面積
 *
 * https://onlinejudge.u-aizu.ac.jp/courses/library/4/CGL/7/CGL_7_I
 *
 */

fn main() {
    let a = read::<f64>();
    let b = read::<f64>();
    let c1 = Circle::new(Vector::new(a[0], a[1]), a[2]);
    let c2 = Circle::new(Vector::new(b[0], b[1]), b[2]);
    let ans = CircleFns::area_of_two_cricles(c1, c2);

    println!("{:.20}", ans);
}
