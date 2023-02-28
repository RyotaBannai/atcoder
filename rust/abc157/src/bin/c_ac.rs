use itertools::Itertools;
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
use std::collections::{BTreeMap, BTreeSet};
type Map = BTreeMap<usize, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * Guess The Number
 *
 * https://atcoder.jp/contests/abc157/tasks/abc157_c
 *
 * tags: #string #文字列 #桁 #数字
 *
 */
// #[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        st: [(usize, usize); m]
    }
    let mut m = Map::new();
    for (s, t) in st {
        if s == 1 && t == 0 && n != 1 {
            println!("-1");
            return;
        }
        if let Some(&x) = m.get(&s) {
            if x != t {
                println!("-1");
                return;
            }
        } else {
            m.insert(s, t);
        }
    }
    let mut num = vec![0; n];
    if n != 1 {
        num[0] = 1;
    }
    for (k, v) in m.into_iter() {
        num[k - 1] = v;
    }
    for x in num.into_iter().take(n) {
        print!("{}", x);
    }
}
