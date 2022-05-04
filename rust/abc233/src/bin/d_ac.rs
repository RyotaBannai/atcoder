use proconio::{fastout, input};
use std::collections::BTreeMap;
/**
 * Count Interval
 *
 * https://atcoder.jp/contests/abc233/tasks/abc233_d
 *
 * cumsum の 「end の数値　- k」 -> start の数値から k 増えた、ということ
 * start の数値を loop ごとに map で管理し、end - k から前に遡っていくつ同じ数値になるかを効率的チェック
 * （end の前に常に start を全て管理している）
 */

type Map = BTreeMap<isize, usize>;
#[fastout]
fn main() {
    input! {
        n: usize,
        k: isize,
        a: [isize;n]
    }
    let mut cumsum = vec![0isize];
    for (i, x) in a.iter().enumerate() {
        cumsum.push(x + cumsum[i]);
    }

    let mut m = Map::new();
    let mut ret = 0;
    for r in 1..=n {
        let start = cumsum[r - 1];
        let end = cumsum[r] - k;

        if let Some(v) = m.get_mut(&start) {
            *v += 1;
        } else {
            m.insert(start, 1);
        }

        if let Some(&v) = m.get(&end) {
            ret += v;
        }
    }

    dbg!(m);

    println!("{}", ret);
}
