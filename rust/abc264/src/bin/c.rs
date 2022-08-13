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
 * Matrix Reducing
 *
 * https://atcoder.jp/contests/abc264/tasks/abc264_c
 *
 *
 * 組み合わせの全パターンが欲しい時は、bit 全探索を使うとらく
 *
 */

#[fastout]
fn main() {
    input! {
        h1: usize,
        w1: usize,
        a:[[usize; w1]; h1],
        h2: usize,
        w2: usize,
        b:[[usize; w2]; h2]
    }

    // let mut q = VecDeque::<(Vec<usize>, Vec<usize>)>::new();
    // q.push_back((vec![], vec![]));

    // while !q.is_empty() {
    //     let (rs, cs) = q.pop_back().unwrap();

    //     let mut tmp = vec![];
    //     for i in 0..h1 {
    //         if !rs.contains(&i) {
    //             tmp.push(a[i].clone());
    //         }
    //     }

    //     //  後ろから削除
    //     for &c in cs.iter().rev() {
    //         for i in 0..tmp.len() {
    //             tmp[i].remove(c);
    //         }
    //     }

    //     if tmp.len() == h2 && tmp[0].len() == w2 {
    //         let mut flag = true;
    //         for i in 0..tmp.len() {
    //             for j in 0..tmp[0].len() {
    //                 flag &= tmp[i][j] == b[i][j];
    //             }
    //         }
    //         if flag {
    //             println!("Yes");
    //             return;
    //         }
    //     }

    //     q.push_back((rs.push(), cs))
    // }

    // println!("No");
}
