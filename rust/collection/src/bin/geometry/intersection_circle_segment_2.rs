/**
 * @cpg_dirspec intersection_circle_segment_2
 *
 * cargo run --bin intersection_circle_segment_2
 */
use collection::geo_lib::*;

/**
 * 円と直線の交点
 */

fn main() {
    let c = Circle::new(Vector::new(3., 2.), 5.);
    let (v1, v2) =
        CircleFns::points_at_intersection_line_from_le(c, LinearEquation::new(3., 2., -16.));
    println!("{:.8} {:.8} {:.8} {:.8}", v1.x, v1.y, v2.x, v2.y);
}
