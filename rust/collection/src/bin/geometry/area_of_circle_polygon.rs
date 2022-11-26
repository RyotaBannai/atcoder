/**
 * @cpg_dirspec area_of_circle_polygon
 *
 * cpg run -p src/bin/geometry/area_of_circle_polygon.rs
 */
use collection::{geometry::*, utils::read::*};

/**
 * 円と多角形の共通部分
 *
 * https://onlinejudge.u-aizu.ac.jp/courses/library/4/CGL/7/CGL_7_H
 *
 */

fn main() {
    let a = read::<f64>();
    let (n, r) = (a[0] as usize, a[1]);
    let c = Circle::new(Vector::new(0., 0.), r);
    let mut p = vec![];
    for _ in 0..n {
        let b = read::<f64>();
        p.push(Vector::new(b[0], b[1]));
    }
    let ans = CircleFns::area_of_circle_polygon(c, p);

    println!("{:.20}", ans);
}
