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
 * D - Union of Interval
 *
 * https://atcoder.jp/contests/abc256/tasks/abc256_d
 *
 * ・開始時間の昇順にソートする
 * ・n, n+1 の区間が重ならない場合、前の和集合を追加、start をリセットして、新しい和集合に移る
 * ・最後の開区間の際も追加
 */

#[fastout]
fn main() {
    input! {
        n: usize,
        mut ran: [(usize, usize); n]
    }

    if ran.len() == 1 {
        for (l, r) in ran {
            println!("{} {}", l, r);
        }
        return;
    }

    ran.sort_unstable();

    let mut s = vec![];
    let (mut start, mut end) = ran[0];
    for (i, &(l, r)) in ran.iter().skip(1).enumerate() {
        // dbg!(start, end);
        // dbg!(l, r);
        if end < l {
            s.push((start, end));
            start = l;
        }
        if i == ran.len() - 2 {
            s.push((start, end.max(r)));
        }

        end = end.max(r);
    }

    for (l, r) in s {
        println!("{} {}", l, r);
    }
}
