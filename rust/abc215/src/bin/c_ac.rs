use proconio::{fastout, input, marker::Chars};
use superslice::{self, Ext};
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
 * One More aab aba baa
 *
 * https://atcoder.jp/contests/abc215/tasks/abc215_c
 *
 * tags: #next_permutation #prev_permutation
 *
 * 自前で実装するにしても決められた文字列の中からk番目をO(1)で抜き出すのは難しい.
 *
 */

// #[fastout]
fn main() {
    input! {
        mut s: Chars,
        x: usize
    }
    s.sort_unstable();
    for _ in 0..x - 1 {
        s.next_permutation();
    }
    for c in s {
        print!("{}", c);
    }
    println!();
}
