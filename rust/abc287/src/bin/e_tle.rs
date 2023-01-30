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
type Map = BTreeMap<Vec<char>, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 *
 * Karuta
 *
 * https://atcoder.jp/contests/abc287/tasks/abc287_e
 *
 * 部分文字列を全部map で持っておく TLE
 *
 */
// #[fastout]
fn main() {
    input! {
        n: usize,
        xs: [Chars; n]
    }
    let mut m = Map::new();
    let mut subs = vec![vec![]; n];
    for i in 0..n {
        let mut s = vec![];
        for &c in xs[i].iter() {
            s.push(c);
            *m.entry(s.clone()).or_insert(0) += 1;
            subs[i].push(s.clone());
        }
    }
    for i in 0..n {
        let mut ok = false;
        for st in subs[i].iter().rev() {
            if let Some(&count) = m.get(st) {
                if count > 1 {
                    println!("{}", st.len());
                    ok = true;

                    break;
                }
            }
        }
        if !ok {
            println!("0"); // 一個も一致する文字列が他にない
        }
    }
}
