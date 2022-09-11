/**
 * @cpg_dirspec contain_point
 *
 * cpg run -p src/bin/geometry/contain_point.rs
 */
use collection::{geo_lib::*, utils::read};

/**
 * 多角形-点の包含
 *
 * https://onlinejudge.u-aizu.ac.jp/courses/library/4/CGL/3/CGL_3_C
 *
 * tags: #containments #包含
 *
 * 水平線 R の左側にある頂点に対しては、頂点の y でスワップした結果、placement 判定が時計回りになるため交差判定をパスできる
 * またスワップすることで、非凸（concave, 凹）な多角形でも正しく判定できる
 *
 */

fn main() {
    let n = read::<usize>()[0];
    let mut p = vec![];
    for _ in 0..n {
        let a = read::<f64>();
        p.push(Vector::new(a[0], a[1]));
    }

    let q = read::<usize>()[0];

    for _ in 0..q {
        let b = read::<f64>();
        let ans = PolygonFns::contain_point(p.clone(), Vector::new(b[0], b[1]));
        println!(
            "{}",
            match ans {
                1 => 2,
                3 => 0,
                5 => 1,
                _ => unreachable!(),
            }
        );
    }
}
