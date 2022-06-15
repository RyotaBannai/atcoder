use proconio::{fastout, input, marker::Chars};
use std::cmp::{max, min};
// use ac_library_rs::modint::ModInt998244353 as Mint;
// use superslice::{self, Ext};
// use derive_new::new;
// use indexmap::indexmap;
// use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
* C - ±1 Operation 1
*
* https://atcoder.jp/contests/abc255/tasks/abc255_c
*
* 初項 A 、公差 D 、項数 N の等差数列 S に含まれる数を「良い数」と呼びます。
* 与えられる X に +- の操作を行うことで「良い数」にするために必要な最小の「操作」回数を求めよ.
*
* 注意
* ・数列の範囲内にない場合を考慮
*/

#[fastout]
fn main() {
    input! {
        x: isize,
        a: isize,
        d: isize,
        n: usize,
    }

    let rest = x - a;
    if d == 0 {
        println!("{}", rest.abs());
        return;
    }

    let m = a + ((n as isize - 1) * d as isize);
    // 数列の範囲内に x がある
    if a <= x && x <= m || m <= x && x <= a {
        let diff = rest % d;
        // MOD D をしたときに +d 側に寄せても「良い数」になるため、d-diff との最小をとる
        println!("{}", min(diff.abs(), (d - diff).abs()));
    }
    // 範囲内にない
    else {
        // 範囲外にあるときは、初項側に近いのか、末項側に近いのか 最小をとる
        // a>=0, m>=0 どっちも正の時       a-x, x-m
        // a<0, m>=0 範囲が 0 をまたがる時  a-x, x-m
        // m<0, a<0 どっちも負の時         a-x, x-m
        // m<0, a>=0 範囲が 0 をまたがる時  a-x, x-m
        // いずれの場合距離を計算する式は同じで、その絶対値を取ればいいとわかる(x-a でも可)
        println!("{}", min((a - x).abs(), (x - m).abs()))
    }
}
