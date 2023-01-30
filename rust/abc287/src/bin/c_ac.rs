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
use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
type Set = BTreeSet<usize>;
// use easy_ext::ext;
use std::collections::{BinaryHeap, VecDeque};

/**
 * Path Graph?
 *
 * https://atcoder.jp/contests/abc287/tasks/abc287_c
 *
 * tags: #star #スター #グラフ
 *
 * 連結かどうか、巡回してるかどうかチェック
 *
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(usize, usize); m]
    }

    let mut deg = vec![0; n + 1]; // 入次数を管理
    let mut list = vec![Set::new(); n + 1];
    for (u, v) in uv {
        list[u].insert(v);
        list[v].insert(u);
        deg[u] += 1;
        deg[v] += 1;

        // グラフがスターになっている時とかはパスグラフになり得ないから入次数でチェック
        if deg[u] > 2 || deg[v] > 2 {
            println!("No");
            return;
        }
    }

    let mut q = VecDeque::new();
    let mut used = vec![false; n + 1];
    used[0] = true;

    q.push_back((1, 0)); // 連結だから1 から初めて良い // 端から始めなくても良い.

    // p: 親
    while let Some((u, p)) = q.pop_back() {
        if used[u] {
            continue;
        }
        used[u] = true;

        for &v in list[u].iter() {
            if v != p && used[v] {
                // 巡回してる == 数列にできない
                println!("No");
                return;
            }
            deg[v] -= 1;
            if deg[v] <= 1 {
                q.push_back((v, u));
            }
        }
    }

    // 非連結ならだめ
    for x in used {
        if !x {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
