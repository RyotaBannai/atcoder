/**
 * @cpg_dirspec projection
 *
 * cpg run -p src/bin/geometry/projection.rs
 */
use library::{
    geometry::vector::{other::*, prelude::*},
    utils::read::*,
};

/**
 * 射影
 *
 * https://onlinejudge.u-aizu.ac.jp/courses/library/4/CGL/1/CGL_1_A
 *
 * tags: #projection
 *
 * 線分 p1,p2 からできる線分に対して、点 p から下ろした正射影の交点 x（点）のを求めよ
 *
 * 線分 p1,x の線分の大きさを t, 線分 p1,p の大きさを hypo, 線分 p1,p2 の大きさを base とすると
 * t=|hypo|cosθ, hypo・base = |hypo||base|cosθ
 * より、t の式にできるから
 * t=hypo・base/|base|
 * （t はあくまで x における p1 からの大きさである）
 * t と |base| の比率r = t/|base|
 * これより、x は
 * x = p1 + base*r = p1 + base*(t/base) = base*(hypo・base/|base|^2)
 *
 */

fn main() {
    let a = read::<f64>();
    let (v1, v2) = (Vector::new(a[0], a[1]), Vector::new(a[2], a[3]));

    let n = read::<usize>()[0];
    for _ in 0..n {
        let a = read::<f64>();
        let v = Vector::new(a[0], a[1]);
        let ans = projection(v, v1, v2);

        // 0.00000001 以下の誤差（小数点8桁+1）だから、9 桁まで出力
        println!("{:.10} {:.10}", ans.x, ans.y);
    }
}
