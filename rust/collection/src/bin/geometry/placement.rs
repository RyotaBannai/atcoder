/**
 * @cpg_dirspec placement
 *
 * cpg run -p src/bin/geometry/placement.rs
 */
use collection::{geo_lib::*, utils::read};

/**
 * 反時計回り
 *
 * https://onlinejudge.u-aizu.ac.jp/courses/library/4/CGL/1/CGL_1_C
 *
 * tags: #counter_clockwise
 *
 */

fn main() {
    let a = read::<f64>();
    let (v1, v2) = (Vector::new(a[0], a[1]), Vector::new(a[2], a[3]));

    let n = read::<usize>()[0];
    for _ in 0..n {
        let b = read::<f64>();
        let v = Vector::new(b[0], b[1]);
        let ans = VectorFns::place(v, v1, v2);
        println!(
            "{}",
            match ans {
                1 => "COUNTER_CLOCKWISE",
                3 => "CLOCKWISE",
                5 => "ONLINE_BACK",
                7 | 11 | 17 | 19 | 23 => "ON_SEGMENT", // 同じ位置は線分上とみる
                13 => "ONLINE_FRONT",
                _ => unreachable!(),
            }
        );
    }
}
