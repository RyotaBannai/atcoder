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
 * Not Equal
 *
 * https://atcoder.jp/contests/abc209/tasks/abc209_c
 *
 * DP に見せかけた組合せの数え上げ
 * ソートして小さい方から順に１つずつ選んでいきつつ、選んだ数値は後段の処理では引いていく.
 *
 * B の並び順で固定されているから、選んで出来上がった数列の並び順は考慮しなくて良い.
 */
// #[fastout]
fn main() {
    input! {
        n: usize,
        mut cs: [usize; n]
    }
    let mo = 1_000_000_007;
    cs.sort_unstable();
    let mut ans = 1;
    for (i, c) in cs.iter().enumerate() {
        ans *= c - i;
        ans %= mo;
    }
    println!("{}", ans);
}
