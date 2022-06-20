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
 * D - 射撃王
 *
 * https://atcoder.jp/contests/abc023/tasks/abc023_d
 *
 * tags: #二分探索 #最大値の最小化
 *
 * 参考:
 * https://drken1215.hatenablog.com/entry/2021/06/12/113100
 *
 */

#[fastout]
fn main() {
    input! {
        n: usize,
        v: [(usize,usize); n]
    }

    let check = |x: usize| -> bool {
        let mut ok = true;
        let mut t = vec![0; n];
        for i in 0..n {
            let (h, s) = v[i];
            if h > x {
                ok = false; // 初期段階で間に合わない
            } else {
                t[i] = (x - h) / s; // 差し迫っている具合（デッドラインに達するまでの時間）を計算
            }
        }

        t.sort_unstable();

        for i in 0..n {
            // 差し迫っている風船から確認
            if t[i] < i {
                ok = false; // 時間切れ
            }
        }
        ok
    };

    let mut left = 0;
    let mut right = std::usize::MAX;

    while right - left > 1 {
        let mid = (right + left) / 2;
        if check(mid) {
            right = mid;
        } else {
            left = mid;
        }
    }

    println!("{}", right);
}
