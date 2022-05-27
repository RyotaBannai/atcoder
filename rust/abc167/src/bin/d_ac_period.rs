use proconio::{fastout, input, marker::Chars};
// use std::cmp::{min, max};
// use ac_library_rs::modint::ModInt998244353 as Mint;
// use superslice::{self, Ext};
// use derive_new::new;
// #[derive(new)]
// use indexmap::indexmap;
use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
type Set = BTreeSet<usize>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * Teleporter
 *
 * https://atcoder.jp/contests/abc167/tasks/abc167_d
 *
 * tags: #鳩の巣の原理 #周期
 *
 * https://algo-logic.info/abc167d/#toc_id_4
 * https://manabitimes.jp/math/692
 *
 */

// #[fastout]
fn main() {
    input! {
        n:usize,
        mut k:usize,
        mut a:[usize; n]
    }

    a = a.iter().map(|&x| x - 1).collect();

    let mut used = Set::new();
    let mut v = vec![0];

    let mut now = 0; // 0-index

    // 最大 N 回移動すれば、同じ場所に戻る（鳩の巣の原理）
    while !used.contains(&now) {
        used.insert(now); // 通った
        v.push(a[now]); // 道順を記憶
        now = a[now];
    }

    let s = v.iter().position(|&r| r == now).unwrap();
    let r = v.len() - 1 - s; // contains で false になる時、１つ同じ位置が２つある

    // else: 周期分 r で削って、残りは周期が始まる s 位置から余りを移動するだけ
    println!("{}", if k < s { v[k] } else { v[(k - s) % r + s] } + 1); // 0-index にしてるため
}
