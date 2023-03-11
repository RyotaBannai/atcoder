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
 * Four Variables
 *
 * https://atcoder.jp/contests/abc292/tasks/abc292_c
 *
 * tags: #約数
 *
 * あまりが i になる時のi の約数を先に計算しておくとよい.
 *
 *
 */
use library::number::divisor;
// #[fastout]
fn main() {
    input! {
        n: usize
    }
    let mut d = vec![0; n];
    for i in 1..n {
        d[i] = divisor(i).len();
    }
    let mut ans = 0;
    for a in 1..n {
        for b in 1..n {
            if a * b >= n {
                // あまりが0 になってもだめ
                break;
            }
            let rest = n - a * b;
            ans += d[rest];
        }
    }
    println!("{}", ans);
}
