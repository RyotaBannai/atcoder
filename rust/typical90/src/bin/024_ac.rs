use proconio::{fastout, input, marker::Chars};
// use std::cmp::{min, max};
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
 * 024 - Select +／- One（★2）
 *
 * https://atcoder.jp/contests/typical90/tasks/typical90_x
 *
 *
 * Bad：
 * ・全探索->実行時間がかかりすぎ
 * ・aray a と array b を先に総和を求めて、差分をみる->各index ごとに調整したいから、総和の差分が 0 でも要素同士は異なることが起こる
 * (i.g. a: [1,2], b: [2,1])
 *
 * Good：
 * ・要素毎に差分を計算する
 * ・差分と k が偶数or 奇数どうしかつ、操作回数が差分以上なら、余りは偶数で-1+1 すれば戻るからok
 *
*/

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [isize; n],
        b: [isize; n]
    }

    let sub = a
        .iter()
        .zip(b.iter())
        .map(|(x, y)| (x - y).abs())
        .sum::<isize>() as usize;

    let cond1 = sub % 2 == 0 && k % 2 == 0 && k >= sub; // sub=4, k=2 は無理、sub=2, k=4 なら２つ調整して、残り２回フリップするだけ
    let cond2 = sub % 2 == 1 && k % 2 == 1 && k >= sub; // sub=3, k=5 は3つ調整して、残り２回フリップするだけ
    println!("{}", if cond1 || cond2 { "Yes" } else { "No" });
}
