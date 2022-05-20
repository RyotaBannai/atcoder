use proconio::{fastout, input, marker::Chars};
// use ac_library_rs::modint::ModInt998244353 as Mint;
// use superslice::{self, Ext};
// use derive_new::new;
// #[derive(new)]
// use indexmap::indexmap;
// use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;

/**
 * Moves on Binary Tree
 *
 * https://atcoder.jp/contests/abc243/tasks/abc243_d
 *
 * ・U の時は一つ前の操作を打ち消す
 * ・途中で 10^18 を超える場合があるため、binary string を使う
 * ・String の slice は遅い. Vec の末尾を操作するだけだと高速
 *
 * std::fmt::Binary
 * https://doc.rust-lang.org/std/fmt/trait.Binary.html
*/

#[fastout]
fn main() {
    input! {
        _: usize,
        mut x: usize,
        s: Chars
    }

    let mut bs = format!("{:b}", x).chars().collect::<Vec<char>>();

    for c in s {
        if c == 'L' {
            bs.push('0');
        } else if c == 'R' {
            bs.push('1');
        } else if c == 'U' && x != 1 {
            bs.pop();
        }
    }

    println!(
        "{}",
        usize::from_str_radix(&bs.iter().collect::<String>(), 2).unwrap()
    );
}
