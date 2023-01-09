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
use library::number::*;

/**
 * D - Factorial and Multiple
 *
 * https://atcoder.jp/contests/abc280/tasks/abc280_d
 *
 * K が106 以上の素因数を持つとき
 * その素因数をp とおくと、 明らかに答えは p 以上で、K≤1012 より K/p<p なので、K/p∈{1,2,…,p−1} であるから答えはp となる。
 *
 * K が106 以上の素因数を持たないとき
 * K≤1012 より K は 104 以下の素因数を高々 200 個、104 より大きく 106 以下の素因数を高々 2 個しか持たないので (2×106)! が K の倍数となる。よって答えは高々 2×106 なので、以下の実装例のように昇順に調べていけばよい。
 *
 * ２つ目の 10^6 に '2'をかける必要があるあたりのお気持ちがわからない..
 */

// #[fastout]
fn main() {
    input! {
        mut k : isize
    }

    for i in 2..=2_000_000 {
        let g = gcd(vec![i, k]);
        k /= g; // 毎回共通を削るの面倒なので、gcd で一気に削るの賢い
        if k == 1 {
            println!("{}", i);
            return;
        }
    }

    println!("{}", k);
}
