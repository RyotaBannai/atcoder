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
type Map = BTreeMap<usize, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * Not Divisible
 *
 * https://atcoder.jp/contests/abc170/tasks/abc170_d
 *
 * tags: #約数 #math #調和級数
 *
 * Σ_i{1~M} M/i = O(MlogM) で計算できる
 * もし ai <=10^6 が全て素数だった場合、素数の数は 78498 だけど、
 * 素数のマスしか埋めなくて良いから同様に MlogM で処理できる.
 *
 * 2 2 2 2 2 2 2 ...
 * のような小さい整数だけで構成される数列では、全てのO(MN) になってしまうから、
 * ２回目に現れる際に、初めの１つをチェックした時点でbreak する
 *
 *
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let ma = 2 * 1_000_000;
    let mut map = Map::new();
    for &x in a.iter() {
        for i in 1.. {
            if i * x > ma {
                break;
            }
            *map.entry(i * x).or_insert(0) += 1;
            if let Some(&c) = map.get(&x) {
                if i == 1 && c >= 2 {
                    // 同じ数字が複数ある
                    break;
                }
            }
        }
    }

    let mut ans = 0;
    for &x in a.iter() {
        if let Some(&c) = map.get(&x) {
            if c == 1 {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
