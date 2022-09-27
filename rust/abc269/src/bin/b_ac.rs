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
// use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * B - Rectangle Detection
 *
 * https://atcoder.jp/contests/abc269/tasks/abc269_b
 *
 * 正方形になることは保証されている
 * '#' が一つでも現れる行・列を管理
 */

#[fastout]
fn main() {
    input! {
        t: [Chars;10]
    }

    let mut rs = vec![];
    let mut cs = vec![];

    for (i, s) in t.iter().enumerate() {
        let mut flag = false;
        for &c in s {
            flag |= c == '#';
        }

        if flag {
            rs.push(i);
        }
    }

    for i in 0..10 {
        let mut flag = false;
        for s in t.iter() {
            flag |= s[i] == '#';
        }

        if flag {
            cs.push(i);
        }
    }

    println!("{} {}", rs[0] + 1, rs[rs.len() - 1] + 1);
    println!("{} {}", cs[0] + 1, cs[cs.len() - 1] + 1);
}
