use proconio::{fastout, input, marker::Chars};
// use std::cmp::{
//     max, min,
//     Ordering::{Equal, Greater, Less},
// };
// use ac_library_rs::modint::ModInt998244353 as Mint;
use superslice::Ext;
// use superslice::{self, Ext};
// use derive_new::new;
// #[derive(new)]
// use indexmap::indexmap;
use std::collections::{BTreeMap, BTreeSet};
type Map = BTreeMap<usize, Vec<usize>>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};
/**
 * D - Range Count Query
 *
 * https://atcoder.jp/contests/abc248/tasks/abc248_d
 *
 * tags: #二分探索
 *
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
        m: usize,
        qs: [(usize, usize, usize); m]
    }

    let mut m = Map::new();

    for (i, &x) in a.iter().enumerate() {
        m.entry(x).or_insert(vec![]).push(i + 1);
    }

    for (l, r, x) in qs {
        if let Some(v) = m.get(&x) {
            let pos_l = v.lower_bound(&l);
            // index が実際に要素を指していてもupper を見ることで+1 する必要がない.（他には全要素より大きく末尾を見る場合）
            let pos_r = v.upper_bound(&r);

            // 同じ位置なら１カウントできてok
            println!("{}", pos_r - pos_l);
        } else {
            // ます整数が存在していない
            println!("0");
        }
    }
}
