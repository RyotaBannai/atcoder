use proconio::{fastout, input, marker::Chars};
// use std::cmp::{
//     max, min,
//     Ordering::{Equal, Greater, Less},
// };
// use ac_library_rs::modint::ModInt998244353 as Mint;
// use superslice::{self, Ext};
// use derive_new::new;
// #[derive(new)]
// use indexmap::indexmap;
// use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
use library::chmin;
use std::usize::MAX;

/**
 * B - Job Assignment
 *
 * https://atcoder.jp/contests/abc194/tasks/abc194_b
 *
 */
// #[fastout]
fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n]
    }

    let mut ans = MAX;
    let mut ax = vec![];
    let mut bx = vec![];
    for (i, (a, b)) in ab.into_iter().enumerate() {
        ax.push((a, i));
        bx.push((b, i));
        if ans > a + b {
            ans = a + b;
        }
    }
    ax.sort_unstable();
    bx.sort_unstable();
    for (a, i) in ax {
        let (b, j) = bx[0];
        if i == j {
            let (nb, _) = bx[1]; // ２個まで保証されてる
            chmin!(ans, a.max(nb));
        } else {
            chmin!(ans, a.max(b));
        }
    }

    println!("{}", ans);
}
