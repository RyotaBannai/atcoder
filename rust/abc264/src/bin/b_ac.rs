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
 * Nice Grid
 *
 * https://atcoder.jp/contests/abc264/tasks/abc264_b
 *
 * チェビシェフ距離（チェス盤距離）
 * max{∣R−8∣,∣C−8∣}
 *
 * を使うといい
 */

#[fastout]
fn main() {
    input! {
        r: usize,
        c: usize
    }

    // 縦に見る
    // c 1, 15 は全部黒
    // c 2, 14 は r 1, 15 以外白
    // c 3, 13 は r 2, 14 以外黒
    // c 4, 12 は r 1, 15, 3, 13 以外白
    // c 5, 11 は r 2, 14, 4, 12 以外黒
    // c 6, 10 は r 1, 15, 3, 13, 5, 11 以外白
    // c 7, 9  は r 2, 14, 4, 12, 6,10 以外黒
    // c 8     は 白

    if c == 1 || c == 15 {
        println!("black");
        return;
    }
    // c 2, 14 は r 1, 15 以外白
    if c == 2 || c == 14 {
        if r == 1 || r == 15 {
            println!("black");
        } else {
            println!("white");
        }
        return;
    }

    // c 3, 13 は r 2, 14 以外黒
    if c == 3 || c == 13 {
        if r == 2 || r == 14 {
            println!("white");
        } else {
            println!("black");
        }
        return;
    }

    // c 4, 12 は r 1, 15, 3, 13 以外白
    if c == 4 || c == 12 {
        if r == 1 || r == 15 || r == 3 || r == 13 {
            println!("black");
        } else {
            println!("white");
        }
        return;
    }

    // c 5, 11 は r 2, 14, 4, 12 以外黒
    if c == 5 || c == 11 {
        if r == 2 || r == 14 || r == 4 || r == 12 {
            println!("white");
        } else {
            println!("black");
        }
        return;
    }

    // c 6, 10 は r 1, 15, 3, 13, 5, 11 以外白
    if c == 6 || c == 10 {
        if r == 1 || r == 15 || r == 3 || r == 13 || r == 5 || r == 11 {
            println!("black");
        } else {
            println!("white");
        }
        return;
    }

    // c 7, 9  は r 2, 14, 4, 12, 6,10 以外黒
    if c == 7 || c == 9 {
        if r == 2 || r == 14 || r == 4 || r == 12 || r == 6 || r == 10 {
            println!("white");
        } else {
            println!("black");
        }
        return;
    }

    if c == 8 && r == 8 {
        println!("white");
    }
    // c 8     は 白
}
