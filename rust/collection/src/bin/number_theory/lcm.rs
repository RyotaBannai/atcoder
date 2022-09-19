/**
 * @cpg_dirspec lcm
 *
 * cpg run -p src/bin/number_theory/lcm.rs
 */
use collection::{geo_lib::*, utils::read};

/**
 * 最小公倍数
 *
 * https://onlinejudge.u-aizu.ac.jp/courses/library/6/NTL/1/NTL_1_C
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
