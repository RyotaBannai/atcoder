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
 * C - Submask
 *
 * https://atcoder.jp/contests/abc269/tasks/abc269_c
 *
 * tags: #部分集合 #subset #bit全探索
 *
 * １が n の部分集合となる整数は、１を使うか使わないかの２通りの全ての組み合わせであるから、１が３つ立っていれば８通り.
 * １が立っている位置の部分集合をbit全探索で求める
 *
 */

#[fastout]
fn main() {
    input! {
        n: usize
    }

    let mut pops = vec![];
    for i in 0..60 {
        if (n >> i) & 1 == 1 {
            pops.push(i);
        }
    }

    // bit 全探索
    for bit in 0..1 << pops.len() {
        let mut num = 0usize;
        for i in 0..pops.len() {
            if (bit >> i) & 1 == 1 {
                num |= 1 << pops[i];
            }
        }
        println!("{}", num);
    }
}
