use itertools::Itertools;
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
 * 宇宙人からのメッセージ
 *
 * https://atcoder.jp/contests/zone2021/tasks/zone2021_d
 *
 * 隣り合う文字が同じ場合に削除していく処理は queue を使う.
 *
 */

// #[fastout]
fn main() {
    input! {
        s: Chars
    }
    let mut rev = false;
    let mut t = VecDeque::new();
    for c in s {
        if c == 'R' {
            rev = !rev;
        } else if rev {
            t.push_front(c);
        } else {
            t.push_back(c);
        }
    }
    let mut v = t.into_iter().collect_vec();
    if !rev {
        v = v.into_iter().rev().collect_vec();
    }
    let mut q = VecDeque::new();
    while let Some(x) = v.pop() {
        if let Some(y) = q.pop_back() {
            if y == x {
                continue;
            }
            q.push_back(y); // 違うなら戻す
            q.push_back(x); // 新しく追加する
        } else {
            // 何も入ってなければ単に追加するだけ
            q.push_back(x);
        }
    }
    while let Some(c) = q.pop_front() {
        print!("{}", c);
    }
}
