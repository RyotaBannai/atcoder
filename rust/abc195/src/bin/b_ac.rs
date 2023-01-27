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
 * Many Oranges
 *
 * https://atcoder.jp/contests/abc195/tasks/abc195_b
 *
 *
 * 下限さえうまくいけば、何個でも使えるから条件を満たす.
 *
 * 下限で割ったあまりを、下限で割った時の商を下限に足した値が
 * 上限を越えなければいくらでも調整できる.
 *
 *
 * 300~333 の時
 * 998 -> 333 333 332
 * 999 -> 333 333 333
 * 1000 -> 333 333 333 1
 *
 */
// #[fastout]
fn main() {
    input! {
        a: f64,
        b: f64,
        mut w: f64
    }
    w *= 1000.; // kg
    let p = (w / a).floor();
    let r = w % a;
    if r / p > b - a {
        println!("UNSATISFIABLE");
        return;
    }

    print!("{} {}", (w / b).ceil(), (w / a).floor());
}
