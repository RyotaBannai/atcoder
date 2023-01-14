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
use library::{chmax, chmin};

/**
 * String Shifting
 *
 * https://atcoder.jp/contests/abc223/tasks/abc223_b
 *
 * 文字をshifting した結果得られるパターン数は n
 * 元の文字列を２倍にして、長さn ずつずらしながら全探索すると全パターン得られる.
 *
 */

// #[fastout]
fn main() {
    input! {
        s: Chars
    }
    let mut mi = vec!['z'; 505];
    let mut ma = vec!['a'];

    let mut ss = s.clone();
    ss.append(&mut s.clone());
    for i in 0..s.len() {
        chmin!(mi, ss[i..i + s.len()].to_vec());
        chmax!(ma, ss[i..i + s.len()].to_vec());
    }

    for c in mi {
        print!("{}", c);
    }
    println!();
    for c in ma {
        print!("{}", c);
    }
    println!();
}
