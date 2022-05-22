use proconio::{fastout, input, marker::Chars};
// use std::cmp::{max, min};
// use ac_library_rs::modint::ModInt998244353 as Mint;
use superslice::{self, Ext};
// use derive_new::new;
// #[derive(new)]
// use indexmap::indexmap;
// use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<usize, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * Snuke Festival
 *
 * https://atcoder.jp/contests/arc084/tasks/arc084_a
 *
 * tags: #中部全探索
 *
 * ・同じ数値は別物と考える（index が違う）
 * ・とりうる最大値が 10^9 くらいの場合は、二分探索を使う
*/

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
        b: [usize; n],
        mut c: [usize; n],
    }

    a.sort_unstable();
    c.sort_unstable();

    // 中部（B）は、上部（A）より大きく、下部（C）より小さい
    let mut count = 0;
    for x in b {
        // lower_bound は一つ大きい index を返す. 計算自体は 1-index だからそのまま使う
        // upper_bound は x を含まない、より大きい数を返す. 0-index だから n から -1 しない
        count += a.lower_bound(&x) * (n - c.upper_bound(&x));
    }

    println!("{}", count);
}
