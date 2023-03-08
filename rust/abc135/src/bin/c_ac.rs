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
// use std::collections::{BinaryHeap, VecDeque};
use library::min;

/**
 * City Savers
 *
 * https://atcoder.jp/contests/abc135/tasks/abc135_c
 *
 * tags: #貪欲法
 *
 * 両端から取れる分だけ取っていくことが最適解となる.
 *
 * 右端をとるときは、左端をとる順と逆順でとることに注意.
 *
 *
 */
// #[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [usize;n+1],
        mut b: [usize; n]
    }
    let mut ans = 0;
    for i in 0..(n + 1) / 2 {
        let opp = n - 1 - i;

        let t = min!(a[i], b[i]);
        a[i] -= t;
        b[i] -= t;
        let s = min!(a[i + 1], b[i]);
        a[i + 1] -= s;
        b[i] -= s;

        ans += t + s;

        if i != opp {
            {
                let s = min!(a[opp + 1], b[opp]);
                a[opp + 1] -= s;
                b[opp] -= s;

                let t = min!(a[opp], b[opp]);
                a[opp] -= t;
                b[opp] -= t;

                ans += t + s;
            }
        }
    }
    println!("{}", ans);
}
