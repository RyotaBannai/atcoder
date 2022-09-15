/**
 * @cpg_dirspec intersection_circle_segment_2
 *
 * cargo run --bin intersection_circle_segment_2
 *
 * output
 * 0.95747864 6.56378204 6.42713674 -1.64070512
 */
use collection::geo_lib::*;

/**
 * 円と直線の交点
 */

fn main() {
    let c = Circle::new(Vector::new(3., 2.), 5.);
    let mut pt =
        CircleFns::points_at_intersection_line_from_le(c, LinearEquation::new(3., 2., -16.));
    pt.sort();
    let (v1, v2) = (pt[0], pt[1]);
    println!("{:.8} {:.8} {:.8} {:.8}", v1.x, v1.y, v2.x, v2.y);
}
