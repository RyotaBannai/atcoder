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
type Set = BTreeSet<usize>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * D - Do use hexagon grid
 *
 * https://atcoder.jp/contests/abc269/tasks/abc269_d
 *
 * tags: #union_find
 *
 */
use library::structure::disjoint_set::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        es: [(isize,isize); n]
    }

    let neibours = vec![(-1, -1), (-1, 0), (0, -1), (0, 1), (1, 0), (1, 1)];

    let range = 4006; // １行分 max(N)=1000, |xi|=1000, |yi|=1000 だから、正に 2000 分確保したい. 下限は 1 だから、1000 分くらい
    let mut ds = DisjointSet::new(range * range);
    let mut used = vec![false; range * range];
    let mut used_list = vec![];

    let shift = (range / 2) as isize;
    let calc = |x: usize, y: usize| y * shift as usize + x;
    let mut es_shift = vec![];
    for (a, b) in es {
        es_shift.push(((a + shift) as usize, (b + shift) as usize));
    }

    for (a, b) in es_shift {
        let pos = calc(a, b);
        // println!("-----");
        // println!("{:?} -> pos {}", (a, b), pos);
        used[pos] = true;
        used_list.push(pos);
        for (dx, dy) in &neibours {
            let (x, y) = (a as isize + dx, b as isize + dy);
            let nei_pos = calc(x as usize, y as usize);
            // println!("{:?} -> nei_pos {}", (x, y), nei_pos);
            if used[nei_pos] && !ds.same(pos, nei_pos) {
                // すでに色が塗られrているなら、つなげる
                ds.unite(pos, nei_pos);
            }
        }
    }

    let mut s = Set::new();
    // 異なる親（異なる組み）を探す
    for pos in used_list {
        s.insert(ds.find(pos));
    }

    println!("{}", s.len());
}
