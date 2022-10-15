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
type Map = BTreeMap<usize, Set>;
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

    let mut row = Map::new();
    let mut col = Map::new();
    for (r, c) in walls {
        let kr = r as usize - 1;
        if let Some(s) = row.get_mut(&kr) {
            s.insert(c - 1);
        } else {
            let mut s = Set::new();
            s.insert(c - 1);
            row.insert(kr, s);
        }

        let kc = c as usize - 1;
        if let Some(s) = col.get_mut(&kc) {
            s.insert(r - 1);
        } else {
            let mut s = Set::new();
            s.insert(r - 1);
            col.insert(kc, s);
        }
    }

    // d の向きに l の最大値を高速に求めたい
    let mut r = rs as isize - 1;
    let mut c = cs as isize - 1;
    for (d, l) in qs {
        match d {
            'L' => {
                let mut defa = c - l;
                if let Some(s) = row.get(&(r as usize)) {
                    if let Some(&nc) = s.range(..c).last() {
                        if defa <= nc {
                            defa = nc + 1;
                        }
                    }
                }
                c = defa.max(0);
            }
            'R' => {
                let mut defa = c + l;
                if let Some(s) = row.get(&(r as usize)) {
                    if let Some(&nc) = s.range(c + 1..).next() {
                        if nc <= defa {
                            defa = nc - 1;
                        }
                    }
                }
                c = defa.min(w - 1);
            }
            'U' => {
                let mut defa = r - l;
                if let Some(s) = col.get(&(c as usize)) {
                    if let Some(&nr) = s.range(..r).last() {
                        if defa <= nr {
                            defa = nr + 1;
                        }
                    }
                }
                r = defa.max(0);
            }
            'D' => {
                let mut defa = r + l;
                if let Some(s) = col.get(&(c as usize)) {
                    if let Some(&nr) = s.range(r + 1..).next() {
                        if nr <= defa {
                            defa = nr - 1;
                        }
                    }
                }
                r = defa.min(h - 1);
            }
            _ => unreachable!(),
        }

        println!("{} {}", r + 1, c + 1);
    }
}
