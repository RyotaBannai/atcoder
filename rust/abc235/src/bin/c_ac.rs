use proconio::{fastout, input};
use std::collections::BTreeMap;
/**
 * The Kth Time Query
 *
 * https://atcoder.jp/contests/abc235/tasks/abc235_c
 *
 */
type Map = BTreeMap<usize, Vec<usize>>;
#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize;n],
        qs: [(usize,usize); q]
    }

    let mut m = Map::new();

    for (i, x) in a.iter().enumerate() {
        if let Some(v) = m.get_mut(x) {
            v.push(i + 1);
        } else {
            m.insert(*x, vec![i + 1]);
        }
    }
    for (x, k) in qs {
        if let Some(v) = m.get(&x) {
            if v.len() >= k {
                println!("{}", v[k - 1]);
                continue;
            }
        }
        println!("-1");
    }
}
