use proconio::{fastout, input, marker::Chars};
// use std::cmp::{min, max};
use ac_library_rs::modint::ModInt1000000007 as Mint;
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
 * 050 - Stair Jump（★3）
 *
 * https://atcoder.jp/contests/typical90/tasks/typical90_ax
 *
 *
 * 考察：
 * - dp でやってみる. 1 を n回使った場合と、L を k回(k: 1<=k<=m, ただし、m*L<=n) を'分けて'考える -> 112, 211 を区別できない
 * - 再帰でやってみる. 現在の残り rest に対して、rest-1, rest-L を再帰的に見て、rest==0 の時点で解答に１加える -> 効率悪くて TLE
 * - 初めの dp のやり方で、11 を考慮するときに 2 も同時に見てあげれば、211 も考慮できる -> AC (本コード)
 */
#[fastout]
fn main() {
    input! {
        n: usize,
        l: usize
    }

    let mut dp = vec![Mint::new(0); n + 1];
    dp[0] += 1;

    for x in 1..=n {
        let y = dp[x - 1]; // 1 ステップ
        dp[x] += y;
        if x >= l {
            let z = dp[x - l]; // L ステップ
            dp[x] += z;
        }
    }

    println!("{}", dp[n]);
}
