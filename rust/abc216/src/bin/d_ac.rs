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
// use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
use std::collections::{BinaryHeap, VecDeque};

/**
 * Pair of Balls
 *
 * https://atcoder.jp/contests/abc216/tasks/abc216_d
 *
 * tags: #heap
 *
 * heap で2 個になった組から優先的にランダムに取り出すと良い.
 * 未処理でかつ count==1の色を取り出したら組ができないと考えられる.
 *
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        m: usize
    }
    let mut ms = vec![vec![]; m + 1];
    for j in 1..=m {
        input! {
            k: usize,
            a: [usize; k]
        }
        ms[j] = a.into_iter().rev().collect_vec();
    }
    let mut used = vec![false; n + 1];
    let mut count = vec![vec![]; n + 1];
    let mut q = BinaryHeap::new();

    for j in 1..=m {
        if let Some(a) = ms[j].pop() {
            count[a].push(j);
            q.push((count[a].len(), a));
        }
    }

    while let Some((c, a)) = q.pop() {
        if used[a] {
            // すでにペアの他方で処理ずみならpass
            continue;
        }

        if c != 2 {
            // 各筒の一番上にあるボールがカウントされた状態で２にならない=取り出せない
            println!("No");
            return;
        }

        used[a] = true;

        let j1 = count[a][0];
        let j2 = count[a][1];
        if let Some(a1) = ms[j1].pop() {
            count[a1].push(j1);
            q.push((count[a1].len(), a1))
        }
        if let Some(a2) = ms[j2].pop() {
            count[a2].push(j2);
            q.push((count[a2].len(), a2))
        }
    }

    if q.is_empty() {
        println!("Yes");
    } else {
        println!("No");
    }
}
