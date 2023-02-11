use proconio::{fastout, input, marker::Chars};
// use std::cmp::{
// max, min,
// Ordering::{Equal, Greater, Less},
// };
// use superslice::Ext;
// use ac_library_rs::modint::ModInt998244353 as Mint;
// use superslice::{self, Ext};
// use derive_new::new;
// use indexmap::indexmap;
// use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * Smartphone Addiction
 *
 * https://atcoder.jp/contests/abc185/tasks/abc185_b
 *
 * .5 は処理は1 の時に減ると考えて良い.
 */
use library::min;
fn main() {
    input! {
        n: isize, // バッテリー容量
        m: usize,
        t: isize, // 帰宅時刻
        ab: [(isize, isize); m]
    }

    let mut rest = n;
    let mut prev = 0;
    // カフェi をでた後の容量をチェック
    for (a, b) in ab {
        rest -= a - prev;
        if rest <= 0 {
            println!("No");
            return;
        }
        rest = min!(n, rest + b - a);
        prev = b;
    }
    rest -= t - prev;
    if rest <= 0 {
        println!("No");
    } else {
        println!("Yes");
    }
}
