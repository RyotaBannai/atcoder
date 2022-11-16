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
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
// type Map = BTreeMap<isize, bool>;
type Set = HashSet<isize>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * D - Robot Arms 2
 *
 * https://atcoder.jp/contests/abc274/tasks/abc274_d
 *
 * tags: #dp
 */

#[fastout]
fn main() {
    input! {
        n: usize,
        x: isize,
        y: isize,
        a: [isize; n]
    }

    // 0 index に x, 1 index に y
    let mut v = vec![vec![]; 2];
    for (i, b) in a.iter().enumerate() {
        v[i % 2].push(b);
    }

    let mut mapx = vec![Set::new(); v[0].len() + 1];
    let mut mapy = vec![Set::new(); v[1].len() + 1];
    mapx[0].insert(0);
    mapy[0].insert(0);

    for (i, &x) in v[0].iter().enumerate() {
        for k in mapx[i].clone().iter() {
            let mut ns = vec![k + x];
            if i != 0 {
                ns.push(k - x);
            }
            for nk in ns {
                mapx[i + 1].insert(nk);
            }
        }
    }

    if !mapx[v[0].len()].contains(&x) {
        println!("No");
        return;
    }

    for (i, &y) in v[1].iter().enumerate() {
        for k in mapy[i].clone().iter() {
            for &nk in &[k + y, k - y] {
                mapy[i + 1].insert(nk);
            }
        }
    }

    if !mapy[v[1].len()].contains(&y) {
        println!("No");
        return;
    }

    println!("Yes");
}
