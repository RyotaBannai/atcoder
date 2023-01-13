use itertools::Itertools;
use proconio::{fastout, input, marker::Chars};
// use std::cmp::{
//     max, min,
//     Ordering::{Equal, Greater, Less},
// };
// use ac_library_rs::modint::ModInt998244353 as Mint;
use superslice::{self, Ext};
// use derive_new::new;
// #[derive(new)]
// use indexmap::indexmap;
use std::{
    collections::{BTreeSet, HashMap, HashSet},
    iter::FromIterator,
};
type Map = HashMap<(isize, isize), bool>;
type Set = HashSet<Vec<(isize, isize)>>;
use std::option::Option;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

use library::structure::compress::*;

/**
 * D - Rectangles
 *
 * https://atcoder.jp/contests/abc218/tasks/abc218_d
 *
 * 座圧 + binary search ver.
 *
 * tags: #binary_search #compress #二分探索 #座圧
 *
 */
// #[fastout]
fn main() {
    input! {
        n: usize,
        mut xy: [(isize ,isize); n]
    }
    let comp = compress2d_point(xy, false, false);
    let mut nxy = comp.compressed;
    nxy.sort_unstable();
    let mut m = Map::new();
    for &k in &nxy {
        m.insert(k, true);
    }
    let mut s = Set::new();
    for i in 0..n {
        for j in i + 1..n {
            // 対角に位置する点２点を取り出す、と想定
            let a = nxy[i];
            let b = nxy[j];
            // 点a,b から作られる他の２点が存在すれば、x,y軸にそれぞれ並行な四角形が作れる.
            let c = (a.0, b.1); // ax,by
            let d = (b.0, a.1); // ay,bx
            if nxy[..].binary_search(&c).is_ok() && nxy[..].binary_search(&d).is_ok() {
                let z = BTreeSet::from_iter(vec![a, b, c, d]);
                if z.len() == 4 {
                    s.insert(z.into_iter().collect_vec());
                }
            }
        }
    }
    println!("{}", s.len());
}
