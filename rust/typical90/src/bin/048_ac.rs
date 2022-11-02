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
 * 048 - I will not drop out（★3）
 *
 * https://atcoder.jp/contests/typical90/tasks/typical90_av
 *
 * 類題：
 * - ABC141 -D 「Powerful Discount Tickets」
 * - ARC115 -C 「N Coloring」
 * - ARC119 -B 「Electric Board」
 * - JOI 2017 本番２ 「準急電車」
 */

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        p: [(usize, usize); n]
    }

    let mut v = vec![];
    for (a, b) in p {
        v.push(b);
        v.push(a - b);
    }
    v.sort_unstable();
    let mut ans = 0;

    let mut m = k;
    for x in v.iter().rev() {
        if m == 0 {
            break;
        }
        m -= 1;
        ans += x;
    }

    println!("{}", ans);
}
