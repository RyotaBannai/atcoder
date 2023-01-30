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

/**
 * Ringo's Favorite Numbers 2
 *
 * https://atcoder.jp/contests/abc200/tasks/abc200_c
 */
// #[fastout]
fn main() {
    input! {
        n: usize,
        xs: [usize; n]
    }
    let mut m = vec![0usize; 200];

    for x in xs {
        m[x % 200] += 1;
    }

    let mut ans = 0;
    for a in m {
        if a <= 1 {
            continue;
        }
        ans += a * (a - 1) / 2;
    }

    println!("{}", ans);
}
