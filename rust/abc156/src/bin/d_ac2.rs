use proconio::{fastout, input, marker::Chars};
// use std::cmp::{
//     max, min,
//     Ordering::{Equal, Greater, Less},
// };
use ac_library_rs::modint::ModInt1000000007 as mint;
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
 * Bouquet
 *
 * https://atcoder.jp/contests/abc156/tasks/abc156_d
 *
 * tags: #数え上げ #組み合わせ
 *
 * n個からk 選ぶ時の全ての組み合わせ数は 2^n でもとまる
 *
 * 一つも選ばない場合を除く
 *
 *
 */

fn mod_comb(n: mint, k: mint) -> mint {
    let a = mod_perm(n, k);
    let b = mod_perm(k, k);

    a * b.inv()
}

fn mod_perm(n: mint, k: mint) -> mint {
    let mut ret = mint::from(1);

    for i in 0..k.val() {
        ret *= n - i;
    }

    ret
}

// #[fastout]
fn main() {
    input! {
        n: mint,
        a: mint,
        b: mint
    }
    let ans = mint::from(2usize).pow(n.val() as u64) - mod_comb(n, a) - mod_comb(n, b) - 1;
    println!("{}", ans);
}
