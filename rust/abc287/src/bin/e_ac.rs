use itertools::Itertools;
use library::chmax;
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
 *
 * Karuta
 *
 * https://atcoder.jp/contests/abc287/tasks/abc287_e
 *
 *
 * tags: #sort #ソート
 *
 * ソートして、前後で比較すると良い.
 *
 *
 */
// #[fastout]
fn main() {
    input! {
        n: usize,
        xs: [Chars; n]
    }
    let s = xs
        .into_iter()
        .enumerate()
        .map(|(i, xs)| (xs, i))
        .sorted()
        .collect_vec();

    let mut ans = vec![0; n];
    for i in 0..n {
        let mut ret = 0;
        let (xs, a) = &s[i];
        if i != 0 {
            let mut j = 0;
            let (ys, b) = &s[i - 1];
            while xs.len() > j && ys.len() > j && xs[j] == ys[j] {
                j += 1;
            }
            chmax!(ret, j);
        }
        if i != n - 1 {
            let mut j = 0;
            let (ys, b) = &s[i + 1];
            while xs.len() > j && ys.len() > j && xs[j] == ys[j] {
                j += 1;
            }
            chmax!(ret, j);
        }
        ans[*a] = ret;
    }
    for x in ans {
        println!("{}", x);
    }
}
