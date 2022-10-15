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
type Set = BTreeSet<isize>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * https://atcoder.jp/contests/abc273/tasks/abc273_d
 */

#[fastout]
fn main() {
    input! {
        h: isize,
        w: isize,
        rs: usize,
        cs: usize,
        n: usize,
        walls: [(isize,isize); n],
        q: usize,
        qs: [(char,isize); q]
    }

    let mut row = vec![Set::new(); h as usize];
    let mut col = vec![Set::new(); w as usize];
    for (x, y) in walls {
        row[y as usize - 1].insert(x - 1);
        col[x as usize - 1].insert(y - 1);
    }

    // d の向きに l の最大値を高速に求めたい
    let mut pos = (rs as isize - 1, cs as isize - 1);
    for (d, l) in qs {
        let (x, y) = pos;
        match d {
            'L' => {
                if let Some(&nx) = row[y as usize].range(..x).last() {
                    if nx > x - l {
                        pos = (nx, y);
                    } else {
                        pos = ((x - l).max(0), y);
                    }
                } else if x < l {
                    pos = (0, y);
                } else {
                    pos = (x - l, y);
                }
            }
            'R' => {
                if let Some(&nx) = row[y as usize].range(x..).next() {
                    if nx < x + l {
                        pos = (nx, y);
                    } else {
                        pos = ((x + l).min(w - 1), y);
                    }
                } else if x + l > w - 1 {
                    pos = (w - 1, y);
                } else {
                    pos = (x + l, y);
                }
            }
            'U' => {
                if let Some(&ny) = col[x as usize].range(..y).last() {
                    if ny > y - l {
                        pos = (x, ny);
                    } else {
                        pos = (x, (y - l).max(0));
                    }
                } else if y < l {
                    pos = (x, 0);
                } else {
                    pos = (x, y - l);
                }
            }
            'D' => {
                if let Some(&ny) = col[x as usize].range(..).next() {
                    if ny < y + l {
                        pos = (ny, y);
                    } else {
                        pos = (x, (y + l).min(h - 1));
                    }
                } else if y + l > h - 1 {
                    pos = (x, h - 1);
                } else {
                    pos = (x, y + l);
                }
            }
            _ => unreachable!(),
        }

        println!("{} {}", pos.0, pos.1);
    }
}
