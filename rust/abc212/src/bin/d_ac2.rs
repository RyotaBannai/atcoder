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
// type Set = BTreeSet<isize>;
// use easy_ext::ext;
use library::query::seg_tree::*;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * Querying Multiset
 *
 * https://atcoder.jp/contests/abc212/tasks/abc212_d
 *
 * tags: #seg_tree #セグ木 #raq_ruq
 *
*/

// #[fastout]
fn main() {
    input! {
        n: usize
    }

    let mut hist = vec![];
    let mut seg = LazySegTree::new(
        n,
        std::isize::MAX, // 配列の初期値
        std::isize::MAX, // query で完全に探索外だった時の無効値
        0,
        0,
        |a: isize, b: isize| a.min(b),                      // 区間最小
        |a: isize, b: isize, _: usize| a.saturating_add(b), // 初期値の無効値が最大の時は saturaring する
        |_: isize, b: isize, _: usize| b,
        |a: isize, b: isize| a + b,
        |_: isize, b: isize| b,
        |a: isize, x: isize| a > x,
    );

    let mut index = 0;

    for _ in 0..n {
        input! { m: usize }
        if m == 1 {
            // 追加
            input! { x: isize }
            seg.update(index, index + 1, x);
            // 追加クエリが入るごとにindex を加算して有効値を葉に追加していく.
            // query 数<=10^5 だから初めからその分確保. min で無効値になったpos はそのまま.
            index += 1;
        } else if m == 2 {
            // 区間更新 RAQ
            input! { x: isize }
            seg.add(0, n + 1, x);
        } else {
            // 取り出す操作は、区間最小 RMQ をしてから、
            // その要素のpos を無効値でリセットすることに等しい.
            let mi = seg.query(0, n + 1);
            hist.push(mi);
            let pos = seg.find_rightest(0, n + 1, mi) as usize;
            seg.update(pos, pos + 1, std::isize::MAX);
        }
    }
    for x in hist {
        println!("{}", x);
    }
}
