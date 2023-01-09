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
type Set = BTreeSet<(isize, isize)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};
use library::number::*;

/**
 * Teleportation
 *
 * https://atcoder.jp/contests/abc226/tasks/abc226_d
 *
 * x,yどちらかが0 になる時は、他方を1 したステップを採用
 * x,yどちらも0 にならない時は、gcd をとって最小のステップ幅を採用
 *
 */

fn main() {
    input! {
        n: usize,
        xy: [(isize, isize); n]
    }

    let mut ans = 0;
    let mut s = Set::new();
    for i in 0..n {
        for j in i + 1..n {
            let (x1, y1) = xy[i];
            let (x2, y2) = xy[j];
            let dx = x2 - x1;
            let dy = y2 - y1;
            if dx == 0 {
                if !s.contains(&(1, 0)) {
                    ans += 2;
                }
                s.insert((1, 0));
                s.insert((-1, 0));
            } else if dy == 0 {
                if !s.contains(&(0, 1)) {
                    ans += 2;
                }
                s.insert((0, 1));
                s.insert((0, -1));
            } else {
                let g = gcd(vec![dx, dy]) as isize;
                if !s.contains(&(dx / g, dy / g)) {
                    ans += 2;
                }
                s.insert((dx / g, dy / g));
                s.insert((-(dx / g), -(dy / g)));
            }
        }
    }

    println!("{}", ans);
}
