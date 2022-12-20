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
type Set = BTreeSet<isize>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * C - Choose Elements
 *
 * https://atcoder.jp/contests/abc245/tasks/abc245_c
 *
 * i 番目以前の値を引き継かずに、一番新しいi 番目の２つの値だけ管理.(直近の状態だけわかれば良い)
 * それまでどのPath を辿ってきたかは無視できると、高速化できる.
 *
 *
 * i=0 番目をデフォルトとして、set に入れておいて、i=1~n において、
 * 一つ前の要素とk 以下になる要素だけを新しいset として管理するようにする.
 *
 * この時、新しいset のsize=0 であれば、No とする.
 *
 *
 * 別解
 * https://atcoder.jp/contests/abc245/tasks/abc245_c/editorial
 * AまたはB それぞれ軸に計算結果を関したDP として解く
 * それぞれの軸は AまたはB の i と i-1 と比較して条件を満たすかどうか最後までcheck
 *
 * もし i-1 ですでに失敗していたらi 以降の処理は false （デフォルト）のままにする.
 * dp[n] or ep[n] == true になっていればよい.
 *
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        k: isize,
        a: [isize; n],
        b: [isize; n],
    }

    let mut dp = vec![Set::new(); 2];
    dp[0].insert(a[0]);
    dp[0].insert(b[0]);
    let mut t = 0;
    for i in 1..n {
        let mut ns = Set::new();
        for &x in &[a[i], b[i]] {
            for &y in dp[t].iter() {
                if (x - y).abs() <= k {
                    ns.insert(x); // 新しい方だけ管理. 最大管理数2
                }
            }
        }
        if ns.is_empty() {
            println!("No");
            return;
        }
        dp[(t + 1) % 2] = ns;
        t = (t + 1) % 2;
    }

    println!("Yes");
}
