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
 * Teleporter
 *
 * https://atcoder.jp/contests/abc167/tasks/abc167_d
 *
 * tags: #doubling #ダブリング #繰り返し二乗法
 */

// #[fastout]
fn main() {
    input! {
        n:usize,
        mut k:usize,
        mut a:[usize; n]
    }

    a = a.iter().map(|&x| x - 1).collect();

    let mut log_k = 1;
    while (1 << log_k) < k {
        log_k += 1;
    }

    let mut t = vec![vec![0; n]; log_k + 1];
    t[0][..].copy_from_slice(&a[..]);

    for i in 0..log_k {
        for j in 0..n {
            // i.e. k=1 で、2 回移動したことになるから、最初の位置 O から、1 回移動した時の位置 A、A から一回移動した時の位置 B が分かれば、
            // O から2回移動した時の位置が、B とわかる
            // 4 回移動した位置は、O -> B とわかっているため、B から 2 回移動した位置が分かれば、O -> ?? とわかる
            let prev_pos = t[i][j];
            t[i + 1][j] = t[i][prev_pos];
        }
    }

    let mut now = 0; // 1 から始める
    let mut i = 0;
    // i=0 の時、2^0 = 1、すなわち１回移動した時の位置
    // i=3 それまでに移動して now 時点から、 8 回移動した時の位置
    while k > 0 {
        if (k & 1) != 0 {
            now = t[i][now];
        }
        k >>= 1;
        i += 1;
    }

    println!("{}", now + 1); // 0-index にしてるため
}
