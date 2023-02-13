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
type Map = BTreeMap<isize, usize>;
type Set = BTreeSet<isize>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * Coprime
 *
 * https://atcoder.jp/contests/abc177/tasks/abc177_e
 *
 *
 * tags: #素因数分解 #高速素因数分解
 *
 * 高速素因数分解
 * 前処理として、整数N の最小の素数を値に持った素数表を作っておく.
 * その素数表をもとに再帰的に素数を求めていき、index==value となったらbreakする.
 * １回の処理でN/2 となるから logN の素因数分解ができる.
 *
 */
use library::number::{gcd, prime};
// #[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [isize; n]
    }

    let (_, _, t) = prime(1000006);

    let mut ok = true;
    let mut m = Map::new();
    for &x in a.iter() {
        // 高速素因数分解
        let mut factors = Set::new();
        let mut a = x;
        loop {
            let f = t[a as usize] as isize;
            if f != 1 {
                factors.insert(f);
            }
            if f == a {
                break;
            }
            a /= f;
        }
        // ここまで
        for k in factors {
            if m.get(&k).is_some() {
                ok = false;
                break;
            }
            m.insert(k, 1);
        }
    }

    if ok {
        println!("pairwise coprime");
        return;
    }

    if gcd(a) == 1 {
        println!("setwise coprime");
        return;
    }
    println!("not coprime");
}
