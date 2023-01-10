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
type Set = BTreeSet<(usize, usize)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * D - Restricted Permutation
 *
 * https://atcoder.jp/contests/abc223/editorial
 *
 *
 * Wrong Answer
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        mut ab: [(usize, usize); m]
    }

    ab.sort_unstable();
    let mut op = vec![0; n + 1]; // 1 左、2 右
    let mut count = 0; // 削除も容易にできる順位を決定するid決めをしたい.
    let mut pos = vec![-1; n + 1]; //

    for (a, b) in ab {
        if op[b] == 1 {
            println!("-1");
            return;
        } else if op[b] == 2 {
            if op[a] == 0 {
                op[a] = 1;
                pos[a] = count;
                count += 1;
            }
            // 元々左側の制約として入っていたものを更に左へ移動する
            pos[b] = count;
            count += 1;
        } else {
            if op[a] == 0 {
                op[a] = 1;
                pos[a] = count;
                count += 1;
            }

            op[b] = 2;
            pos[b] = count;
            count += 1;
        }
    }
    let mut s = Set::new();
    for (i, cnt) in pos.iter().skip(1).enumerate() {
        s.insert((*cnt as usize, i + 1));
    }
    for (_, i) in s.iter() {
        print!("{} ", i);
    }
    println!();
}
