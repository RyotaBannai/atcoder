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
 * D - Strange Balls
 *
 * https://atcoder.jp/contests/abc240/submissions/37476094
 *
 *
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let mut v = VecDeque::new();
    let mut count = 0usize;

    for x in a {
        if let Some((k, c)) = v.pop_back() {
            // k:一番上に入っている数 c:何回連続してるか
            if k == x && c + 1 == k {
                count -= c;
            } else if k == x {
                v.push_back((k, c + 1));
                count += 1;
            } else {
                // もし違う数値なら
                v.push_back((k, c)); // そのまま戻す
                v.push_back((x, 1)); // 新しく追加
                count += 1;
            }
        } else {
            // 初回以外にも、筒の中のボールが全て消える可能性もあって、その場合はqueue が空
            v.push_back((x, 1));
            count += 1;
        }
        println!("{}", count);
    }
}
