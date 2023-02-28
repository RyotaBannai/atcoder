use proconio::{fastout, input, marker::Chars};
// use std::cmp::{
//     max, min,
//     Ordering::{Equal, Greater, Less},
// };
// use ac_library_rs::modint::ModInt998244353 as usize;
// use superslice::{self, Ext};
// use derive_new::new;
// #[derive(new)]
// use indexmap::indexmap;
use std::collections::{BTreeMap, BTreeSet};
type Map = BTreeMap<usize, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * Banned K
 *
 * https://atcoder.jp/contests/abc159/tasks/abc159_d
 *
 * tags: #組み合わせ
 *
 */
// #[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }
    let mut m = Map::new();
    for &x in &a {
        *m.entry(x).or_insert(0) += 1;
    }
    let mut sum = 0;
    for (_, &v) in m.iter() {
        sum += v * (v - 1) / 2;
    }

    for x in &a {
        if let Some(&count) = m.get(x) {
            let mut ans = sum;

            if count >= 2 {
                ans -= count * (count - 1) / 2;
                ans += (count - 1) * (count - 2) / 2;
                println!("{}", ans)
            } else {
                println!("{}", ans);
            }
        }
    }
}
