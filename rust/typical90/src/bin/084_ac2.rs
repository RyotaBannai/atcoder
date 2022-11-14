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

/**
 * https://atcoder.jp/contests/typical90/tasks/typical90_cf
 *
 * 類題：
 * - AOJ ALDS 1D 「Maximum Profit」
 * - ABC125-C 「GCD on Blackboard」
 * - ABC182-D 「Wandering」
 * - 典型90 006
 *
 * 考察：
 * 唯一満たさなければいけない条件は、o と x を一つずつ持っている状態であるため、
 * o であれば、最後に x が存在する位置、
 * x であれば、最後に o が存在する位置、
 * さえ分かればその位置の一つ前まで削除することができる. ゆえにこれが１つの o/x を加えるときに増えるパターン数となる
 *
 * 参考
 * https://twitter.com/e869120/status/1412179495868534784/photo/3
 */

#[fastout]
fn main() {
    input! {
        _: usize,
        s: Chars
    }
    let mut o = 0;
    let mut x = 0;
    let mut ans = 0;
    for (i, &c) in s.iter().enumerate() {
        if c == 'o' {
            o = i + 1;
            ans += x; // 片方の一つ前まで削ることができる
        } else {
            x = i + 1;
            ans += o;
        }
    }

    println!("{}", ans);
}
