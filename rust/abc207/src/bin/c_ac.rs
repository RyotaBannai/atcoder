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
// type Set = BTreeSet<usize>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};
/**
 * Many Segments
 *
 * https://atcoder.jp/contests/abc207/tasks/abc207_c
 *
 * tags: #開区間 #閉区間 #区間
 *
 */

// t=1 → [li​,ri​]
// t​=2 → [li​,ri​)
// t​=3 → (li​,ri​]
// t​=4 → (li​,ri​)
// b を基準にして、a が小さ過ぎるor 大きすぎるパターンをチェックする.
// (bの区間をもとに、aの区間を前後に配置したような範囲外のケースを考える.)
// ]( や )[ の重なりも不正.
fn has_union(ta: usize, tb: usize, a: (isize, isize), b: (isize, isize)) -> bool {
    if a.1 < b.0 {
        return false;
    }
    if a.1 == b.0 {
        if ta == 2 || ta == 4 {
            return false;
        }
        if tb == 3 || tb == 4 {
            return false;
        }
    }

    if b.1 < a.0 {
        return false;
    }
    if b.1 == a.0 {
        if tb == 2 || tb == 4 {
            return false;
        }
        if ta == 3 || ta == 4 {
            return false;
        }
    }

    true
}

// #[fastout]
fn main() {
    input! {
        n: usize,
        xs: [(usize, (isize,isize)); n]
    }

    let mut count = 0;
    for i in 0..n {
        for j in i + 1..n {
            let ((ta, a), (tb, b)) = (xs[i], xs[j]);
            if has_union(ta, tb, a, b) {
                count += 1;
            }
        }
    }
    println!("{}", count);
}
