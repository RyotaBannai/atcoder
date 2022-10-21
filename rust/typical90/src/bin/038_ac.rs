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

use typical90::nt_lib::*;

/**
 * 038 - Large LCM（★3）
 *
 * https://atcoder.jp/contests/typical90/tasks/typical90_al
 *
 * tags: #overflow #オーバフロー #lcm #最小公倍数
 *
 */

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize
    }
    let l = 1_000_000_000_000_000_000;
    let g = gcd(vec![a, b]);
    let c = a / g;
    // a*b の結果がoverflow にならないよう, a/g で l を割った結果と b を比較する
    if b <= l / c {
        println!("{}", c * b); // ? a * b / g この出力だとうまく行かない. <- lcm は l より小さいが、それでも a*b は overflow する可能性が考えられる
    } else {
        println!("Large");
    }
}
