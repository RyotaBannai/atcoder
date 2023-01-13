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
use std::{
    collections::{BTreeSet, HashMap, HashSet},
    iter::FromIterator,
};
type Map = HashMap<(usize, usize), bool>;
type Set = HashSet<Vec<(usize, usize)>>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * D - Rectangles
 *
 * https://atcoder.jp/contests/abc218/tasks/abc218_d
 *
 * Set の Set は定数倍遅い
 *
 */
// #[fastout]
fn main() {
    input! {
        n: usize,
        mut xy: [(usize ,usize); n]
    }
    let mut m = Map::new();
    for &k in &xy {
        m.insert(k, true);
    }
    let mut s = Set::new();
    for i in 0..n {
        for j in i + 1..n {
            // 対角に位置する点２点を取り出す、と想定
            let a = xy[i];
            let b = xy[j];
            // 点a,b から作られる他の２点が存在すれば、x,y軸にそれぞれ並行な四角形が作れる.
            let c = (a.0, b.1); // ax,by
            let d = (b.0, a.1); // ay,bx
            if m.get(&c).is_some() && m.get(&d).is_some() {
                let z = BTreeSet::from_iter(vec![a, b, c, d]);
                if z.len() == 4 {
                    s.insert(z.into_iter().collect_vec());
                }
            }
        }
    }
    println!("{}", s.len());
}
