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
use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
type Set = BTreeSet<(isize, usize)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * C - Swiss-System Tournament
 *
 * https://atcoder.jp/contests/abc222/tasks/abc222_c
 *
 * BinaryHeap は扱いにくいから使わない.
 * set のソート順を変えたい時は負値をうまく利用する.
 */

fn calc(f: char, s: char) -> isize {
    // 勝ちなら1 負け、引き分けなら0
    if f == 'G' && s == 'C' || f == 'C' && s == 'P' || f == 'P' && s == 'G' {
        return 1;
    }
    0
}

// #[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        xs: [Chars; 2*n]
    }
    let mut new = Set::new();
    for i in 0..2 * n {
        new.insert((0, i));
    }

    let mut prev = Set::new();
    for j in 0..m {
        prev.clear();
        prev.clone_from(&new);
        new.clear();

        for ab in &prev.iter().chunks(2) {
            let (&(ca, ka), &(cb, kb)) = ab.collect_tuple().unwrap();
            new.insert((ca - calc(xs[ka][j], xs[kb][j]), ka));
            new.insert((cb - calc(xs[kb][j], xs[ka][j]), kb));
        }
    }

    for &(_, i) in new.iter() {
        println!("{}", i + 1);
    }
}
