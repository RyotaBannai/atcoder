use proconio::{fastout, input, marker::Chars};
// use std::cmp::{
//     max, min,
//     Ordering::{Equal, Greater, Less},
// };
// use ac_library_rs::modint::ModInt998244353 as Mint;
use superslice::{self, Ext};
// use derive_new::new;
// #[derive(new)]
// use indexmap::indexmap;
use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
// type Set = BTreeSet<(usize, usize)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * C - Min Max Pair
 *
 * https://atcoder.jp/contests/abc262/tasks/abc262_c
 *
 * tags: #二分探索
 */

// fn ord_tup<T: std::cmp::PartialOrd>(a: T, b: T) -> (T, T) {
//     if a > b {
//         (b, a)
//     } else {
//         (a, b)
//     }
// }

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }
    let mut pos = vec![];
    for (i, x) in a.iter().enumerate() {
        // 自分のindex と同じ数字だけ管理する. 管理するのはそのindex
        if i + 1 == *x {
            pos.push(i);
        }
    }

    // 1.自分のindex と同じ数字は、他のindex 上に同じ数字があるもの全てとペアになりうる (if)
    // 2.それ以外の index は交換する先が自分の index である場合 (else if)
    // どちらの場合も、自分より前のパターンはすでにカウント済みである、ということにだけ気をつける.
    // 1では、自分より後の位置全てを高速に求める必要がある
    let mut ans = 0;
    for (i, &x) in a.iter().enumerate() {
        if i + 1 == x {
            let p = pos.upper_bound(&i);
            if p == pos.len() {
                continue;
            } else {
                ans += pos.len() - p;
            }
        } else if a[x - 1] == i + 1 && a[x - 1] > x {
            // 自分(x) より小さい組み合わせはすでに確認済み
            ans += 1;
        }
    }

    println!("{}", ans);
}
