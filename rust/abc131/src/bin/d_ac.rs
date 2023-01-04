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
 * D - Megalomania
 *
 * tags: #貪欲法
 *
 * https://atcoder.jp/contests/abc131/tasks/abc131_d
 *
 * 結局は〆切時刻が差し迫っている仕事から先に終わらせないといけないため、
 * その順にソートして、順に処理していったときに、
 * どこかのタイミングで仕事の完了時刻が累積時間に追い越されるタイミングがあれば
 * 全て完了できない、と考えることができる.
 *
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        mut ab: [(usize, usize); n]
    }

    // 〆切時刻でソート
    ab.sort_unstable_by(|x, y| x.1.cmp(&y.1));
    let mut acc = 0;
    for (a, b) in ab {
        if acc + a > b {
            println!("No");
            return;
        }
        acc += a;
    }
    println!("Yes");
}
