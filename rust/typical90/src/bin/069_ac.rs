use proconio::{fastout, input, marker::Chars};
// use std::cmp::{min, max};
use ac_library_rs::modint::ModInt1000000007 as Mint;
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
 * 069 - Colorful Blocks 2（★3
 *
 * https://atcoder.jp/contests/typical90/tasks/typical90_bq
 *
 * tags: #繰り返し二乗法
 *
 * 類題：
 * - ABC178-C 「Ubiquity」
 * - JSC2021-D 「Nowwhere P」
 * - ARC116-B 「Products of Min-Max」
 * - AGC017-A 「Biscuits」
 *
 */

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize
    }
    // エッジケース対応.
    if n == 1 {
        println!("{}", k);
        return;
    } else if (n == 2 || n == 3) && k < n {
        println!("0");
        return;
    } else if n >= 4 && k < 3 {
        // この条件がなくても通る..
        println!("0");
        return;
    }

    let mut ans = Mint::new(1usize);
    // まず初めの二つを加える
    ans *= k;
    ans *= k - 1;

    let mut m = Mint::new(k - 2);
    let mut a = n - 2;

    while a != 0 {
        if a & 1 == 1 {
            ans *= m;
        }
        m *= m; // 桁増える時、自身を二乗していく.
        a >>= 1;
    }
    println!("{}", ans);
}
