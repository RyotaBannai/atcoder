/**
 * @cpg_dirspec convex_hull
 *
 * cpg run -p src/bin/geometry/convex_hull.rs
 */
use library::{
    geometry::{polygon::convex_hull::*, vector::prelude::Vector},
    utils::read::*,
};
use std::cmp::Ordering::Less;

/**
 * 多角形-点の包含
 *
 * https://onlinejudge.u-aizu.ac.jp/courses/library/4/CGL/4/CGL_4_A
 *
 * tags: #convex_hull #凸包
 *
 */

fn main() {
    let n = read::<usize>()[0];
    let mut p = vec![];
    for _ in 0..n {
        let a = read::<f64>();
        p.push(Vector::new(a[0], a[1]));
    }

    let (mut up, mut low) = convex_hull(p);
    let mut vs = up;
    let wl = low.len();
    vs.append(&mut low[1..wl - 1].to_vec());

    let mut s = vs[0];
    let mut k = 0;
    for (i, &v) in vs.iter().enumerate() {
        if v.cmp_y(s) == Less {
            s = v;
            k = i;
        }
    }

    /*
    vs が 開始地点から最後までの上半分の点と、終了地点から開始地点までの下半分（開始地点と終了地点の重複分は省く）
    で構成されている.
    最小の y の中で最小の x となる頂点から初めて、反時計回りに出力したいため
    vs に vs を加えて、開始地点 s + vs.len() 地点から -1 した位置を順に出力
    */
    println!("{}", vs.len());

    k += vs.len();
    let mut other = vs.clone();
    vs.append(&mut other);

    loop {
        println!("{} {}", vs[k].x, vs[k].y);
        k -= 1;
        if vs[k] == s {
            break;
        }
    }
}
