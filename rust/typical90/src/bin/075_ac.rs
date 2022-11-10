use proconio::{fastout, input, marker::Chars};
// use std::cmp::{min, max};
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
use typical90::nt_lib::*;

/**
 * 075 - Magic For Balls（★3）
 *
 * tags: #素因数分解
 *
 * 考察
 * 与えられた整数n を素因数分解したときに、ある二つの素数をかけて合成数にして置き換える（置き換えた合成数は次に他の素数又は合成数とかけて置き換える）、
 * ことを繰り返すと与えられた整数n になる. n={a,b,c,d,e} -> {ab,cd,e} -> {ab, cde} -> {abcde} これよりこれを逆に（問題通りに）考えたときに、
 * 全ての素因数にするのに３ステップ必要なことがわかる.
 * 1 回の操作で全て合成数であれば、^2 増えるから、3 回の操作は最大８個の素数ができる操作である.
 * 8 個の素数があるかどうかを知るために先に素因数分解をしたい.
 *
 * 類題：
 * - ABC149-C 「Next Prime」
 * - ABC180-C 「Cream Puff」
 * - ABC144-C 「Walk on Multiplication Table」
 * - ABC084-D 「2017-like Number」
 */

#[fastout]
fn main() {
    input! {
        n: usize
    }

    let facts = factorize(n);
    let num = facts.iter().map(|(_, v)| v).sum::<usize>();
    let mut ans = 0;
    let mut k = 1;
    while k < num {
        k <<= 1;
        ans += 1;
    }
    println!("{}", ans);
}
