use proconio::{fastout, input, marker::Chars};
// use std::cmp::{min, max};
// use ac_library_rs::modint::ModInt998244353 as Mint;
// use superslice::{self, Ext};
// use derive_new::new;
// #[derive(new)]
// use indexmap::indexmap;
// use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
// type Set = BTreeSet<usize>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * Distinct Trio
 *
 * https://atcoder.jp/contests/abc252/tasks/abc252_d
 *
 * tags: #置き換え #中部全探索 #Snuke Festival
 *
 * ・index が i < j < k -(1)で Ai != Aj != Ak -(2)である組み合わせは一意に定まる！
 *   (2) のケースは複数考えられるが、(1)の index が異なるため、別の組み合わせ
 *   問題では A の大小の並びは気にしていない
 * ・ Ai < Aj < Ak として、i != j != k のケースと読み替えることができる
 * 　i != j != k すなわち 組(i,j,k) は、(1)を並び替えた結果であり、これもまた一意に定まる！
 * 　ケース２を解いた結果はケース１の結果と一致
 */

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let mk = 200_005;
    let mut v = vec![0; mk];
    for x in &a {
        v[*x] += 1;
    }

    for i in 0..v.len() - 1 {
        v[i + 1] += v[i];
    }

    // 中部（B）は、上部（A）より大きく、下部（C）より小さい
    let mut count = 0;
    for x in a {
        count += v[x - 1] * (n - v[x]);
    }

    println!("{}", count);
}
