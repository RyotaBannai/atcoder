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
use std::collections::{BTreeMap, BTreeSet, HashSet};
type Map = BTreeMap<(usize, usize), bool>;
type Set = HashSet<Vec<usize>>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * B - Triangle (Easier)
 *
 * https://atcoder.jp/contests/abc262/tasks/abc262_b
 *
 */

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        es: [(usize, usize); m]
    }

    let mut map = Map::new();
    for (a, b) in es {
        map.insert((a, b), true);
        map.insert((b, a), true);
    }

    // よくある dfs/bfs で隣接行列で回すやり方だと m=n(n-1)/2 だから 3 つの組み合わせを選ぶことを考えると n=10^2 だから m=10^12 になって TLE
    // 頂点N は小さいから、N の組み合わせ数を考えると 10^6 でAC できる
    let mut ans = Set::new();
    for a in 1..=100 {
        for b in a + 1..=100 {
            if map.get(&(a, b)).is_some() {
                for c in b + 1..=100 {
                    if map.get(&(b, c)).is_some() && map.get(&(c, a)).is_some() {
                        let mut v = vec![a, b, c];
                        v.sort_unstable();
                        ans.insert(v);
                    }
                }
            }
        }
    }

    println!("{:?}", ans.len())
}
