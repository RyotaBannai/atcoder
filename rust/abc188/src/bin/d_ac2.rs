use itertools::Itertools;
use proconio::{fastout, input, marker::Chars};
use superslice::Ext;
// use std::cmp::{
//     max, min,
//     Ordering::{Equal, Greater, Less},
// };
// use ac_library_rs::modint::ModInt998244353 as Mint;
// use superslice::{self, Ext};
// use derive_new::new;
// #[derive(new)]
// use indexmap::indexmap;
use std::collections::{BTreeMap, BTreeSet};
type Map = BTreeMap<usize, isize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * Snuke Prime
 *
 * https://atcoder.jp/contests/abc188/tasks/abc188_d
 *
 * tags: #event #イベント
 *
 * イベントの発生するタイミングだけを記録するやり方.
 *
 * 日付の間隔では、i をみているときは次のi+1 までみたいから、n-1 回まで処理する.
 * n-1 回目は終了日だから、この処理は正しい.
 *
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        c: isize,
        abc: [(usize, usize, isize); n]
    }

    let mut m = Map::new();
    for (a, b, c) in abc {
        *m.entry(a).or_insert(0) += c;
        *m.entry(b + 1).or_insert(0) -= c;
    }

    let mut sum = 0;
    let mut ans = 0;
    let v = m.into_iter().sorted().collect_vec();
    for i in 0..v.len() - 1 {
        let (d, p) = v[i];
        let (nd, _) = v[i + 1];
        sum += p;
        if sum > c {
            ans += (nd - d) * c as usize;
        } else {
            ans += (nd - d) * sum as usize;
        }
    }
    println!("{}", ans);
}
