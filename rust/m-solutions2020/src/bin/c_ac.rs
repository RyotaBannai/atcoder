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
 * Marks
 *
 * https://atcoder.jp/contests/m-solutions2020/tasks/m_solutions2020_c
 *
 * a<=10^9 n<=2*10^5 だから掛け合わせると数値が大きくなる.BitUInt を使ってもいいけど、clone しないといけないから処理が重くなってTLEになる.
 *
 * 結局は比較するiとi+1 では異なる部分は両端の得点（i+1 で追加される k+i 番目の得点と、i+1 時に取り除かれる得点i+1-k 番目の得点）
 * でしかないから、それを順に比較するだけで良い.
 *
 */
// #[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n]
    }
    for i in 0..n - k {
        if a[i] < a[i + k] {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
