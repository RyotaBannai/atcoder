use proconio::{fastout, input};
use std::collections::BTreeSet;
type Set = BTreeSet<usize>;

/**
 * At Most 3 (Judge ver.)
 * https://atcoder.jp/contests/abc251/tasks/abc251_b
 *
 * ・組み合わせの結果同じ数値ならカウントしない
*/

#[fastout]
fn main() {
    input! {
        n: usize,
        w: usize,
        a: [usize; n]
    }

    let mut s = Set::new();
    for &x in &a {
        if x <= w {
            s.insert(x);
        }
    }

    for i in 0..n {
        for j in (i + 1)..n {
            if a[i] + a[j] <= w {
                s.insert(a[i] + a[j]);
            }
        }
    }

    for i in 0..n {
        for j in (i + 1)..n {
            for k in (j + 1)..n {
                if a[i] + a[j] + a[k] <= w {
                    s.insert(a[i] + a[j] + a[k]);
                }
            }
        }
    }

    println!("{}", s.len());
}
