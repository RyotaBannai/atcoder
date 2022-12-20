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
use std::collections::{BinaryHeap, VecDeque};

/**
 * C - Coupon
 *
 * https://atcoder.jp/contests/abc246/tasks/abc246_c
 *
 * tags: #priority_quque
 *
 * ちょうど0 円になったら追加しない!
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        mut k: usize,
        x: usize,
        a: [usize; n]
    }

    let mut pq = BinaryHeap::new();
    for b in a {
        pq.push(b);
    }

    while k > 0 {
        if let Some(a) = pq.pop() {
            let mut times = a / x;
            if times == 0 {
                // 残りがx より小さい時は0 にする
                times += 1;
            }
            if times < k {
                k -= times;
                let rest = a.saturating_sub(times * x);
                // ちょうど0 円になったら追加しない!
                if rest != 0 {
                    pq.push(rest);
                }
            } else {
                let rest = a.saturating_sub(k * x);
                pq.push(rest);
                // チケットが無くなった.
                break;
            }
        } else {
            break;
        }
    }

    let mut ans = 0;
    while let Some(a) = pq.pop() {
        ans += a;
    }

    println!("{}", ans);
}
