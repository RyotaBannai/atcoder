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
 * Anti-Division
 *
 * https://atcoder.jp/contests/abc131/tasks/abc131_c
 *
 * tags: #lcm #最小公倍数
 *
 * 重複する数を差し引いて余事象を求める
 *
 *
 */
use library::number::lcm;
// #[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
    }
    let mut ans = b - a + 1;

    let l = lcm(vec![c as isize, d as isize]) as usize;
    {
        // a より小さい c で割り切れる数
        let x = (a - 1) / c;
        // a より小さい d で割り切れる数
        let y = (a - 1) / d;
        // a より小さい c,dで割り切れる数
        let z = (a - 1) / l;

        ans += x + y - z;
    }

    {
        // b 以下の c で割り切れる数
        let x = b / c;
        // b 以下の d で割り切れる数
        let y = b / d;
        // b 以下の c,dで割り切れる数
        let z = b / l;

        ans -= x + y - z;
    }
    println!("{}", ans);
}
