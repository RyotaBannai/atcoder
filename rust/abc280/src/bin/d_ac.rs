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
use library::number::factorize;

/**
 * D - Factorial and Multiple
 *
 * https://atcoder.jp/contests/abc280/tasks/abc280_d
 *
 * tags: #素数判定 #二分探索
 */

// #[fastout]
fn main() {
    input! {
        k : usize
    }

    fn f(mut n: usize, p: usize) -> usize {
        if n == 0 {
            return 0;
        }
        n /= p;
        n + f(n, p)
    }

    let facts = factorize(k);
    let mut l = 0;
    let mut r = k;

    while r - l > 1 {
        let mid = (r + l) / 2;
        let mut ok = true;

        for (&p, &cnt) in facts.iter() {
            if f(mid, p) < cnt {
                ok = false;
                break;
            }
        }
        if ok {
            // 最小を求めたいから、r を更新.
            r = mid;
        } else {
            l = mid;
        }
    }

    println!("{}", r);
}
