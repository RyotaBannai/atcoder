/**
 * @cpg_dirspec reflection
 *
 * cpg run -p src/bin/geometry/reflection.rs
 */
use collection::{geo_lib::*, utils::read};

/**
 * 反射
 *
 * https://onlinejudge.u-aizu.ac.jp/courses/library/4/CGL/1/CGL_1_B
 *
 * tags: #reflection
 *
 * 正射影で落とした点 x への p からのベクトル px を２倍したベクトルを p に加えると、
 * 線分p1,p2 を対称軸としたベクトル（reflection）が求まる
 */

fn main() {
    let a = read::<f64>();
    let (v1, v2) = (Vector::new(a[0], a[1]), Vector::new(a[2], a[3]));

    let n = read::<usize>()[0];
    for _ in 0..n {
        let a = read::<f64>();
        let v = Vector::new(a[0], a[1]);
        let ans = VectorFns::reflection(v, v1, v2);
        println!("{:.10} {:.10}", ans.x, ans.y);
    }
}
