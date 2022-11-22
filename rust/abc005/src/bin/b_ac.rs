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
 * B - おいしいたこ焼きの食べ方
 *
 * https://atcoder.jp/contests/abc005/tasks/abc005_2
 *
 * N 個のたこ焼きがそれぞれ何秒前にできたか、それぞれi 行目に入力される.(1<=i<=N)
 * 一番出来立てのたこ焼きが何秒前にできたか答えよ. (最小を出力せよ)
 *
 */
#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }
    println!("{}", a.iter().min().unwrap());
}
