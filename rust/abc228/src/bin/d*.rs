use proconio::input;
use std::collections::BTreeSet;

/**
 * Liner Probing
 *
 * https://atcoder.jp/contests/abc228/tasks/abc228_d
 *
 * AC
 *
 * SegTree/UnionFind
 * ・https://atcoder.jp/contests/abc228/submissions/27367620
 * ・https://atcoder.jp/contests/abc228/submissions/27368971
 * ・https://atcoder.jp/contests/abc228/submissions/31085677
 * ・https://atcoder.jp/contests/abc228/submissions/31069281
 *
 * Set
 * ・https://atcoder.jp/contests/abc228/submissions/30610991
 */

fn main() {
    let mo = 1 << 20;
    input! {
        n: usize,
        qs: [(usize, usize); n]
    };
    let mut v: Vec<isize> = vec![-1; mo];
    let mut used = (0..mo).collect::<BTreeSet<_>>();
    for (q, x) in qs {
        let h = x % mo;
        if q == 1 {
            let pos = if let Some(&k) = used.range(h..).next() {
                k
            } else {
                *used.range(..).next().unwrap()
            };

            used.remove(&pos);
            v[pos] = x as isize;
        } else if q == 2 {
            println!("{}", v[h]);
        } else {
            unimplemented!();
        }
    }
}
