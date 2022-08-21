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
 * Belt Conveyor
 *
 * https://atcoder.jp/contests/abc265/tasks/abc265_c
 *
 */
#[fastout]
fn main() {
    input! {
        h: isize, // 1<=h
        w: isize, // 1<=w
        g: [Chars; h]
    }

    let mut used = vec![vec![false; w as usize]; h as usize]; // 0-index で考えたい

    let mut lp = false;
    let mut pos = (0isize, 0isize);
    loop {
        // すでに通った
        if used[pos.0 as usize][pos.1 as usize] {
            lp = true;
            break;
        }

        // マークする
        used[pos.0 as usize][pos.1 as usize] = true;

        let op = g[pos.0 as usize][pos.1 as usize];

        // println!("{},{}", pos.0, pos.1);
        let (di, dj) = match op {
            'U' => (-1, 0),
            'D' => (1, 0),
            'L' => (0, -1),
            'R' => (0, 1),
            _ => unreachable!(),
        };
        let (ni, nj) = (pos.0 + di, pos.1 + dj);
        if ni < 0 || h <= ni || nj < 0 || w <= nj {
            // マスからはみ出るため、操作が終了
            break;
        } else {
            // マスからはみ出ない == 次の操作 op がある
            pos = (ni, nj);
        }
    }

    if lp {
        println!("-1");
    } else {
        println!("{} {}", pos.0 + 1, pos.1 + 1);
    }
}
