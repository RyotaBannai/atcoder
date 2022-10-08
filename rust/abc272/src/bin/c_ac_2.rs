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
use std::collections::{BinaryHeap, VecDeque};

/**
 * C - Max Even
 *
 * https://atcoder.jp/contests/abc272/tasks/abc272_c
 *
 * 偶奇それぞれ最大２つ管理
 *
 * Better
 */

#[fastout]
fn main() {
    input! {
        n: usize,
        a:[usize; n]
    }
    let mut vs = vec![BinaryHeap::new(); 2]; // 大きいのが優先
    for x in a {
        vs[x % 2].push(x);
    }
    let mut ma = -1;
    for xs in vs.iter_mut() {
        if xs.len() > 1 {
            ma = ma.max((xs.pop().unwrap() + xs.pop().unwrap()) as isize)
        }
    }

    println!("{}", ma);
}
