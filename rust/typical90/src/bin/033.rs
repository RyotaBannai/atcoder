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
 * 033 - Not Too Bright（★2）
 *
 * https://atcoder.jp/contests/typical90/tasks/typical90_ag
 *
 * tags: #コーナーケース
 *
 */

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize
    }

    // 問題文「縦 2 × 横 2 の、4 つの LED を含む領域であって、..」 -> 縦1or横1 しかない場合は、条件を常に満たしてしまう(!)ため、1x8 なら 8LED が点灯可能である、と考えられる.
    if h < 2 || w < 2 {
        println!("{}", h * w);
        return;
    }
    println!("{}", ((h + 1) / 2) * ((w + 1) / 2));
}
