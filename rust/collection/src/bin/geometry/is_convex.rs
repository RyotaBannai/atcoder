/**
 * @cpg_dirspec is_convex
 *
 * cpg run -p src/bin/geometry/is_convex.rs
 */
use collection::{geometry::*, utils::read::*};

/**
 * 凸性判定
 *
 * https://onlinejudge.u-aizu.ac.jp/courses/library/4/CGL/3/CGL_3_B
 *  
 * 注意
 * 全ての内角の大きさが180度以下であるような多角形を凸多角形とします。（全てが一直線上にある場合も凸多角形とみなす）
 * => cross の判定に 0 を含める
 *
 */

fn main() {
    let n = read::<usize>()[0];
    let mut vs = vec![];
    for _ in 0..n {
        let a = read::<f64>();
        vs.push(Vector::new(a[0], a[1]));
    }
    let mut ans = true;
    for i in 0..n {
        let j = (i + 1) % n; // next vector
        let k = (i + 2) % n; // next next vector
        let place = VectorFns::place(vs[i], vs[j], vs[k]);
        ans &= place != 3;
    }

    println!("{}", if ans { "1" } else { "0" });
}
