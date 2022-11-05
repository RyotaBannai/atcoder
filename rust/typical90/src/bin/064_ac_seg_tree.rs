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
use typical90::query_lib::*;

/**
 * 064 - Uplift（★3）
 *
 * https://atcoder.jp/contests/typical90/tasks/typical90_bl
 *
 * tags: #seg_tree #セグ木
 *
 */

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [isize; n],
        qs: [(usize,usize,isize); q],
    }

    let f = |a: isize, b: isize| a + b;
    let mut seg = LazySegTree::new(
        n,
        0,
        0,
        0,
        f,
        f,
        f,
        |a: isize, n: usize| a * n as isize,
        |a: isize, x: isize| a > x,
    );
    for (i, x) in a.iter().enumerate() {
        seg.set(i, *x);
    }
    seg.build();

    let mut sum = 0;
    for i in 0..n - 1 {
        sum += (a[i + 1] - a[i]).abs();
    }

    for (mut l, mut r, x) in qs {
        // 更新したい閉区間[l,r] を0index に直す.
        l -= 1;
        r -= 1;
        // もし両端であれば、地殻変動が起きても標高の差分に変化がないから処理しない
        // 地殻変動が起きて標高に差分ができるのは両端だけ.
        if l >= 1 {
            let l1 = seg.query(l - 1, l);
            let l2 = seg.query(l, l + 1);
            sum -= (l1 - l2).abs();
            sum += (l1 - (l2 + x)).abs();
        }
        if r + 1 < n {
            let r1 = seg.query(r, r + 1);
            let r2 = seg.query(r + 1, r + 2);
            sum -= (r1 - r2).abs();
            sum += ((r1 + x) - r2).abs();
        }

        seg.update(l, r + 1, x); // r の位置まで更新したい
        println!("{}", sum);
    }
}
