use proconio::{fastout, input};

/**
 * Neighbors
 *
 * https://atcoder.jp/contests/abc231/tasks/abc231_d
 *
 *
 * 1~N の数字を M 個の数値を繋げながら、横一直線に並べることができるか判定せよ
 *
 * ・一つの頂点から３以上連結していないこと
 * ・頂点が循環しないこと
 * をチェック
 */
use abc231::structure::disjoint_set::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        vers: [(usize, usize); m]
    }

    let mut ds = DisjointSet::new(n);
    let mut v = vec![0usize; n + 1];
    for (a, b) in vers {
        v[a] += 1;
        v[b] += 1;
        if v[a] == 3 || v[b] == 3 {
            println!("No");
            return;
        }
        if ds.same(a, b) {
            println!("No");
            return;
        } else {
            ds.unite(a, b);
        }
    }
    println!("Yes");
}
