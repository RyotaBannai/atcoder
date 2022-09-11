/**
 * @cpg_dirspec diameter_convex
 *
 * cpg run -p src/bin/geometry/diameter_convex.rs
 */
use collection::{geo_lib::*, utils::read};

/**
 * 凸多角形の直径
 *
 * https://onlinejudge.u-aizu.ac.jp/courses/library/4/CGL/4/CGL_4_B
 *
 * tags: #diameter_convex #凸多角形の直径
 *
 * 多角形から直径を求める
 * 凹多角形でも大丈夫. 先に凸包を求めて、その頂点の中から直径になる２点を見つける
 *
 * 二つのベクトルの内積が反時計周りのとき、直径が小さくなるベクトルだとわかるから、再度、時計回りになるように初めの位置の index (pos1) を進める
 * 時計回りの時は、直径が広がる方向にベクトルが進んでいるから、後ろの位置の index (pos2) を進める
 *
 * 初めの位置を固定して、全頂点をチェックすることを全ての頂点に対して行うと O(n^2) で TLE
 * 最大値をとる尺取り法のような感覚で実装するとよい
 *
 * 一回、最大値をとる位置が (ma_pos1, ma_pos2) で決まると、
 * もう一周した時に同じ位置が最大値としてまた戻ってくるため、それを break の条件とする
 *
 */

fn main() {
    let n = read::<usize>()[0];
    let mut p = vec![];
    for _ in 0..n {
        let a = read::<f64>();
        p.push(Vector::new(a[0], a[1]));
    }

    let ans = PolygonFns::diameter(p);
    println!("{:.7}", ans);
}
