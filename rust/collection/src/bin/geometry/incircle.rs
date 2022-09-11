/**
 * @cpg_dirspec incircle
 *
 * cpg run -p src/bin/geometry/incircle.rs
 */
use collection::{geo_lib::*, utils::read};

/**
 * 内接円
 */

fn main() {
    let mut p = vec![];
    for _ in 0..3 {
        let a = read::<f64>();
        p.push(Vector::new(a[0], a[1]));
    }

    let c = CircleFns::incircle(p[0], p[1], p[2]);
    println!("{} {} {}", c.c.x, c.c.y, c.r);
}
