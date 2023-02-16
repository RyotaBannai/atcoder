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
// type Map = BTreeMap<usize, usize>;
// type Set = BTreeSet<usize>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * A x B + C
 *
 * https://atcoder.jp/contests/abc179/tasks/abc179_c
 *
 * > A×B<N を満たす正整数の組 (A,B) に対し、A×B+C=N となるような正整数 C はちょうど一つに定まります。
 * > したがって、そのような正整数の組 (A,B) の個数を求めればよいです。
 */

// #[fastout]
fn main() {
    input! {
        n: usize
    }
    let mut ans = 0;
    for a in 1..=n {
        for b in 1..=n {
            if a * b > n - 1 {
                break;
            }
            ans += 1;
        }
    }

    println!("{}", ans);
}
